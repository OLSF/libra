//! `chain_info`

use chrono::Utc;
use diem_json_rpc_client::views::OracleUpgradeStateView;
use diem_types::{
  account_address::AccountAddress, account_state::AccountState, waypoint::Waypoint,
};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
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
  pub upgrade: Option<OracleUpgradeStateView>,
  /// validator view
  pub validator_view: Option<Vec<ValidatorView>>,
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
}

impl Node {
  /// fetch state from system address 0x0
  pub fn refresh_chain_info(&mut self) -> (Option<ChainView>, Option<Vec<ValidatorView>>) {
    // let mut client = client::pick_client();
    let (blob, _version) = match self.client
      .get_account_state_blob(&AccountAddress::ZERO) {
      Ok(t)=> t,
      Err(_) => (None, 0),
    };
    let mut cs = ChainView::default();

    // TODO: This is duplicated with check.rs
    let _ = self.client.update_and_verify_state_proof();
    
    cs.waypoint = self.client.waypoint();

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
      cs.upgrade = self.client.get_oracle_upgrade_state().expect(
        "could not get upgrade oracle view"
      );

      let validators: Vec<ValidatorView> = account_state
        .get_validator_set()
        .unwrap()
        .unwrap()
        .payload()
        .iter()
        .map(|val| {
          let full_node_ip = match val.config().fullnode_network_addresses() {
            Ok(ips) => {
              if !ips.is_empty() {
                ips.last().unwrap().to_string()
              } else {
                "--".to_string()
              }
            }
            Err(_) => "--".to_string(),
          };
          let validator_ip = match val.config().validator_network_addresses() {
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
            .get_miner_state(&val.account_address())
            .unwrap()
            .unwrap();

          ValidatorView {
            account_address: val.account_address().to_string(),
            voting_power: val.consensus_voting_power(),
            full_node_ip,
            pub_key: val.consensus_public_key().to_string(),
            validator_ip,

            tower_height: ms.verified_tower_height,
            tower_epoch: ms.latest_epoch_mining,

            count_proofs_in_epoch: ms.count_proofs_in_epoch,
            epochs_validating_and_mining: ms.epochs_validating_and_mining,
            contiguous_epochs_validating_and_mining: ms
              .contiguous_epochs_validating_and_mining,
            epochs_since_last_account_creation: ms.epochs_since_last_account_creation,
          }
        })
        .collect();
      
      cs.validator_view = Some(validators.clone());

      self.vitals.chain_view = Some(cs.clone());

      return (Some(cs), Some(validators));
    }

    (None, None)
  }
}
// get chain info from cache
// pub fn read_chain_info_cache() -> ChainView {
//   let chain_state = DB_CACHE
//     .get(CHAIN_INFO_DB_KEY.as_bytes())
//     .unwrap()
//     .expect("could not reach chain_info cache");
//   let c: ChainView = serde_json::de::from_slice(&chain_state.as_slice()).unwrap();
//   c
// }

// /// get chain info from cache
// pub fn read_val_info_cache() -> Vec<ValidatorView> {
//   let val_info = DB_CACHE
//     .get(VAL_INFO_DB_KEY.as_bytes())
//     .unwrap()
//     .expect("could not reach chain_info cache");
//   let v: Vec<ValidatorView> = serde_json::de::from_slice(&val_info.as_slice()).unwrap();
//   v
// }
