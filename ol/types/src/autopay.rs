//! autopay view for web monitor

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
use num_format::{Locale, ToFormattedString};

/// Struct that represents a AutoPay resource
#[derive(Debug, Serialize, Deserialize)]
pub struct AutoPayResource {
    ///
    pub payment: Vec<Payment>,
}

/// Struct that represents a view for AutoPay resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoPayView {
    /// 
    pub payments: Vec<PaymentView>,
    ///
    pub recurring_sum: u64,
}

/// Autopay instruction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentView {
    ///
    pub uid: u64,
    ///
    pub in_type: u8,
    ///
    pub type_desc: String,
    ///
    pub payee: AccountAddress,
    ///
    pub end_epoch: u64,
    ///
    pub prev_bal: u64,
    ///
    pub amt: u64,    
    ///
    pub amount: String,
    ///
    pub note: Option<String>,
}

impl PaymentView {
    ///
    pub fn is_percent_of_change(&self) -> bool {
        self.in_type == 1u8
    }
}

/// Autopay instruction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    ///
    pub uid: u64,
    ///
    pub in_type: u8,
    ///
    pub payee: AccountAddress,
    ///
    pub end_epoch: u64,
    ///
    pub prev_bal: u64,
    ///
    pub amt: u64,
}

impl Payment {
    /// get description for in_type value
    pub fn get_type_desc(&self) -> String {
        match self.in_type {
            0 => String::from("percent of balance"),
            1 => String::from("percent of change"),
            2 => String::from("fixed recurring"),
            3 => String::from("fixed once"),
            _ => String::from("type unknown"),
        }
    }

    /// format amount according to type
    pub fn get_amount_formatted(&self) -> String {
        match self.in_type {
            0 | 1   => format!("{:.2}%", self.amt as f64 / 100.00),
            _       => self.amt.to_formatted_string(&Locale::en),
        }
    }
}

impl MoveResource for AutoPayResource {
    const MODULE_NAME: &'static str = "AutoPay2";
    const STRUCT_NAME: &'static str = "Data";
}

impl AutoPayResource {
    ///
    pub fn struct_tag() -> StructTag {
        StructTag {
            address: CORE_CODE_ADDRESS,
            module: AutoPayResource::module_identifier(),
            name: AutoPayResource::struct_identifier(),
            type_params: vec![],
        }
    }
    ///
    pub fn access_path(account: AccountAddress) -> AccessPath {
        let resource_key = ResourceKey::new(
            account,
            AutoPayResource::struct_tag(),
        );
        AccessPath::resource_access_path(&resource_key)
    }
    ///
    pub fn resource_path() -> Vec<u8> {
        AccessPath::resource_access_vec(&AutoPayResource::struct_tag())
    }

    /// 
    pub fn try_from_bytes(bytes: &[u8]) -> Result<Self> {
        lcs::from_bytes(bytes).map_err(Into::into)
    }

    ///
    pub fn get_view(&self) -> AutoPayView {
        let payments = self.payment.iter().map(|each| {
            PaymentView {
                uid: each.uid,
                in_type: each.in_type,
                type_desc: each.get_type_desc(),
                payee: each.payee,
                end_epoch: each.end_epoch,
                prev_bal: each.prev_bal,
                amt: each.amt,
                amount: each.get_amount_formatted(),
                note: None,
            }
        }).collect();

        // sum amount of recurring instructions
        let sum = self.payment.iter()
            .filter(|payment| payment.in_type == 1u8)
            .map(|x| x.amt)
            .sum();

        AutoPayView { 
            payments: payments,
            recurring_sum: sum,
        }
    }
}
