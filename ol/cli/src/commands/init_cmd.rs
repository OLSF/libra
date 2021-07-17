//! `init` subcommand

#![allow(clippy::never_loop)]

use crate::{application::app_config, config::AppCfg, entrypoint, migrate};
use abscissa_core::{Command, FrameworkError, Options, Runnable, config};
use anyhow::Error;
use libra_genesis_tool::{init, key};
use libra_types::waypoint::Waypoint;
use ol_keys::{scheme::KeyScheme, wallet};
use libra_json_rpc_client::AccountAddress;
use libra_types::transaction::authenticator::AuthenticationKey;
use std::{fs, path::PathBuf};
use libra_wallet::WalletLibrary;
use url::Url;
use fs_extra::file::{copy, CopyOptions};
use fs_extra::dir::{create};
use crate::checkup;

/// `init` subcommand
#[derive(Command, Debug, Default, Options)]
pub struct InitCmd {
    #[options(help = "home path for app config")]
    path: Option<PathBuf>,
    #[options(help = "An upstream peer to use in 0L.toml")]
    upstream_peer: Option<Url>,
    #[options(help = "Skip app configs")]
    skip_app: bool,
    #[options(help = "Skip validator init")]
    skip_val: bool,
    #[options(help = "Check config file and give hints if something seems wrong")]
    checkup: bool,
    #[options(help = "Fix config file, and migrate any missing fields")]
    fix: bool,
    #[options(help = "Set a waypoint in config files")]
    waypoint: Option<Waypoint>,
    #[options(help = "Path to source code, for devs")]
    source_path: Option<PathBuf>,
}


impl Runnable for InitCmd {
    /// Print version message
    fn run(&self) {
        if *&self.checkup {
          // check 0L.toml file
          checkup::checkup(self.path.to_owned());
          return
        };

       if *&self.fix {
          // fix 0L.toml file
          migrate::migrate(self.path.to_owned());
          return
        };
        
        let entry_args = entrypoint::get_args();
        if let Some(path) = entry_args.swarm_path {
          let swarm_node_home = entrypoint::get_node_home();
          let absolute = fs::canonicalize(path).unwrap();
          initialize_host_swarm(absolute, swarm_node_home, entry_args.swarm_persona, &self.source_path);
          return
        }
        
        let (authkey, account, wallet) = wallet::get_account_from_prompt();
        // start with a default value, or read from file if already initialized
        let mut app_cfg = app_config().to_owned();
        if !self.skip_app { 
          app_cfg = initialize_app_cfg(
            authkey,
            account, 
            &self.upstream_peer,
            &self.path,
            &None, // TODO: probably need an epoch option here.
            &self.waypoint,
            &self.source_path,
          ).unwrap()
        };

        if !self.skip_val {
          initialize_validator(&wallet, &app_cfg, self.waypoint).unwrap() 
        };
    }
}

/// Initializes the necessary 0L config files: 0L.toml
pub fn initialize_app_cfg(
  authkey: AuthenticationKey,
  account: AccountAddress,
  upstream_peer: &Option<Url>,
  path: &Option<PathBuf>,
  epoch_opt: &Option<u64>,
  wp_opt: &Option<Waypoint>,
  source_path: &Option<PathBuf>
) -> Result <AppCfg, Error>{
    let cfg = AppCfg::init_app_configs(
      authkey,
      account,
      upstream_peer,
      path,
      epoch_opt,
      wp_opt,
      source_path);
    Ok(cfg)
}

/// Initializes the necessary 0L config files: 0L.toml and populate blocks directory
/// assumes the libra source is checked out at $HOME/libra
pub fn initialize_host_swarm(swarm_path: PathBuf, node_home: PathBuf, persona: Option<String>, source_path: &Option<PathBuf>) {
    let cfg = AppCfg::init_app_configs_swarm(swarm_path, node_home, source_path.clone());
    if persona.is_some() {
      let source = &cfg.workspace.source_path.unwrap().join("ol/fixtures/blocks/test").join(persona.unwrap()).join("block_0.json");
      let blocks_dir = PathBuf::new().join(&cfg.workspace.node_home).join(&cfg.workspace.block_dir);
      let target_file = PathBuf::new().join(&cfg.workspace.node_home).join(&cfg.workspace.block_dir).join("block_0.json");
      println!("copy first block from {:?} to {:?}", source, target_file);
      match create(blocks_dir, true) {
        Err(why) => println!("create block dir failed: {:?}", why),
        _ => match copy(source, target_file, &CopyOptions::new()) {
          Err(why) => println!("copy block failed: {:?}", why),
          _ => (),
        }
      }
    }
}

/// Initializes the necessary validator config files: genesis.blob, key_store.json
pub fn initialize_validator(wallet: &WalletLibrary, miner_config: &AppCfg, way_opt: Option<Waypoint>) -> Result <(), Error>{
    let home_dir = &miner_config.workspace.node_home;
    let keys = KeyScheme::new(wallet);
    let namespace = miner_config.profile.auth_key.to_owned();
    init::key_store_init(home_dir, &namespace, keys, false);
    key::set_operator_key(home_dir, &namespace);
    key::set_owner_key(home_dir, &namespace);
    if let Some(way) = way_opt {
      key::set_genesis_waypoint(home_dir, &namespace, way);
      key::set_waypoint(home_dir, &namespace, way);
    }
    Ok(())
}

impl config::Override<AppCfg> for InitCmd {
    // Process the given command line options, overriding settings from
    // a configuration file using explicit flags taken from command-line
    // arguments.
    fn override_config(&self, config: AppCfg) -> Result<AppCfg, FrameworkError> {
        Ok(config)
    }
}