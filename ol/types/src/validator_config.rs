//! validator config view for web monitor

use libra_types::{
    access_path::AccessPath,
    account_config::constants:: CORE_CODE_ADDRESS,
};
use anyhow::Result;
use move_core_types::{
    language_storage::{ResourceKey, StructTag},
    move_resource::MoveResource,
};
use serde::{Deserialize, Serialize};
use move_core_types::account_address::AccountAddress;

/// Struct that represents a Validator Config resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorConfigResource {
    ///
    pub config: Option<ConfigResource>,
    ///
    pub operator_account: Option<AccountAddress>,
    ///
    pub human_name: Vec<u8>,
}

/// Struct that represents a Config resource
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ConfigResource {
    consensus_pubkey: Vec<u8>,
    validator_network_addresses: Vec<u8>,
    fullnode_network_addresses: Vec<u8>,
}

impl MoveResource for ValidatorConfigResource {
    const MODULE_NAME: &'static str = "ValidatorConfig";
    const STRUCT_NAME: &'static str = "ValidatorConfig";
}

impl ValidatorConfigResource {
    ///
    pub fn struct_tag() -> StructTag {
        StructTag {
            address: CORE_CODE_ADDRESS,
            module: ValidatorConfigResource::module_identifier(),
            name: ValidatorConfigResource::struct_identifier(),
            type_params: vec![],
        }
    }
    ///
    pub fn access_path(account: AccountAddress) -> AccessPath {
        let resource_key = ResourceKey::new(
            account,
            ValidatorConfigResource::struct_tag(),
        );
        AccessPath::resource_access_path(&resource_key)
    }
    ///
    pub fn resource_path() -> Vec<u8> {
        AccessPath::resource_access_vec(&ValidatorConfigResource::struct_tag())
    }

    /// 
    pub fn try_from_bytes(bytes: &[u8]) -> Result<Self> {
        lcs::from_bytes(bytes).map_err(Into::into)
    }

    ///
    pub fn get_view(&self) -> ValidatorConfigView {
        ValidatorConfigView {
            operator_account: self.operator_account.clone(),
            operator_has_balance: None
        }
    }
}

/// Struct that represents a view for Validator Config view
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorConfigView {
    ///
    pub operator_account: Option<AccountAddress>,
    ///
    pub operator_has_balance: Option<bool>,
}