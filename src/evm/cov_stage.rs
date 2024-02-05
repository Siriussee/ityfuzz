use std::{
    cell::RefCell,
    fs,
    ops::Deref,
    rc::Rc,
    path,
    time::Instant
};
use std::fs::{File, OpenOptions};
use std::io::{Seek, Write};

use itertools::Itertools;
use libafl::{
    corpus::Corpus,
    events::ProgressReporter,
    prelude::{CorpusId, HasMetadata, ObserversTuple, Stage},
    state::{HasCorpus, UsesState},
    Error,
    Evaluator,
};

use crate::{
    evm::{
        host::CALL_UNTIL,
        input::EVMInput,
        middlewares::{
            call_printer::{CallPrinter, SingleRound, RoundPrinterResult},
            coverage::{Coverage, EVAL_COVERAGE},
            middleware::MiddlewareType,
        },
        types::{EVMFuzzExecutor, EVMFuzzState, EVMQueueExecutor, EVMStagedVMState},
    },
    generic_vm::vm_executor::GenericVM,
    oracle::BugMetadata,
    state::HasInfantStateState,
};
use crate::state::{HasCurrentInputIdx, HasExecutionResult, HasPresets};
use tracing::info;

pub struct CoverageStage<OT> {
    pub last_corpus_idx: usize,
    pub last_fuzz_round: usize,
    pub last_execution_count: usize,
    pub start_time: Instant,
    round_info: RoundPrinterResult,
    round_info_file: fs::File,
    executor: Rc<RefCell<EVMQueueExecutor>>,
    coverage: Rc<RefCell<Coverage>>,
    call_printer: Rc<RefCell<CallPrinter>>,
    trace_dir: String,
    pub phantom: std::marker::PhantomData<OT>,
}

impl<OT> UsesState for CoverageStage<OT> {
    type State = EVMFuzzState;
}

impl<OT> CoverageStage<OT> {
    pub fn new(
        executor: Rc<RefCell<EVMQueueExecutor>>,
        coverage: Rc<RefCell<Coverage>>,
        call_printer: Rc<RefCell<CallPrinter>>,
        work_dir: String,
    ) -> Self {
        let trace_dir = format!("{}/traces", work_dir);
        if !std::path::Path::new(&trace_dir).exists() {
            std::fs::create_dir_all(&trace_dir).unwrap();
        }
        let mut round_info_file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(false)
            .open(format!("{}/round_info.json", work_dir)).unwrap();
        Self {
            last_corpus_idx: 0,
            last_fuzz_round: 0,
            last_execution_count: 0,
            start_time: Instant::now(),
            round_info: Default::default(),
            round_info_file: round_info_file,
            executor,
            coverage,
            call_printer,
            trace_dir,
            phantom: std::marker::PhantomData,
        }
    }

    fn get_call_seq(vm_state: &EVMStagedVMState, state: &mut EVMFuzzState) -> Vec<(EVMInput, u32)> {
        if let Some(from_idx) = vm_state.trace.from_idx {
            let corpus_item = state.get_infant_state_state().corpus().get(from_idx.into());
            // This happens when full_trace feature is not enabled, the corpus item may be
            // discarded
            if corpus_item.is_err() {
                return vec![];
            }
            let testcase = corpus_item.unwrap().clone().into_inner();
            let testcase_input = testcase.input();
            if testcase_input.is_none() {
                return vec![];
            }
            let prev_state = testcase_input.clone().unwrap();
            let prev = Self::get_call_seq(testcase_input.as_ref().unwrap(), state);

            return [
                prev,
                vm_state
                    .trace
                    .transactions
                    .iter()
                    .enumerate()
                    .map(|(idx, ci)| {
                        if idx == 0 {
                            ci.to_input(prev_state.clone())
                        } else {
                            ci.to_input(EVMStagedVMState::new_uninitialized())
                        }
                    })
                    .collect_vec(),
            ]
            .concat();
        }
        vec![]
    }

    fn get_call_seq_as_string(call_printer: &CallPrinter, vm_state: &mut EVMStagedVMState, state: &mut EVMFuzzState) -> Vec<String> {
        if let Some(from_idx) = vm_state.trace.from_idx {
            let corpus_item = state.get_infant_state_state().corpus().get(from_idx.into());
            // This happens when full_trace feature is not enabled, the corpus item may be
            // discarded
            if corpus_item.is_err() {
                return vec![];
            }
            let testcase = corpus_item.unwrap().clone().into_inner();
            let testcase_input = testcase.input();
            if testcase_input.is_none() {
                return vec![];
            }
            let prev_state = testcase_input.clone().unwrap();
            let prev = Self::get_call_seq_as_string(call_printer, &mut testcase_input.clone().unwrap(), state);

            return [
                prev,
                vm_state
                    .trace
                    .transactions
                    .iter()
                    .enumerate()
                    .map(|(idx, ci)| {
                        if idx == 0 {
                            call_printer.translate_address(ci.to_input(prev_state.clone()).0.contract)
                        } else {
                            call_printer.translate_address(ci.to_input(EVMStagedVMState::new_uninitialized()).0.contract)
                        }
                    })
                    .collect_vec(),
            ]
                .concat();
        }
        vec![]
    }
}

impl<EM, Z, OT> Stage<EVMFuzzExecutor<OT>, EM, Z> for CoverageStage<OT>
where
    Z: Evaluator<EVMFuzzExecutor<OT>, EM, State = Self::State>,
    EM: ProgressReporter + UsesState<State = Self::State>,
    OT: ObserversTuple<Self::State>,
{
    fn perform(
        &mut self,
        _fuzzer: &mut Z,
        _executor: &mut EVMFuzzExecutor<OT>,
        state: &mut Self::State,
        _manager: &mut EM,
        _corpus_idx: CorpusId,
    ) -> Result<(), Error> {
        // Advance fuzzing round
        let last_idx = state.corpus().last();
        let target_contract = self.call_printer.deref().borrow_mut().translate_address(
            state.corpus().get(CorpusId::from(state.get_current_input_idx())).unwrap().clone().into_inner().input().as_ref().unwrap().contract.clone()
        );
        let input_state = &mut state.corpus().get(CorpusId::from(state.get_current_input_idx())).unwrap().clone().into_inner().input().as_ref().unwrap().sstate.clone();
        // For the current fuzzing input (s,t), record how it is formed ==> txs forming s
        let previous_transactions = Self::get_call_seq_as_string(
            self.call_printer.deref().borrow_mut().deref(),
            input_state,
            state
        );

        let mut data = SingleRound {
            contract: target_contract,
            prev_contracts: previous_transactions,
            fuzzing_round: self.last_fuzz_round,
            sec_elapsed: self.start_time.elapsed().as_secs(),
            total_mutations: state.executions - self.last_execution_count,
            total_interesting: 0,
        };

        // info!("state: {}", serde_json::to_string(&input_state.trace).unwrap());
        // info!("input: {}", serde_json::to_string(&data).unwrap());

        self.last_execution_count = state.executions;
        self.last_fuzz_round += 1;

        if last_idx.is_none() {
            // clear the round info file and write latest round info to it
            self.round_info.data.push(data);
            let json = serde_json::to_string(&self.round_info).unwrap();
            self.round_info_file.set_len(0).unwrap();
            self.round_info_file.rewind().unwrap();
            self.round_info_file.write_all(json.as_bytes()).unwrap();
            return Ok(());
        }
        let last_idx = last_idx.unwrap().into();
        if self.last_corpus_idx == last_idx {
            // clear the round info file and write latest round info to it
            self.round_info.data.push(data);
            let json = serde_json::to_string(&self.round_info).unwrap();
            self.round_info_file.set_len(0).unwrap();
            self.round_info_file.rewind().unwrap();
            self.round_info_file.write_all(json.as_bytes()).unwrap();
            return Ok(());
        }

        let mut exec = self.executor.deref().borrow_mut();
        exec.host.add_middlewares(self.call_printer.clone());

        let meta = state.metadata_map().get::<BugMetadata>().unwrap().clone();
        let mut current_idx = CorpusId::from(self.last_corpus_idx);
        // For each new interesting corpus item (s'',t''), replay how it is formed
        // Comes two ways:
        //  1. By mutating t of current fuzzing input (s, t) => exec(s, t') => (s'', t'')
        //  2. By mutating s of current fuzzing input (s, t) => exec(s', t) => (s'', t'')
        while let Some(i) = state.corpus().next(current_idx) {
            self.call_printer.deref().borrow_mut().cleanup();
            data.total_interesting += 1;
            let testcase = state.corpus().get(i).unwrap().borrow().clone();
            let last_input = testcase.input().as_ref().expect("Input should be present");

            let mut last_state: EVMStagedVMState = Default::default();
            for (mut tx, call_until) in Self::get_call_seq(&last_input.sstate, state) {
                if tx.step {
                    self.call_printer.deref().borrow_mut().mark_step_tx();
                }
                unsafe {
                    CALL_UNTIL = call_until;
                }
                if !tx.sstate.initialized {
                    tx.sstate = last_state.clone();
                }
                let res = exec.execute(&tx, state);
                last_state = res.new_state.clone();
                self.call_printer
                    .deref()
                    .borrow_mut()
                    .mark_new_tx(last_state.state.post_execution.len());
            }
            unsafe {
                CALL_UNTIL = u32::MAX;
            }
            unsafe {
                EVAL_COVERAGE = true;
            }

            {
                if last_input.step {
                    self.call_printer.deref().borrow_mut().mark_step_tx();
                }
                exec.execute(last_input, state);
            }

            self.call_printer
                .deref()
                .borrow_mut()
                .save_trace(format!("{}/{}", self.trace_dir, i).as_str());
            if let Some(bug_idx) = meta.corpus_idx_to_bug.get(&i.into()) {
                for id in bug_idx {
                    fs::copy(
                        format!("{}/{}.json", self.trace_dir, i),
                        format!("{}/bug_{}.json", self.trace_dir, id),
                    )
                    .unwrap();
                }
            }
            unsafe {
                EVAL_COVERAGE = false;
            }

            current_idx = i;
        }

        exec.host.remove_middlewares_by_ty(&MiddlewareType::CallPrinter);

        // clear the round info file and write latest round info to it
        self.round_info.data.push(data);
        let json = serde_json::to_string(&self.round_info).unwrap();
        self.round_info_file.set_len(0).unwrap();
        self.round_info_file.rewind().unwrap();
        self.round_info_file.write_all(json.as_bytes()).unwrap();

        if self.last_corpus_idx == last_idx {
            return Ok(());
        }

        self.coverage.deref().borrow_mut().record_instruction_coverage();
        self.last_corpus_idx = last_idx;
        Ok(())
    }
}
