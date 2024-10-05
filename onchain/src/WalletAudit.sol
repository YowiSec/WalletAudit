// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

interface IERC20 {
    function balanceOf(address account) external view returns (uint256);
}

contract WalletAudit {
    
    struct TokenBalance {
        address token;
        uint256 balance;
    }

    // Fetch ERC20 token balances for a given wallet address
    function getTokenBalances(address wallet, address[] memory tokenAddresses) public view returns (TokenBalance[] memory) {
        TokenBalance[] memory balances = new TokenBalance[](tokenAddresses.length);

        for (uint i = 0; i < tokenAddresses.length; i++) {
            address tokenAddress = tokenAddresses[i];
            uint256 balance = IERC20(tokenAddress).balanceOf(wallet);
            balances[i] = TokenBalance({
                token: tokenAddress,
                balance: balance
            });
        }

        return balances;
    }
} 
