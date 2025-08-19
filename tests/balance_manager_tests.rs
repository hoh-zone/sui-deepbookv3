use anyhow::Context;
use std::{collections::HashMap, str::FromStr};

use sui_deepbookv3::{
    transactions::balance_manager::BalanceManagerContract,
    utils::{
        config::{DeepBookConfig, Environment},
        types::BalanceManager,
    },
};
use sui_sdk::{
    types::{
        base_types::SuiAddress, programmable_transaction_builder::ProgrammableTransactionBuilder,
    },
    SuiClientBuilder,
};
use utils::dry_run_transaction;

mod utils;

#[tokio::test]
async fn test_create_and_share_balance_manager() -> anyhow::Result<()> {
    // Skip test if no wallet address is available
    let wallet_result = utils::retrieve_wallet();
    if wallet_result.is_err() {
        println!("Skipping test: no wallet address available");
        return Ok(());
    }

    let sui_client = SuiClientBuilder::default().build_testnet().await
        .context("Failed to build Sui testnet client")?;

    let config = deep_book_config();

    let balance_manager = BalanceManagerContract::new(sui_client.clone(), config);

    let mut ptb = ProgrammableTransactionBuilder::new();

    let _ = balance_manager.create_and_share_balance_manager(&mut ptb);
    // execute_transaction(ptb).await;
    let result = dry_run_transaction(&sui_client, ptb).await
        .context("Failed to dry run transaction")?;
    
    // Assert that the transaction was successful
    assert!(!result.is_empty(), "Transaction should return results");
    
    Ok(())
}

#[tokio::test]
async fn test_balance_manager_owner() -> anyhow::Result<()> {
    // Skip test if no wallet address is available
    let wallet_result = utils::retrieve_wallet();
    if wallet_result.is_err() {
        println!("Skipping test: no wallet address available");
        return Ok(());
    }

    let sui_client = SuiClientBuilder::default().build_testnet().await
        .context("Failed to build Sui testnet client")?;

    let config = deep_book_config();

    let balance_manager = BalanceManagerContract::new(sui_client.clone(), config);

    let mut ptb = ProgrammableTransactionBuilder::new();

    let _ = balance_manager.owner(&mut ptb, "DEEP").await;

    let result = dry_run_transaction(&sui_client, ptb).await
        .context("Failed to dry run transaction")?;
    let result = result.first()
        .ok_or_else(|| anyhow::anyhow!("No results found"))?;
    let owner_address = bcs::from_bytes::<SuiAddress>(&result.0)
        .context("Failed to deserialize owner address")?;
    
    assert_eq!(
        owner_address,
        SuiAddress::from_str("0x7731f9c105f3c2bde96f0eca645e718465394d609139342f3196383b823890a9")
            .context("Failed to parse expected SuiAddress")?
    );
    Ok(())
}

#[tokio::test]
async fn test_balance_manager_id() -> anyhow::Result<()> {
    // Skip test if no wallet address is available
    let wallet_result = utils::retrieve_wallet();
    if wallet_result.is_err() {
        println!("Skipping test: no wallet address available");
        return Ok(());
    }

    let sui_client = SuiClientBuilder::default().build_testnet().await
        .context("Failed to build Sui testnet client")?;

    let config = deep_book_config();

    let balance_manager = BalanceManagerContract::new(sui_client.clone(), config);

    let mut ptb = ProgrammableTransactionBuilder::new();

    let _ = balance_manager.id(&mut ptb, "DEEP").await;

    let result = dry_run_transaction(&sui_client, ptb).await
        .context("Failed to dry run transaction")?;
    let result = result.first()
        .ok_or_else(|| anyhow::anyhow!("No results found"))?;
    let id_address = bcs::from_bytes::<SuiAddress>(&result.0)
        .context("Failed to deserialize ID address")?;
    
    assert_eq!(
        id_address,
        SuiAddress::from_str("0x722c39b7b79831d534fbfa522e07101cb881f8807c28b9cf03a58b04c6c5ca9a")
            .context("Failed to parse expected SuiAddress")?
    );
    Ok(())
}

fn deep_book_config() -> DeepBookConfig {
    let balance_managers = HashMap::from([(
        "DEEP",
        BalanceManager {
            address: "0x722c39b7b79831d534fbfa522e07101cb881f8807c28b9cf03a58b04c6c5ca9a"
                .to_string(),
            trade_cap: None,
            deposit_cap: None,
            withdraw_cap: None,
        },
    )]);

    DeepBookConfig::new(
        Environment::Testnet,
        SuiAddress::random_for_testing_only(),
        None,
        Some(balance_managers),
        None,
        None,
    )
}
