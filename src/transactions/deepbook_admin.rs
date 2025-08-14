use std::str::FromStr;

use anyhow::Result;
use sui_sdk::{
    types::{
        base_types::{ObjectID, SuiAddress},
        programmable_transaction_builder::ProgrammableTransactionBuilder,
        Identifier, TypeTag,SUI_CLOCK_OBJECT_ID
    },
    SuiClient,
};

use crate::utils::config::{DeepBookConfig, FLOAT_SCALAR};

use crate::DataReader;

/// Parameters for creating a pool as admin
pub struct CreatePoolAdminParams {
    pub base_coin_key: String,
    pub quote_coin_key: String,
    pub tick_size: f64,
    pub lot_size: f64,
    pub min_size: f64,
    pub whitelisted: bool,
    pub stable_pool: bool,
}

/// DeepBookAdminContract struct for managing admin actions
pub struct DeepBookAdminContract {
    client: SuiClient,
    config: DeepBookConfig,
}

impl DeepBookAdminContract {
    /// Creates a new DeepBookAdminContract instance
    ///
    /// @param client - SuiClient instance
    /// @param config - Configuration for DeepBookAdminContract
    pub fn new(client: SuiClient, config: DeepBookConfig) -> Self {
        Self { client, config }
    }

    /// Gets the admin capability required for admin operations
    ///
    /// @returns The admin capability
    /// @throws Error if the admin capability is not set
    fn admin_cap(&self) -> anyhow::Result<String> {
        self.config
            .admin_cap()
            .ok_or_else(|| anyhow::anyhow!("ADMIN_CAP environment variable not set"))
    }

    /// Create a new pool as admin
    ///
    /// @param ptb - ProgrammableTransactionBuilder instance
    /// @param params - Parameters for creating pool as admin
    pub async fn create_pool_admin(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        params: CreatePoolAdminParams,
    ) -> Result<()> {
        let base_coin = self.config.get_coin(&params.base_coin_key)?;
        let quote_coin = self.config.get_coin(&params.quote_coin_key)?;

        let base_scalar = base_coin.scalar;
        let quote_scalar = quote_coin.scalar;

        let adjusted_tick_size =
            (params.tick_size * FLOAT_SCALAR as f64 * quote_scalar as f64) / base_scalar as f64;
        let adjusted_lot_size = params.lot_size * base_scalar as f64;
        let adjusted_min_size = params.min_size * base_scalar as f64;

        let base_coin_tag = TypeTag::from_str(&base_coin.type_name)?;
        let quote_coin_tag = TypeTag::from_str(&quote_coin.type_name)?;

        let registry_id = ObjectID::from_hex_literal(self.config.registry_id())?;
        let admin_cap = ObjectID::from_hex_literal(&self.admin_cap()?)?;
        let arguments = vec![
            ptb.obj(self.client.share_object(registry_id).await?)?,
            ptb.pure(adjusted_tick_size as u64)?,
            ptb.pure(adjusted_lot_size as u64)?,
            ptb.pure(adjusted_min_size as u64)?,
            ptb.pure(params.whitelisted)?,
            ptb.pure(params.stable_pool)?,
            ptb.obj(self.client.share_object(admin_cap).await?)?,
        ];

        ptb.programmable_move_call(
            ObjectID::from_hex_literal(self.config.deepbook_package_id())?,
            Identifier::new("pool")?,
            Identifier::new("create_pool_admin")?,
            vec![base_coin_tag, quote_coin_tag],
            arguments,
        );

        Ok(())
    }

    /// Unregister a pool as admin
    ///
    /// @param ptb - ProgrammableTransactionBuilder instance
    /// @param pool_key - The key of the pool to be unregistered by admin
    pub async fn unregister_pool_admin(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        pool_key: &str,
    ) -> Result<()> {
        let pool = self.config.get_pool(pool_key)?;
        let base_coin = self.config.get_coin(&pool.base_coin)?;
        let quote_coin = self.config.get_coin(&pool.quote_coin)?;

        let base_coin_tag = TypeTag::from_str(&base_coin.type_name)?;
        let quote_coin_tag = TypeTag::from_str(&quote_coin.type_name)?;

        let pool_id = ObjectID::from_hex_literal(&pool.address)?;
        let registry_id = ObjectID::from_hex_literal(self.config.registry_id())?;
        let admin_cap = ObjectID::from_hex_literal(&self.admin_cap()?)?;

        let arguments = vec![
            ptb.obj(self.client.share_object(pool_id).await?)?,
            ptb.obj(self.client.share_object(registry_id).await?)?,
            ptb.obj(self.client.share_object(admin_cap).await?)?,
        ];

        ptb.programmable_move_call(
            ObjectID::from_hex_literal(self.config.deepbook_package_id())?,
            Identifier::new("pool")?,
            Identifier::new("unregister_pool_admin")?,
            vec![base_coin_tag, quote_coin_tag],
            arguments,
        );

        Ok(())
    }

    /// Update the allowed versions for a pool
    ///
    /// @param ptb - ProgrammableTransactionBuilder instance
    /// @param pool_key - The key of the pool to be updated
    pub async fn update_allowed_versions(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        pool_key: &str,
    ) -> Result<()> {
        let pool = self.config.get_pool(pool_key)?;
        let base_coin = self.config.get_coin(&pool.base_coin)?;
        let quote_coin = self.config.get_coin(&pool.quote_coin)?;

        let base_coin_tag = TypeTag::from_str(&base_coin.type_name)?;
        let quote_coin_tag = TypeTag::from_str(&quote_coin.type_name)?;

        let pool_id = ObjectID::from_hex_literal(&pool.address)?;
        let registry_id = ObjectID::from_hex_literal(self.config.registry_id())?;
        let admin_cap = ObjectID::from_hex_literal(&self.admin_cap()?)?;

        let arguments = vec![
            ptb.obj(self.client.share_object(pool_id).await?)?,
            ptb.obj(self.client.share_object(registry_id).await?)?,
            ptb.obj(self.client.share_object(admin_cap).await?)?,
        ];

        ptb.programmable_move_call(
            ObjectID::from_hex_literal(self.config.deepbook_package_id())?,
            Identifier::new("pool")?,
            Identifier::new("update_allowed_versions")?,
            vec![base_coin_tag, quote_coin_tag],
            arguments,
        );

        Ok(())
    }

    /// Enable a specific version
    ///
    /// @param ptb - ProgrammableTransactionBuilder instance
    /// @param version - The version to be enabled
    pub async fn enable_version(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        version: u64,
    ) -> Result<()> {
        let registry_id = ObjectID::from_hex_literal(self.config.registry_id())?;
        let admin_cap = ObjectID::from_hex_literal(&self.admin_cap()?)?;

        let arguments = vec![
            ptb.obj(self.client.share_object(registry_id).await?)?,
            ptb.pure(version)?,
            ptb.obj(self.client.share_object(admin_cap).await?)?,
        ];

        ptb.programmable_move_call(
            ObjectID::from_hex_literal(self.config.deepbook_package_id())?,
            Identifier::new("registry")?,
            Identifier::new("enable_version")?,
            vec![],
            arguments,
        );

        Ok(())
    }

    /// Disable a specific version
    ///
    /// @param ptb - ProgrammableTransactionBuilder instance
    /// @param version - The version to be disabled
    pub async fn disable_version(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        version: u64,
    ) -> Result<()> {
        let registry_id = ObjectID::from_hex_literal(self.config.registry_id())?;
        let admin_cap = ObjectID::from_hex_literal(&self.admin_cap()?)?;

        let arguments = vec![
            ptb.obj(self.client.share_object(registry_id).await?)?,
            ptb.pure(version)?,
            ptb.obj(self.client.share_object(admin_cap).await?)?,
        ];

        ptb.programmable_move_call(
            ObjectID::from_hex_literal(self.config.deepbook_package_id())?,
            Identifier::new("registry")?,
            Identifier::new("disable_version")?,
            vec![],
            arguments,
        );

        Ok(())
    }

    /// Sets the treasury address where pool creation fees will be sent
    ///
    /// @param ptb - ProgrammableTransactionBuilder instance
    /// @param treasury_address - The treasury address
    pub async fn set_treasury_address(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        treasury_address: &str,
    ) -> Result<()> {
        let registry_id = ObjectID::from_hex_literal(self.config.registry_id())?;
        let admin_cap = ObjectID::from_hex_literal(&self.admin_cap()?)?;

        let arguments = vec![
            ptb.obj(self.client.share_object(registry_id).await?)?,
            ptb.pure(SuiAddress::from_str(treasury_address)?)?,
            ptb.obj(self.client.share_object(admin_cap).await?)?,
        ];

        ptb.programmable_move_call(
            ObjectID::from_hex_literal(self.config.deepbook_package_id())?,
            Identifier::new("registry")?,
            Identifier::new("set_treasury_address")?,
            vec![],
            arguments,
        );

        Ok(())
    }

    /// Add a coin to whitelist of stable coins
    ///
    /// @param ptb - ProgrammableTransactionBuilder instance
    /// @param stable_coin_key - The name of the stable coin to be added
    pub async fn add_stable_coin(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        stable_coin_key: &str,
    ) -> Result<()> {
        let stable_coin = self.config.get_coin(stable_coin_key)?;
        let stable_coin_tag = TypeTag::from_str(&stable_coin.type_name)?;

        let registry_id = ObjectID::from_hex_literal(self.config.registry_id())?;
        let admin_cap = ObjectID::from_hex_literal(&self.admin_cap()?)?;

        let arguments = vec![
            ptb.obj(self.client.share_object(registry_id).await?)?,
            ptb.obj(self.client.share_object(admin_cap).await?)?,
        ];

        ptb.programmable_move_call(
            ObjectID::from_hex_literal(self.config.deepbook_package_id())?,
            Identifier::new("registry")?,
            Identifier::new("add_stablecoin")?,
            vec![stable_coin_tag],
            arguments,
        );

        Ok(())
    }

    /// Remove a coin from whitelist of stable coins
    ///
    /// @param ptb - ProgrammableTransactionBuilder instance 
    /// @param stable_coin_key - The name of the stable coin to be removed
    pub async fn remove_stable_coin(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        stable_coin_key: &str,
    ) -> Result<()> {
        let stable_coin = self.config.get_coin(stable_coin_key)?;
        let stable_coin_tag = TypeTag::from_str(&stable_coin.type_name)?;

        let registry_id = ObjectID::from_hex_literal(self.config.registry_id())?;
        let admin_cap = ObjectID::from_hex_literal(&self.admin_cap()?)?;

        let arguments = vec![
            ptb.obj(self.client.share_object(registry_id).await?)?,
            ptb.obj(self.client.share_object(admin_cap).await?)?,
        ];

        ptb.programmable_move_call(
            ObjectID::from_hex_literal(self.config.deepbook_package_id())?,
            Identifier::new("registry")?,
            Identifier::new("remove_stablecoin")?,
            vec![stable_coin_tag],
            arguments,
        );

        Ok(())
    }

    /// Adjust the tick size of a pool
    ///
    /// @param ptb - ProgrammableTransactionBuilder instance
    /// @param pool_key - The key to identify the pool
    /// @param new_tick_size - The new tick size
    pub async fn adjust_tick_size(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        pool_key: &str,
        new_tick_size: f64,
    ) -> Result<()> {
        let pool = self.config.get_pool(pool_key)?;
        let base_coin = self.config.get_coin(&pool.base_coin)?;
        let quote_coin = self.config.get_coin(&pool.quote_coin)?;

        let base_scalar = base_coin.scalar;
        let quote_scalar = quote_coin.scalar;

        let adjusted_tick_size =
            ((new_tick_size * FLOAT_SCALAR as f64 * quote_scalar as f64) / base_scalar as f64).round() as u64;

        let base_coin_tag = TypeTag::from_str(&base_coin.type_name)?;
        let quote_coin_tag = TypeTag::from_str(&quote_coin.type_name)?;

        let pool_id = ObjectID::from_hex_literal(&pool.address)?;
        let admin_cap = ObjectID::from_hex_literal(&self.admin_cap()?)?;

        let arguments = vec![
            ptb.obj(self.client.share_object(pool_id).await?)?,
            ptb.pure(adjusted_tick_size)?,
            ptb.obj(self.client.share_object(admin_cap).await?)?,
            ptb.obj(self.client.share_object(SUI_CLOCK_OBJECT_ID).await?)?,
        ];

        ptb.programmable_move_call(
            ObjectID::from_hex_literal(self.config.deepbook_package_id())?,
            Identifier::new("pool")?,
            Identifier::new("adjust_tick_size_admin")?,
            vec![base_coin_tag, quote_coin_tag],
            arguments,
        );

        Ok(())
    }

    /// Adjust the lot size and min size of a pool
    ///
    /// @param ptb - ProgrammableTransactionBuilder instance
    /// @param pool_key - The key to identify the pool
    /// @param new_lot_size - The new lot size
    /// @param new_min_size - The new min size
    pub async fn adjust_min_lot_size(
        &self,
        ptb: &mut ProgrammableTransactionBuilder,
        pool_key: &str,
        new_lot_size: f64,
        new_min_size: f64,
    ) -> Result<()> {
        let pool = self.config.get_pool(pool_key)?;
        let base_coin = self.config.get_coin(&pool.base_coin)?;
        let quote_coin = self.config.get_coin(&pool.quote_coin)?;

        let base_scalar = base_coin.scalar;

        let adjusted_lot_size = new_lot_size * base_scalar as f64;
        let adjusted_min_size = new_min_size * base_scalar as f64;

        let base_coin_tag = TypeTag::from_str(&base_coin.type_name)?;
        let quote_coin_tag = TypeTag::from_str(&quote_coin.type_name)?;

        let pool_id = ObjectID::from_hex_literal(&pool.address)?;
        let admin_cap = ObjectID::from_hex_literal(&self.admin_cap()?)?;

        let arguments = vec![
            ptb.obj(self.client.share_object(pool_id).await?)?,
            ptb.pure(adjusted_lot_size as u64)?,
            ptb.pure(adjusted_min_size as u64)?,
            ptb.obj(self.client.share_object(admin_cap).await?)?,
            ptb.obj(self.client.share_object(SUI_CLOCK_OBJECT_ID).await?)?,
        ];

        ptb.programmable_move_call(
            ObjectID::from_hex_literal(self.config.deepbook_package_id())?,
            Identifier::new("pool")?,
            Identifier::new("adjust_min_lot_size_admin")?,
            vec![base_coin_tag, quote_coin_tag],
            arguments,
        );

        Ok(())
    }
}
