//! `version` subcommand

#![allow(clippy::never_loop)]

use ol_keys::wallet;
use ol_types::block::Block;
use miner::{delay, block::write_genesis};
use ol_types::config::AppCfg;
use abscissa_core::{Command, Options, Runnable};
use std::{path::PathBuf};
use ol_types::account;
/// `user-wizard` subcommand
#[derive(Command, Debug, Default, Options)]
pub struct UserWizardCmd {
    #[options(help = "path to write account manifest")]
    home_path: Option<PathBuf>,
    #[options(help = "path to file to be checked")]
    check: bool,
    #[options(help = "regenerates account manifest from mnemonic")]
    fix: bool,
    #[options(help = "creates a validator account")]
    validator: bool,
    #[options(help = "use an existing block_0.json file and skip mining")]
    block_zero: Option<PathBuf>,
}

impl Runnable for UserWizardCmd {
    /// Print version message
    fn run(&self) {
        // let miner_configs = app_config();
        let home_path = self.home_path.clone().unwrap_or_else(|| PathBuf::from("."));
        if self.check {
            check(home_path);
        } else {
            wizard(home_path, self.fix,  &self.block_zero);
        }
    }
}

fn wizard(path: PathBuf, is_fix: bool, block_zero: &Option<PathBuf>) {
    let mut miner_configs = AppCfg::default();
    
    let (authkey, account, _) = if is_fix { 
        wallet::get_account_from_prompt()
        
    } else {
        wallet::keygen()
    };

    // Where to save block_0
    miner_configs.workspace.node_home = path.clone();
    miner_configs.profile.auth_key = authkey.to_string();
    miner_configs.profile.account = account;

    // Create block zero, if there isn't one.
    let block;
    if let Some(block_path) = block_zero {
        block = Block::parse_block_file(block_path.to_owned());
    } else {
        block = write_genesis(&miner_configs);
    }

    // Create Manifest
    account::UserConfigs::new(block)
    .create_manifest(path);
}

/// Checks the format of the account manifest, including vdf proof
fn check(path: PathBuf) {
    let user_data = account::UserConfigs::get_init_data(&path).expect(&format!("could not parse manifest in {:?}", &path));

    match delay::verify(&user_data.block_zero.preimage, &user_data.block_zero.proof) {
        true => println!("Proof verified in {:?}", &path),
        false => println!("Invalid proof in {:?}", &path)
    }
}
