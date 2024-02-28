//! The power schedules. This stage should be invoked after the calibration
//! stage.

use core::{fmt::Debug, marker::PhantomData};

use libafl::{
    corpus::{Corpus, CorpusId},
    executors::{Executor, HasObservers},
    fuzzer::Evaluator,
    mutators::Mutator,
    prelude::Testcase,
    stages::{mutational::MutatedTransform, MutationalStage, Stage},
    state::{HasClientPerfMonitor, HasCorpus, HasMetadata, HasRand, UsesState},
    Error,
};
use libafl::inputs::Input;
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::generic_vm::vm_state::VMStateT;
use crate::input::{ConciseSerde, VMInputT};
use crate::state::{HasExecutionResult, HasFavored, HasInfantStateState};

pub trait TestcaseScoreWithId<S, VI, VS, Loc, Addr, Out, CI>
where
    S: HasCorpus + HasMetadata,
    VI: VMInputT<VS, Loc, Addr, CI> + Input,
    VS: Default + VMStateT,
    Addr: Serialize + DeserializeOwned + Debug + Clone,
    Loc: Serialize + DeserializeOwned + Debug + Clone,
    Out: Default + Into<Vec<u8>> + Clone,
    CI: Serialize + DeserializeOwned + Debug + Clone + ConciseSerde,
{
    /// Computes the favor factor of a [`Testcase`]. Lower is better.
    fn compute(state: &S,
               entry: &mut Testcase<S::Input>,
               id: CorpusId,
               prev_inputs: Vec<[u8;4]>) -> Result<f64, Error>;
}

/// The mutational stage using power schedules
#[derive(Clone, Debug)]
pub struct PowerMutationalStageWithId<E, F, EM, I, M, Z, VI, VS, Loc, Addr, Out, CI> {
    mutator: M,
    #[allow(clippy::type_complexity)]
    phantom: PhantomData<(E, F, EM, I, M, Z, VI, VS, Loc, Addr, Out, CI)>,
}

impl<E, F, EM, I, M, Z, VI, VS, Loc, Addr, Out, CI> UsesState for PowerMutationalStageWithId<E, F, EM, I, M, Z, VI, VS, Loc, Addr, Out, CI>
where
    E: UsesState,
{
    type State = E::State;
}

impl<E, F, EM, I, M, Z, VI, VS, Loc, Addr, Out, CI> MutationalStage<E, EM, I, M, Z> for PowerMutationalStageWithId<E, F, EM, I, M, Z, VI, VS, Loc, Addr, Out, CI>
where
    E: Executor<EM, Z> + HasObservers,
    EM: UsesState<State = E::State>,
    F: TestcaseScoreWithId<E::State, VI, VS, Loc, Addr, Out, CI>,
    M: Mutator<I, E::State>,
    E::State: HasClientPerfMonitor + HasCorpus<Input = VI> + HasMetadata + HasRand + HasFavored + HasInfantStateState<Loc, Addr, VS, CI> + HasExecutionResult<Loc, Addr, VS, Out, CI>,
    Z: Evaluator<E, EM, State = E::State>,
    I: MutatedTransform<E::Input, E::State> + Clone,
    VI: VMInputT<VS, Loc, Addr, CI> + Input,
    VS: Default + VMStateT,
    Addr: Serialize + DeserializeOwned + Debug + Clone,
    Loc: Serialize + DeserializeOwned + Debug + Clone,
    Out: Default + Into<Vec<u8>> + Clone,
    CI: Serialize + DeserializeOwned + Debug + Clone + ConciseSerde,
{
    /// The mutator, added to this stage
    #[inline]
    fn mutator(&self) -> &M {
        &self.mutator
    }

    /// The list of mutators, added to this stage (as mutable ref)
    #[inline]
    fn mutator_mut(&mut self) -> &mut M {
        &mut self.mutator
    }

    /// Gets the number of iterations as a random number
    #[allow(clippy::cast_sign_loss)]
    fn iterations(&self, state: &mut E::State, corpus_idx: CorpusId) -> Result<u64, Error> {
        // Update handicap
        let prev_inputs = state.get_execution_result().new_state.trace.clone().get_function_calls(state);
        let mut testcase = state.corpus().get(corpus_idx)?.borrow_mut();
        let score = F::compute(state, &mut *testcase, corpus_idx, prev_inputs)? as u64;
        Ok(score)
    }
}

impl<E, F, EM, I, M, Z, VI, VS, Loc, Addr, Out, CI> Stage<E, EM, Z> for PowerMutationalStageWithId<E, F, EM, I, M, Z, VI, VS, Loc, Addr, Out, CI>
where
    E: Executor<EM, Z> + HasObservers,
    EM: UsesState<State = E::State>,
    F: TestcaseScoreWithId<E::State, VI, VS, Loc, Addr, Out, CI>,
    M: Mutator<I, E::State>,
    E::State: HasClientPerfMonitor + HasCorpus<Input = VI> + HasMetadata + HasRand + HasFavored + HasInfantStateState<Loc, Addr, VS, CI> + HasExecutionResult<Loc, Addr, VS, Out, CI>,
    Z: Evaluator<E, EM, State = E::State>,
    I: MutatedTransform<E::Input, E::State> + Clone,
    VI: VMInputT<VS, Loc, Addr, CI> + Input,
    VS: Default + VMStateT,
    Addr: Serialize + DeserializeOwned + Debug + Clone,
    Loc: Serialize + DeserializeOwned + Debug + Clone,
    Out: Default + Into<Vec<u8>> + Clone,
    CI: Serialize + DeserializeOwned + Debug + Clone + ConciseSerde,
{
    #[inline]
    #[allow(clippy::let_and_return)]
    fn perform(
        &mut self,
        fuzzer: &mut Z,
        executor: &mut E,
        state: &mut E::State,
        manager: &mut EM,
        corpus_idx: CorpusId,
    ) -> Result<(), Error> {
        let ret = self.perform_mutational(fuzzer, executor, state, manager, corpus_idx);
        ret
    }
}

impl<E, F, EM, M, Z, VI, VS, Loc, Addr, Out, CI> PowerMutationalStageWithId<E, F, EM, E::Input, M, Z, VI, VS, Loc, Addr, Out, CI>
where
    E: Executor<EM, Z> + HasObservers,
    EM: UsesState<State = E::State>,
    F: TestcaseScoreWithId<E::State, VI, VS, Loc, Addr, Out, CI>,
    M: Mutator<E::Input, E::State>,
    E::State: HasClientPerfMonitor + HasCorpus<Input = VI> + HasMetadata + HasRand + HasFavored + HasInfantStateState<Loc, Addr, VS, CI>,
    Z: Evaluator<E, EM, State = E::State>,
    VI: VMInputT<VS, Loc, Addr, CI> + Input,
    VS: Default + VMStateT,
    Addr: Serialize + DeserializeOwned + Debug + Clone,
    Loc: Serialize + DeserializeOwned + Debug + Clone,
    Out: Default + Into<Vec<u8>> + Clone,
    CI: Serialize + DeserializeOwned + Debug + Clone + ConciseSerde,
{
    /// Creates a new [`PowerMutationalStageWithId`]
    pub fn new(mutator: M) -> Self {
        Self::transforming(mutator)
    }
}

impl<E, F, EM, I, M, Z, VI, VS, Loc, Addr, Out, CI> PowerMutationalStageWithId<E, F, EM, I, M, Z, VI, VS, Loc, Addr, Out, CI>
where
    E: Executor<EM, Z> + HasObservers,
    EM: UsesState<State = E::State>,
    F: TestcaseScoreWithId<E::State, VI, VS, Loc, Addr, Out, CI>,
    M: Mutator<I, E::State>,
    E::State: HasClientPerfMonitor + HasCorpus<Input = VI> + HasMetadata + HasRand + HasFavored + HasInfantStateState<Loc, Addr, VS, CI>,
    Z: Evaluator<E, EM, State = E::State>,
    VI: VMInputT<VS, Loc, Addr, CI> + Input,
    VS: Default + VMStateT,
    Addr: Serialize + DeserializeOwned + Debug + Clone,
    Loc: Serialize + DeserializeOwned + Debug + Clone,
    Out: Default + Into<Vec<u8>> + Clone,
    CI: Serialize + DeserializeOwned + Debug + Clone + ConciseSerde,
{
    /// Creates a new transforming [`PowerMutationalStageWithId`]
    pub fn transforming(mutator: M) -> Self {
        Self {
            mutator,
            phantom: PhantomData,
        }
    }
}
