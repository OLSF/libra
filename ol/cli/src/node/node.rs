//! `node` module

use crate::{cache::Vitals, check::items::Items, config::AppCfg, mgmt::management::NodeMode};
use anyhow::Error;
use cli::diem_client::DiemClient;
// use diemdb::DiemDB;
use std::{process::Command, str};
use sysinfo::SystemExt;
use sysinfo::{ProcessExt, ProcessStatus};
use diem_json_rpc_client::views::MinerStateResourceView;
use diem_types::waypoint::Waypoint;
use diem_types::{account_address::AccountAddress, account_state::AccountState};
use super::{account::OwnerAccountView, states::HostState};

/// name of key in kv store for sync
pub const SYNC_KEY: &str = "is_synced";

/// node process name:
pub const NODE_PROCESS: &str = "diem-node";

/// miner process name:
pub const MINER_PROCESS: &str = "miner";

/// Configuration and state of node, account, and host.
pub struct Node {
    /// 0L configs
    pub conf: AppCfg,
    /// diemclient for connecting
    pub client: DiemClient,
    /// vitals
    pub vitals: Vitals,
    // TODO: deduplicate these
    chain_state: Option<AccountState>,
    miner_state: Option<MinerStateResourceView>,
}

impl Node {
    /// Create a instance of Check
    pub fn new(client: DiemClient, conf: AppCfg) -> Self {
        return Self {
            client,
            conf: conf.clone(),
            vitals: Vitals {
                host_state: HostState::new(),
                account_view: OwnerAccountView::new(conf.profile.account),
                chain_view: None,
                items: Items::new(false),
                node_proc: None,
                miner_proc: None,
                monitor_proc: None,
            },
            miner_state: None,
            chain_state: None,
        };
    }

    /// refresh all checks
    pub fn refresh_checks(&mut self) -> &mut Self {
        self.vitals.items.configs_exist = self.configs_exist();
        self.vitals.items.db_files_exist = self.db_files_exist();
        self.vitals.items.db_restored = self.db_bootstrapped();
        self.vitals.items.web_running = Node::is_web_monitor_serving();
        self.vitals.items.node_mode = Node::what_node_mode().ok();
        self.vitals.items.node_running = Node::node_running();
        self.vitals.items.miner_running = Node::miner_running();
        self.vitals.items.account_created = self.accounts_exist_on_chain();
        // TODO: make SyncState an item, so we don't need to assign.
        // affects web-monitor structs
        if let Ok(s) = self.sync_state() {
          self.vitals.items.is_synced = s.is_synced;
          self.vitals.items.sync_delay = s.sync_delay;
          self.vitals.items.sync_height = s.sync_height
        } else {
          self.vitals.items.is_synced = false;
          self.vitals.items.sync_delay = 404;
          self.vitals.items.sync_height = 404;
        }
        self.vitals.items.validator_set = self.is_in_validator_set();
        self
    }

    /// Fetch chain state from the upstream node
    pub fn refresh_onchain_state(&mut self) -> &mut Self {
        self.chain_state = match self.get_account_state(AccountAddress::ZERO) {
            Ok(account_state) => Some(account_state),
            Err(_) => None,
        };
        self.miner_state = match self.client.get_miner_state(&self.conf.profile.account) {
            Ok(state) => state,
            _ => None,
        };
        self
    }

    /// return tower height on chain
    pub fn tower_height_on_chain(&self) -> u64 {
        match &self.miner_state {
            Some(s) => s.verified_tower_height,
            None => 0,
        }
    }

    /// return tower height on chain
    pub fn mining_epoch_on_chain(&self) -> u64 {
        match &self.miner_state {
            Some(s) => s.latest_epoch_mining,
            None => 0,
        }
    }
    /// validator sets
    pub fn validator_set_count(&self) -> usize {
        match &self.chain_state {
            Some(s) => s.get_validator_set().unwrap().unwrap().payload().len(),
            None => 0,
        }
    }

    /// Current monitor account
    pub fn account(&self) -> Vec<u8> {
        self.conf.profile.account.to_vec()
    }

    /// Get waypoint from client
    pub fn waypoint(&mut self) -> Option<Waypoint> {        
        match self.client.update_and_verify_state_proof() {
            Ok(_t) => self.client.waypoint(),
            Err(_) => self.conf.get_waypoint(None),
        }        
    }

    /// is validator jailed
    pub fn is_jailed() -> bool {
        unimplemented!("Don't know how to implement")
    }

    /// Is current account in validator set
    pub fn is_in_validator_set(&self) -> bool {
        match &self.chain_state {
            Some(s) => {
                for v in s.get_validator_set().unwrap().unwrap().payload().iter() {
                    if v.account_address().to_vec() == self.conf.profile.account.to_vec() {
                        return true;
                    }
                }
                false
            }
            None => {
                //println!("No chain state retrieved");
                false
            }
        }
    }

    /// nothing is configured yet, empty box
    pub fn configs_exist(&mut self) -> bool {
        // check to see no files are present
        let home_path = self.conf.workspace.node_home.clone();

        let c_exist = home_path.join("blocks/block_0.json").exists()
            && home_path.join("validator.node.yaml").exists()
            && home_path.join("key_store.json").exists();
        c_exist
    }

    /// the owner and operator accounts exist on chain
    pub fn accounts_exist_on_chain(&mut self) -> bool {
        let addr = self.conf.profile.account;
        // dbg!(&addr);

        let account = self.client.get_account(&addr);
        match account {
            Ok(opt) => match opt {
                Some(_) => true,
                None => false,
            },
            Err(_) => false,
        }
    }

    /// database is initialized, Please do NOT invoke this function frequently
    pub fn db_bootstrapped(&mut self) -> bool {
        let _file = self.conf.workspace.db_path.clone();

        // 0L todo: New DiemDB::open() requires RocksDB options
        // https://github.com/OLSF/libra/issues/530
        //
        // if file.exists() {
        //     // When not committing, we open the DB as secondary so the tool 
        //     // is usable along side a running node on the same DB. 
        //     // Using a TempPath since it won't run for long.
        //     match DiemDB::open(file, true, None) {
        //         Ok(db) => {
        //             return db.get_latest_version().is_ok();
        //         }
        //         Err(_e) => (),
        //     }
        // }
        panic!("Error: Incomplete function"); // Remove this after fixing this fn

        return false;
    }

    /// database is initialized, Please do NOT invoke this function frequently
    pub fn db_files_exist(&mut self) -> bool {
        // check to see no files are present
        let db_path = self.conf.workspace.db_path.clone().join("diemdb");
        db_path.exists()
    }

    /// Check if node is running
    pub fn node_running() -> bool {
        Node::check_process(NODE_PROCESS) | Node::check_systemd(NODE_PROCESS)
    }

    /// Check if miner is running
    pub fn miner_running() -> bool {
        Node::check_process(MINER_PROCESS) | Node::check_systemd(MINER_PROCESS)
    }

    /// Check if miner is running
    pub fn pilot_running() -> bool {
        let mut system = sysinfo::System::new_all();
        system.refresh_all();

        let all_p = system.get_process_by_name("ol");
        // dbg!(&all_p);
        let process = all_p
            .into_iter()
            .filter(|i| match i.status() {
                ProcessStatus::Run => true,
                ProcessStatus::Sleep => true,
                _ => false,
            })
            .find(|i| !i.cmd().is_empty());

        process.unwrap()
            .cmd()
            .into_iter()
            .find(|s| s.contains(&"pilot".to_owned()))
            .is_some()
    }

    fn check_process(process_str: &str) -> bool {
        // get processes from sysinfo
        let mut system = sysinfo::System::new_all();
        system.refresh_all();
        for (_, process) in system.get_processes() {
            if process.name() == process_str {
                // TODO: doesn't always catch `miner` running, see get by name below.
                return true;
            }
        }
        // aldo try by name (yield different results), most reliable.
        let p = system.get_process_by_name(process_str);
        !p.is_empty()
    }
    /// check what mode the node is running in
    pub fn what_node_mode() -> Result<NodeMode, Error> {
        // check systemd first
        if Node::node_running() {
            let output = Command::new("service")
                .args(&["diem-node", "status"])
                .output();
            match output {
                Ok(out) => {
                    let text = str::from_utf8(&out.stdout.as_slice()).unwrap();
                    if text.contains("validator") {
                        return Ok(NodeMode::Validator);
                    }
                    if text.contains("fullnode") {
                        return Ok(NodeMode::Fullnode);
                    }
                }
                Err(e) => return Err(Error::from(e)),
            }

            // check as parent process
            let mut system = sysinfo::System::new_all();
            system.refresh_all();
            let all_p = system.get_process_by_name(NODE_PROCESS);
            // dbg!(&all_p);
            let process = all_p
                .into_iter()
                .filter(|i| match i.status() {
                    ProcessStatus::Run => true,
                    ProcessStatus::Sleep => true,
                    _ => false,
                })
                .find(|i| !i.cmd().is_empty());

            if let Some(p) = process {
                let is_val = p
                    .cmd()
                    .into_iter()
                    .find(|s| s.contains(&"validator".to_owned()))
                    .is_some();
                if is_val {
                    return Ok(NodeMode::Validator);
                }

                let is_fn = p
                    .cmd()
                    .into_iter()
                    .find(|s| s.contains(&"fullnode".to_owned()))
                    .is_some();
                if is_fn {
                    return Ok(NodeMode::Fullnode);
                }

                let is_fn = p
                    .cmd()
                    .into_iter()
                    .find(|s| s.contains(&"swarm".to_owned()))
                    .is_some();
                if is_fn {
                    return Ok(NodeMode::Validator);
                }
            }
        }
        Err(Error::msg("node is not running"))
    }

    /// is web monitor serving on 3030
    pub fn is_web_monitor_serving() -> bool {
        port_scanner::scan_port(3030)
    }

    fn check_systemd(process_name: &str) -> bool {
        let out = Command::new("systemctl")
            .arg("is-active")
            .arg("--quiet")
            .arg(process_name)
            .output();
        match out {
            Ok(o) => o.status.code().unwrap() == 0,
            Err(_) => false,
        }
    }
}
