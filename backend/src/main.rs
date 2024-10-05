use ethers::prelude::*;
use std::convert::TryFrom;
use std::sync::Arc;
use tokio;

// Replace with your deployed contract's address
const CONTRACT_ADDRESS: &str = "0xYourContractAddress";

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Connect to Ethereum Node using Infura/Alchemy
    let provider = Provider::<Http>::try_from("https://mainnet.infura.io/v3/YOUR_INFURA_PROJECT_ID")?;
    let client = Arc::new(provider);

    // Define the wallet audit contract
    let contract_address: Address = CONTRACT_ADDRESS.parse()?;
    let abi = include_bytes!("../onchain/src/WalletAudit.abi"); // Include ABI
    let contract = Contract::from_json(client.clone(), contract_address, abi)?;

    // Define wallet and tokens to analyze
    let wallet: Address = "0xWalletToAnalyze".parse()?;
    let tokens: Vec<Address> = vec![
        "0xToken1Address".parse()?,
        "0xToken2Address".parse()?,
    ];

    // Fetch token balances using the contract
    let token_balances: Vec<(Address, U256)> = contract
        .method::<_, Vec<(Address, U256)>>("getTokenBalances", (wallet, tokens))?
        .call()
        .await?;

    // Print token balances
    for (token, balance) in token_balances {
        println!("Token: {:?}, Balance: {:?}", token, balance);
    }

    Ok(())
}
