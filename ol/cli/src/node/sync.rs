//! `sync` subcommand

use super::node::Node;
use crate::{config::AppCfg, node::client::*};
use anyhow::{Error, anyhow, bail};
use backup_cli::utils::backup_service_client::{BackupServiceClient, BackupServiceClientOpt};
use diem_types::waypoint::Waypoint;
use diemdb::backup::backup_handler::DbState;
use tokio::runtime::Runtime;

/// State of the node's sync
#[derive(Debug)]
pub struct SyncState {
  /// if synced
  pub is_synced: bool,
  /// local height of database
  pub sync_height: u64,
  /// remote database height
  pub remote_height: u64,
  /// delay in blocks between remote and local
  pub sync_delay: i64,
} 


impl Default for SyncState {
    fn default() -> Self {
        SyncState {
            is_synced: false,
            sync_height: 0,
            remote_height: 0,
            sync_delay: 0,
        }
    }
}
impl Node {
  /// check if node is synced
  pub fn check_sync(config: &AppCfg, waypoint: Waypoint) -> Result<SyncState, Error> {
    let mut s = SyncState::default();

    if !Node::node_running() {
      bail!("Node is not running. Cannot connect to localhost:8080.");
    }
    
    let remote_client = default_remote_client(config, waypoint).expect(
      "cannot connect to upstream node"
    ).0;
    
    if let Some(local_db) = get_db_state() {
      s.remote_height = remote_client.get_metadata().unwrap().version;
      s.sync_height = local_db.synced_version;
      s.sync_delay = s.remote_height as i64 - s.sync_height as i64;
      s.is_synced = s.sync_delay < 1000;
      return Ok(s)
    }
    Err(anyhow!("Cannot get local db state"))
  }

  /// check if node is synced
  pub fn sync_state(&mut self) -> Result<SyncState, Error> {
    let wp = self.waypoint().expect("Can not update Waypoint");
    Node::check_sync(&self.conf, wp)
  }

//   /// Compare the nodes from toml config.
//   // TODO: Deprecated. Not used for sync.
//   pub fn compare_from_config(config: &AppCfg, waypoint: Waypoint) -> Option<i64> {
//     let local_client = default_local_client(config, waypoint);
//     let remote_client = default_remote_client(config, waypoint);

//     if local_client.is_some() && remote_client.is_some() {
//       return match compare_client_version(
//         &mut local_client.unwrap().0,
//         &mut remote_client.unwrap().0,
//       ) {
//           Ok(delay) => Some(delay),
//           Err(_) => None
//       }
//     }
//     None
//   }
}

// fn compare_client_version(local: &mut DiemClient, remote: &mut DiemClient) -> Result<i64, Error> {
//   let local_meta = local.get_metadata()?;
//   let remote_meta = remote.get_metadata()?;
//   Ok(remote_meta.version as i64 - local_meta.version as i64)
// }

/// get local sync block height
pub fn get_db_state() -> Option<DbState>{
  let bk = BackupServiceClientOpt {
    address: "http://localhost:6186".to_owned(),
  };
  let client = BackupServiceClient::new_with_opt(bk);

  let mut rt = Runtime::new().unwrap();
  rt.block_on(client.get_db_state()).unwrap()
}