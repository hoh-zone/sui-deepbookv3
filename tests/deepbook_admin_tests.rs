use std::collections::HashMap;

use sui_deepbookv3::{
    transactions::deepbook_admin::DeepBookAdminContract,
    utils::{
        config::{DeepBookConfig, Environment},
        types::{Coin, Pool},
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
async fn test_adjust_tick_size() {
    let sui_client = SuiClientBuilder::default().build_testnet().await.unwrap();

    let config = deep_book_config();

    let deepbook_admin = DeepBookAdminContract::new(sui_client.clone(), config);

    let mut ptb = ProgrammableTransactionBuilder::new();

    let _ = deepbook_admin
        .adjust_tick_size(&mut ptb, "SUI_USDC", 100.0)
        .await;

    let result = dry_run_transaction(&sui_client, ptb).await.unwrap();
    assert!(result.is_empty());
}

#[tokio::test]
async fn test_adjust_min_lot_size() {
    let sui_client = SuiClientBuilder::default().build_testnet().await.unwrap();

    let config = deep_book_config();

    let deepbook_admin = DeepBookAdminContract::new(sui_client.clone(), config);

    let mut ptb = ProgrammableTransactionBuilder::new();

    let _ = deepbook_admin
        .adjust_min_lot_size(&mut ptb, "SUI_USDC", 100.0, 100.0)
        .await;

    let result = dry_run_transaction(&sui_client, ptb).await.unwrap();
    assert!(result.is_empty());
}

fn deep_book_config() -> DeepBookConfig {
    let mut wallet = utils::retrieve_wallet().unwrap();
    let admin_cap = Some(
        "0x7731f9c105f3c2bde96f0eca645e718465394d609139342f3196383b823890a9".to_string(),
    );

    let coins = HashMap::from([(
        "SUI",
        Coin {
            address: "0x2::sui::SUI".to_string(),
            type_name: "0x2::sui::SUI".to_string(),
            scalar: 1_000_000_000,
        },
    )]);

    let pools = HashMap::from([(
        "SUI_USDC",
        Pool {
            address: "0x722c39b7b79831d534fbfa522e07101cb881f8807c28b9cf03a58b04c6c5ca9a"
                .to_string(),
            base_coin: "SUI".to_string(),
            quote_coin: "USDC".to_string(),
        },
    )]);

    DeepBookConfig::new(
        Environment::Testnet,
        wallet.active_address().unwrap(),
        admin_cap,
        None,
        Some(coins),
        Some(pools),
    )
}