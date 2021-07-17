//! `chain_info`
use chrono::Utc;
use libra_json_rpc_client::views::{OracleResourceView};
use libra_types::{
  account_address::AccountAddress, account_state::AccountState, waypoint::Waypoint,
  validators_stats::ValidatorsStatsResource
};
use ol_types::{validator_config::ValidatorConfigView, autopay::AutoPayView};

use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, collections::HashMap};
use super::node::Node;

/// name of chain info key for db
pub const CHAIN_INFO_DB_KEY: &str = "chain_info";
/// name of val info key for db
pub const VAL_INFO_DB_KEY: &str = "val_info";

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
/// ChainInfo struct
pub struct ChainView {
  /// epoch
  pub epoch: u64,
  /// height/version
  pub height: u64,
  /// validator count
  pub validator_count: u64,
  /// total supply of GAS
  pub total_supply: u64,
  /// latest epoch change time
  pub latest_epoch_change_time: u64,
  /// epoch_progress
  pub epoch_progress: f64,
  /// waypoint
  pub waypoint: Option<Waypoint>,
  /// upgrade
  pub upgrade: Option<OracleResourceView>,
  /// validator view
  pub validator_view: Option<Vec<ValidatorView>>,
  /// validators stats
  pub validators_stats: Option<ValidatorsStatsResource>,
  /// audit stats
  pub vals_config_stats: Option<ValsConfigStats>,
  /// autopay payees percentage recurring stats
  pub autopay_watch_list: Option<Vec<PayeeStats>>, 
}

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
/// Validator info struct
pub struct ValidatorView {
  /// account address
  pub account_address: String,
  /// public key
  pub pub_key: String,
  /// voting power
  pub voting_power: u64,
  /// full node ip
  pub full_node_ip: String,
  /// validator ip
  pub validator_ip: String,
  /// tower height
  pub tower_height: u64,
  /// tower epoch
  pub tower_epoch: u64,
  /// proof counts in current epoch
  pub count_proofs_in_epoch: u64,
  /// epoch validating and mining
  pub epochs_validating_and_mining: u64,
  /// contiguous epochs of mining
  pub contiguous_epochs_validating_and_mining: u64,
  /// epoch count since creation
  pub epochs_since_last_account_creation: u64,
  /// total count votes in current epoch
  pub vote_count_in_epoch: u64,
  /// total block propositions in current epoch
  pub prop_count_in_epoch: u64,
  /// validator config in the chain
  pub validator_config: Option<ValidatorConfigView>,
  /// autopay instructions
  pub autopay: Option<AutoPayView>,
}

/// Validators config stats
#[derive(Default, Debug, Deserialize, Serialize, Clone)]
pub struct ValsConfigStats {
  /// total #vals in the stats
  pub total_vals: usize,
  /// total of vals with autopay set
  pub count_vals_with_autopay: u64,
  /// total of vals with operator account
  pub count_vals_with_operator: u64,
  /// total of vals having operator with balance greater than zero
  pub count_positive_balance_operators: u64,
  /// percentage of vals with autopay set
  pub percent_vals_with_autopay: f64,
  /// percentage of vals with operator account
  pub percent_vals_with_operator: f64,
  /// percentage of vals having operator with balance greater than zero
  pub percent_positive_balance_operators: f64,
}

impl Node {
  /// fetch state from system address 0x0
  pub fn refresh_chain_info(&mut self) -> (Option<ChainView>, Option<Vec<ValidatorView>>) {
    // let mut client = client::pick_client();
    let (blob, _version) = match self.client
      .get_account_state_blob(AccountAddress::ZERO) {
      Ok(t)=> t,
      Err(_) => (None, 0),
    };
    let mut cs = ChainView::default();

    // TODO: This is duplicated with check.rs
    let _ = self.client.get_state_proof();
    
    cs.waypoint = self.client.waypoint().ok();

    if let Some(account_blob) = blob {
      let account_state = AccountState::try_from(&account_blob).unwrap();
      let meta = self.client.get_metadata().unwrap();
      cs.epoch = account_state
        .get_configuration_resource()
        .unwrap()
        .unwrap()
        .epoch();

      cs.validator_count = account_state
        .get_validator_set()
        .unwrap()
        .unwrap()
        .payload()
        .len() as u64;

      // Get vals stats
      let validators_stats = account_state
        .get_validators_stats()
        .unwrap()
        .unwrap();

      // Calculate Epoch Progress
      let ts = account_state
        .get_configuration_resource()
        .unwrap()
        .unwrap()
        .last_reconfiguration_time() as i64
        / 1000000;
      let now = Utc::now().timestamp();

      match meta.chain_id {
        // testnet has faster epochs
        4 => cs.epoch_progress = (now - ts) as f64 / 61f64, // 1 minute
        // for main net
        _ => cs.epoch_progress = (now - ts) as f64 / 86401f64, // 24 hours
      }
      if cs.epoch_progress > 1f64 {
        cs.epoch_progress = 0f64;
      };

      if let Some(first) = account_state
        .get_registered_currency_info_resources()
        .unwrap()
        .first()
      {
        cs.total_supply = (first.total_value() / first.scaling_factor() as u128) as u64;
      }

      cs.height = meta.version;

      cs.upgrade = self.client.query_oracle_upgrade().expect("could not get upgrade oracle view");

      let validators: Vec<ValidatorView> = account_state
        .get_validator_set()
        .unwrap()
        .unwrap()
        .payload()
        .iter()
        .map(|v| {
          let full_node_ip = match v.config().fullnode_network_addresses() {
            Ok(ips) => {
              if !ips.is_empty() {
                ips.last().unwrap().to_string()
              } else {
                "--".to_string()
              }
            }
            Err(_) => "--".to_string(),
          };
          let validator_ip = match v.config().validator_network_addresses() {
            Ok(ips) => {
              if !ips.is_empty() {
                ips.get(0).unwrap().seq_num().to_string()
              } else {
                "--".to_string()
              }
            }
            Err(_) => "--".to_string(),
          };
          let ms = self
            .client
            .get_miner_state(v.account_address().clone())
            .unwrap()
            .unwrap();

          let validator_stats = validators_stats.get_validator_current_stats(v.account_address().clone());
          let val_config = self.get_validator_config(v.account_address().clone());
          let autopay = self.get_autopay_view(v.account_address().clone());

          ValidatorView {
            account_address: v.account_address().to_string(),
            voting_power: v.consensus_voting_power(),
            full_node_ip,
            pub_key: v.consensus_public_key().to_string(),
            validator_ip,

            tower_height: ms.verified_tower_height,
            tower_epoch: ms.latest_epoch_mining,

            count_proofs_in_epoch: ms.count_proofs_in_epoch,
            epochs_validating_and_mining: ms.epochs_validating_and_mining,
            contiguous_epochs_validating_and_mining: ms
              .contiguous_epochs_validating_and_mining,
            epochs_since_last_account_creation: ms.epochs_since_last_account_creation,
            vote_count_in_epoch: validator_stats.vote_count,
            prop_count_in_epoch: validator_stats.prop_count,
            validator_config: val_config,
            autopay: autopay,
          }
        })
        .collect();
      
      cs.validator_view = Some(validators.clone());
      cs.validators_stats = Some(validators_stats);
      cs.vals_config_stats = Some(calc_config_stats(cs.validator_view.clone().unwrap()));
      cs.autopay_watch_list = self.get_autopay_watch_list(validators.clone());
            
      self.vitals.chain_view = Some(cs.clone());

      return (Some(cs), Some(validators));
    }

    (None, None)
  }

  /// Get all percentage recurring payees stats
  pub fn get_autopay_watch_list(&mut self, vals: Vec<ValidatorView>) -> Option<Vec<PayeeStats>> {
    let mut payees: HashMap<AccountAddress, PayeeSums> = HashMap::new();
    let mut total: u64 = 0;

    struct PayeeSums {
      pub amount: u64,
      pub payers: u64,
    }

    // iterate over all validators
    for val in vals.iter() {
      if val.autopay.is_some() {
        // iterate over all autopay instructions
        let mut val_payees: HashMap<AccountAddress, u64> = HashMap::new();
        for payment in val.autopay.as_ref().unwrap().payments.iter() {
          if payment.is_percent_of_change() {
            total += payment.amt;
            *val_payees.entry(payment.payee).or_insert(0) += payment.amt;
          }
        }
        // sum payers and amount
        for (payee, amount) in val_payees.iter() {
          let payee_sums = payees.get_mut(&payee);
          match payee_sums {
            Some(p) => {
              p.amount = p.amount + amount;
              p.payers = p.payers + 1;
            },
            None => {
              payees.insert(*payee, PayeeSums { amount: *amount, payers: 1 });    
            }
          }
        }
      }
    }

    
    // collect payees stats
    let dict = self.load_account_dictionary();
    let ret = payees.iter().map(| (payee, stat) | {
      PayeeStats {
        note: dict.get_note_for_address(*payee),
        address: *payee, 
        payers: stat.payers, 
        average_percent: stat.amount as f64 / stat.payers as f64,
        balance: self.get_account_balance(*payee).unwrap(), 
        sum_percentage: stat.amount,
        all_percentage: (stat.amount * 10000) as f64 / total as f64,
      }
    }).collect();
    Some(ret)
  }
}

fn calc_config_stats(vals: Vec<ValidatorView>) -> ValsConfigStats {
  let mut count_autopay = 0;
  let mut count_operators = 0;
  let mut count_positive_balance = 0;

  for val in vals.iter() {
    let config = val.validator_config.clone().unwrap();
    if val.autopay.is_some() && val.autopay.as_ref().unwrap().payments.iter().find(| each | each.is_percent_of_change()).is_some() {
      count_autopay += 1;
    }
    if config.operator_account.is_some() {
      count_operators += 1;
    }
    if config.operator_has_balance.is_some() && config.operator_has_balance.unwrap() {
      count_positive_balance += 1;
    } 
  }
  ValsConfigStats {
    total_vals: vals.len(),
    count_vals_with_autopay: count_autopay,
    count_vals_with_operator: count_operators,
    count_positive_balance_operators: count_positive_balance,
    percent_vals_with_autopay: count_autopay as f64 / vals.len() as f64,
    percent_vals_with_operator: count_operators as f64 / vals.len() as f64,
    percent_positive_balance_operators: count_positive_balance as f64 / vals.len() as f64,
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
///
pub struct PayeeStats {
  ///
  pub address: AccountAddress,
  ///
  pub note: String,
  ///
  pub balance: f64,
  ///
  pub payers: u64,
  ///
  pub average_percent: f64,
  ///
  pub sum_percentage: u64,
  /// 
  pub all_percentage: f64,
}