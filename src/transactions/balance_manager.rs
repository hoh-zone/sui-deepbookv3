use std::str::FromStr;
use sui_sdk::types::base_types::{ObjectID, SuiAddress};
use sui_sdk::types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use sui_sdk::types::transaction::Argument;
use sui_sdk::types::{Identifier, TypeTag, SUI_FRAMEWORK_PACKAGE_ID};
use sui_sdk::SuiClient;

use crate::utils::config::DeepBookConfig;

use crate::DataReader;

/// BalanceManagerContract struct for managing BalanceManager operations.
#[derive(Clone)]
pub struct BalanceManagerContract {
    client: SuiClient,
    config: DeepBookConfig,
}

impl BalanceManagerContract {
    /// Creates a new instance of BalanceManagerContract
    ///
    /// @param client - SuiClient instance
    /// @param config - Configuration object for DeepBook
    pub fn new(client: SuiClient, config: DeepBookConfig) -> Self {
        Self { client, config }
    }

    /// Create and share a new BalanceManager
    ///
    /// @param ptb - ProgrammableTransactionBuilder instance
    pub fn create_and_share_balance_manager(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
    ) -> anyhow::Result<()> {
        let package_id = ObjectID::from_hex_literal(self.config.deepbook_package_id())?;
        let manager = ptb.programmable_move_call(
            package_id,
            Identifier::new("balance_manager")?,
            Identifier::new("new")?,
            vec![],
            vec![],
        );

        let manager_tag = TypeTag::from_str(
            format!(
                "{}::balance_manager::BalanceManager",
                self.config.deepbook_package_id()
            )
            .as_str(),
        )?;

        ptb.programmable_move_call(
            SUI_FRAMEWORK_PACKAGE_ID,
            Identifier::new("transfer")?,
            Identifier::new("public_share_object")?,
            vec![manager_tag],
            vec![manager],
        );
        Ok(())
    }

    /// Create and share a new BalanceManager with a specified owner
    ///
    /// @param ptb - ProgrammableTransactionBuilder instance
    /// @param owner_address - The address of the owner
    pub fn create_and_share_balance_manager_with_owner(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        owner_address: SuiAddress,
    ) -> anyhow::Result<()> {
        let package_id = ObjectID::from_hex_literal(self.config.deepbook_package_id())?;

        let arguments = vec![ptb.pure(owner_address)?];

        let manager = ptb.programmable_move_call(
            package_id,
            Identifier::new("balance_manager")?,
            Identifier::new("new_with_custom_owner")?,
            vec![],
            arguments,
        );

        let manager_tag = TypeTag::from_str(
            format!(
                "{}::balance_manager::BalanceManager",
                self.config.deepbook_package_id()
            )
            .as_str(),
        )?;

        ptb.programmable_move_call(
            SUI_FRAMEWORK_PACKAGE_ID,
            Identifier::new("transfer")?,
            Identifier::new("public_share_object")?,
            vec![manager_tag],
            vec![manager],
        );
        Ok(())
    }

    /// Deposit funds into the BalanceManager
    ///
    /// @param ptb - ProgrammableTransactionBuilder instance
    /// @param manager_key - The key to identify the BalanceManager
    /// @param coin_key - The key to identify the coin
    /// @param amount_to_deposit - The amount to deposit
    pub async fn deposit_into_manager(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        sender: SuiAddress,
        manager_key: &str,
        coin_key: &str,
        amount_to_deposit: f64,
    ) -> anyhow::Result<()> {
        let manager_address = self
            .config
            .get_balance_manager(manager_key)?
            .address
            .as_str();
        let manager_id = ObjectID::from_hex_literal(manager_address)?;
        let package_id = ObjectID::from_hex_literal(self.config.deepbook_package_id())?;
        let coin = self.config.get_coin(coin_key)?.clone();
        let deposit_input = (amount_to_deposit * coin.scalar as f64).round() as u64;
        let deposit_coin = self
            .client
            .get_coin_object(sender, coin.type_name.clone(), deposit_input)
            .await?;

        let arguments = vec![
            ptb.obj(self.client.share_object_mutable(manager_id).await?)?,
            ptb.obj(self.client.coin_object(deposit_coin).await?)?,
        ];

        ptb.programmable_move_call(
            package_id,
            Identifier::new("balance_manager")?,
            Identifier::new("deposit")?,
            vec![TypeTag::from_str(coin.type_name.as_str())?],
            arguments,
        );

        Ok(())
    }

    /// Withdraw funds from the BalanceManager
    ///
    /// @param ptb - ProgrammableTransactionBuilder instance
    /// @param manager_key - The key to identify the BalanceManager
    /// @param coin_key - The key to identify the coin
    /// @param amount_to_withdraw - The amount to withdraw
    /// @param recipient - The recipient of the funds
    pub async fn withdraw_from_manager(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        manager_key: &str,
        coin_key: &str,
        amount_to_withdraw: f64,
        recipient: SuiAddress,
    ) -> anyhow::Result<()> {
        let manager_address = self
            .config
            .get_balance_manager(manager_key)?
            .address
            .as_str();
        let manager_id = ObjectID::from_hex_literal(manager_address)?;
        let package_id = ObjectID::from_hex_literal(self.config.deepbook_package_id())?;
        let coin = self.config.get_coin(coin_key)?;
        let withdraw_input = (amount_to_withdraw * coin.scalar as f64).round() as u64;

        let arguments = vec![
            ptb.obj(self.client.share_object_mutable(manager_id).await?)?,
            ptb.pure(withdraw_input)?,
        ];
        let coin_object = ptb.programmable_move_call(
            package_id,
            Identifier::new("balance_manager")?,
            Identifier::new("withdraw")?,
            vec![TypeTag::from_str(coin.type_name.as_str())?],
            arguments,
        );

        ptb.transfer_arg(recipient, coin_object);
        Ok(())
    }

    /// Withdraw all funds from the BalanceManager
    ///
    /// @param ptb - ProgrammableTransactionBuilder instance
    /// @param manager_key - The key to identify the BalanceManager
    /// @param coin_key - The key to identify the coin
    /// @param recipient - The recipient of the funds
    pub async fn withdraw_all_from_manager(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        manager_key: &str,
        coin_key: &str,
        recipient: SuiAddress,
    ) -> anyhow::Result<()> {
        let manager_address = self
            .config
            .get_balance_manager(manager_key)?
            .address
            .as_str();
        let manager_id = ObjectID::from_hex_literal(manager_address)?;
        let package_id = ObjectID::from_hex_literal(self.config.deepbook_package_id())?;
        let coin = self.config.get_coin(coin_key)?;

        let arguments = vec![ptb.obj(self.client.share_object(manager_id).await?)?];
        let withdrawal_coin = ptb.programmable_move_call(
            package_id,
            Identifier::new("balance_manager")?,
            Identifier::new("withdraw_all")?,
            vec![TypeTag::from_str(coin.type_name.as_str())?],
            arguments,
        );

        ptb.transfer_arg(recipient, withdrawal_coin);
        Ok(())
    }

    /// Check the balance of the BalanceManager
    ///
    /// @param ptb - ProgrammableTransactionBuilder instance
    /// @param manager_key - The key to identify the BalanceManager
    /// @param coin_key - The key to identify the coin
    pub async fn check_manager_balance(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        manager_key: &str,
        coin_key: &str,
    ) -> anyhow::Result<Argument> {
        let manager_address = self
            .config
            .get_balance_manager(manager_key)?
            .address
            .as_str();
        let manager_id = ObjectID::from_hex_literal(manager_address)?;
        let package_id = ObjectID::from_hex_literal(self.config.deepbook_package_id())?;
        let coin = self.config.get_coin(coin_key)?;

        let arguments = vec![ptb.obj(self.client.share_object(manager_id).await?)?];

        Ok(ptb.programmable_move_call(
            package_id,
            Identifier::new("balance_manager")?,
            Identifier::new("balance")?,
            vec![TypeTag::from_str(coin.type_name.as_str())?],
            arguments,
        ))
    }

    /// Generate a trade proof for the BalanceManager
    ///
    /// @param ptb - ProgrammableTransactionBuilder instance
    /// @param manager_key - The key to identify the BalanceManager
    pub async fn generate_proof(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        manager_key: &str,
    ) -> anyhow::Result<Argument> {
        let balance_manager = self.config.get_balance_manager(manager_key)?;
        let manager_address = balance_manager.address.as_str();
        let trade_cap = balance_manager.trade_cap.clone();
        let manager_id = ObjectID::from_hex_literal(manager_address)?;

        if let Some(trade_cap) = trade_cap {
            let trade_cap_id = ObjectID::from_hex_literal(trade_cap.as_str())?;
            Ok(self
                .generate_proof_as_trader(ptb, &manager_id, &trade_cap_id)
                .await?)
        } else {
            Ok(self.generate_proof_as_owner(ptb, &manager_id).await?)
        }
    }

    /// Generate a trade proof as the owner
    ///
    /// @param ptb - ProgrammableTransactionBuilder instance
    /// @param manager_id - The ID of the BalanceManager
    pub async fn generate_proof_as_owner(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        manager_id: &ObjectID,
    ) -> anyhow::Result<Argument> {
        let package_id = ObjectID::from_hex_literal(self.config.deepbook_package_id())?;
        let arguments = vec![ptb.obj(self.client.share_object_mutable(*manager_id).await?)?];
        Ok(ptb.programmable_move_call(
            package_id,
            Identifier::new("balance_manager")?,
            Identifier::new("generate_proof_as_owner")?,
            vec![],
            arguments,
        ))
    }

    /// Generate a trade proof as a trader
    ///
    /// @param ptb - ProgrammableTransactionBuilder instance
    /// @param manager_id - The ID of the BalanceManager
    /// @param trade_cap_id - The ID of the TradeCap
    pub async fn generate_proof_as_trader(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        manager_id: &ObjectID,
        trade_cap_id: &ObjectID,
    ) -> anyhow::Result<Argument> {
        let package_id = ObjectID::from_hex_literal(self.config.deepbook_package_id())?;
        let arguments = vec![
            ptb.obj(self.client.share_object_mutable(*manager_id).await?)?,
            ptb.obj(self.client.share_object(*trade_cap_id).await?)?,
        ];
        Ok(ptb.programmable_move_call(
            package_id,
            Identifier::new("balance_manager")?,
            Identifier::new("generate_proof_as_trader")?,
            vec![],
            arguments,
        ))
    }

    /// Mint a TradeCap
    ///
    /// @param ptb - ProgrammableTransactionBuilder instance
    /// @param manager_key - The key to identify the BalanceManager
    pub async fn mint_trade_cap(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        manager_key: &str,
    ) -> anyhow::Result<()> {
        let package_id = ObjectID::from_hex_literal(self.config.deepbook_package_id())?;
        let manager_address = self
            .config
            .get_balance_manager(manager_key)?
            .address
            .as_str();
        let manager_id = ObjectID::from_hex_literal(manager_address)?;

        let arguments = vec![ptb.obj(self.client.share_object_mutable(manager_id).await?)?];
        ptb.programmable_move_call(
            package_id,
            Identifier::new("balance_manager")?,
            Identifier::new("mint_trade_cap")?,
            vec![],
            arguments,
        );
        Ok(())
    }

    /// Mint a DepositCap
    ///
    /// @param ptb - ProgrammableTransactionBuilder instance
    /// @param manager_key - The key to identify the BalanceManager
    pub async fn mint_deposit_cap(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        manager_key: &str,
    ) -> anyhow::Result<()> {
        let package_id = ObjectID::from_hex_literal(self.config.deepbook_package_id())?;
        let manager_address = self
            .config
            .get_balance_manager(manager_key)?
            .address
            .as_str();
        let manager_id = ObjectID::from_hex_literal(manager_address)?;

        let arguments = vec![ptb.obj(self.client.share_object_mutable(manager_id).await?)?];
        ptb.programmable_move_call(
            package_id,
            Identifier::new("balance_manager")?,
            Identifier::new("mint_deposit_cap")?,
            vec![],
            arguments,
        );
        Ok(())
    }

    /// Mint a WithdrawCap
    ///
    /// @param ptb - ProgrammableTransactionBuilder instance
    /// @param manager_key - The key to identify the BalanceManager
    pub async fn mint_withdraw_cap(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        manager_key: &str,
    ) -> anyhow::Result<()> {
        let package_id = ObjectID::from_hex_literal(self.config.deepbook_package_id())?;
        let manager_address = self
            .config
            .get_balance_manager(manager_key)?
            .address
            .as_str();
        let manager_id = ObjectID::from_hex_literal(manager_address)?;

        let arguments = vec![ptb.obj(self.client.share_object_mutable(manager_id).await?)?];
        ptb.programmable_move_call(
            package_id,
            Identifier::new("balance_manager")?,
            Identifier::new("mint_withdraw_cap")?,
            vec![],
            arguments,
        );
        Ok(())
    }

    /// Deposit using DepositCap
    ///
    /// @param ptb - ProgrammableTransactionBuilder instance
    /// @param manager_key - The key to identify the BalanceManager
    /// @param coin_key - The key to identify the coin
    /// @param amount_to_deposit - The amount to deposit
    pub async fn deposit_with_cap(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        sender: SuiAddress,
        manager_key: &str,
        coin_key: &str,
        amount_to_deposit: f64,
    ) -> anyhow::Result<()> {
        let balance_manager = self.config.get_balance_manager(manager_key)?;
        let manager_id = ObjectID::from_hex_literal(balance_manager.address.as_str())?;
        let deposit_cap_id = ObjectID::from_hex_literal(
            balance_manager
                .deposit_cap
                .clone()
                .ok_or_else(|| anyhow::anyhow!("DepositCap not set for {}", manager_key))?
                .as_str(),
        )?;
        let package_id = ObjectID::from_hex_literal(self.config.deepbook_package_id())?;
        let coin = self.config.get_coin(coin_key)?.clone();
        let deposit_input = (amount_to_deposit * coin.scalar as f64).round() as u64;
        let deposit_coin = self
            .client
            .get_coin_object(sender, coin.type_name.clone(), deposit_input)
            .await?;

        let arguments = vec![
            ptb.obj(self.client.share_object_mutable(manager_id).await?)?,
            ptb.obj(self.client.share_object(deposit_cap_id).await?)?,
            ptb.obj(self.client.coin_object(deposit_coin).await?)?,
        ];

        ptb.programmable_move_call(
            package_id,
            Identifier::new("balance_manager")?,
            Identifier::new("deposit_with_cap")?,
            vec![TypeTag::from_str(coin.type_name.as_str())?],
            arguments,
        );
        Ok(())
    }

    /// Withdraw using WithdrawCap
    ///
    /// @param ptb - ProgrammableTransactionBuilder instance
    /// @param manager_key - The key to identify the BalanceManager
    /// @param coin_key - The key to identify the coin
    /// @param amount_to_withdraw - The amount to withdraw
    pub async fn withdraw_with_cap(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        manager_key: &str,
        coin_key: &str,
        amount_to_withdraw: f64,
        recipient: SuiAddress,
    ) -> anyhow::Result<()> {
        let balance_manager = self.config.get_balance_manager(manager_key)?;
        let manager_id = ObjectID::from_hex_literal(balance_manager.address.as_str())?;
        let withdraw_cap_id = ObjectID::from_hex_literal(
            balance_manager
                .withdraw_cap
                .clone()
                .ok_or_else(|| anyhow::anyhow!("WithdrawCap not set for {}", manager_key))?
                .as_str(),
        )?;
        let package_id = ObjectID::from_hex_literal(self.config.deepbook_package_id())?;
        let coin = self.config.get_coin(coin_key)?;
        let withdraw_amount = (amount_to_withdraw * coin.scalar as f64).round() as u64;

        let arguments = vec![
            ptb.obj(self.client.share_object_mutable(manager_id).await?)?,
            ptb.obj(self.client.share_object(withdraw_cap_id).await?)?,
            ptb.pure(withdraw_amount)?,
        ];

        let coin_object = ptb.programmable_move_call(
            package_id,
            Identifier::new("balance_manager")?,
            Identifier::new("withdraw_with_cap")?,
            vec![TypeTag::from_str(coin.type_name.as_str())?],
            arguments,
        );

        ptb.transfer_arg(recipient, coin_object);
        Ok(())
    }

    /// Get the owner of the BalanceManager
    ///
    /// @param ptb - ProgrammableTransactionBuilder instance
    /// @param manager_key - The key to identify the BalanceManager
    pub async fn owner(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        manager_key: &str,
    ) -> anyhow::Result<Argument> {
        let package_id = ObjectID::from_hex_literal(self.config.deepbook_package_id())?;
        let manager_address = self
            .config
            .get_balance_manager(manager_key)?
            .address
            .as_str();
        let manager_id = ObjectID::from_hex_literal(manager_address)?;
        let arguments = vec![ptb.obj(self.client.share_object(manager_id).await?)?];
        Ok(ptb.programmable_move_call(
            package_id,
            Identifier::new("balance_manager")?,
            Identifier::new("owner")?,
            vec![],
            arguments,
        ))
    }

    /// Get the ID of the BalanceManager
    ///
    /// @param ptb - ProgrammableTransactionBuilder instance
    /// @param manager_key - The key to identify the BalanceManager
    pub async fn id(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        manager_key: &str,
    ) -> anyhow::Result<Argument> {
        let package_id = ObjectID::from_hex_literal(self.config.deepbook_package_id())?;
        let manager_address = self
            .config
            .get_balance_manager(manager_key)?
            .address
            .as_str();
        let manager_id = ObjectID::from_hex_literal(manager_address)?;
        let arguments = vec![ptb.obj(self.client.share_object(manager_id).await?)?];
        Ok(ptb.programmable_move_call(
            package_id,
            Identifier::new("balance_manager")?,
            Identifier::new("id")?,
            vec![],
            arguments,
        ))
    }
}
