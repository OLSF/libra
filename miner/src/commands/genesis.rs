//! `start` subcommand - example of how to write a subcommand

use crate::{block::Block, block::ValConfigs, block::build_block, node_keys::NodePubKeys};
use crate::config::MinerConfig;
use crate::prelude::*;
use std::{fs, path::PathBuf};
use std::io::Write;

/// App-local prelude includes `app_reader()`/`app_writer()`/`app_config()`
/// accessors along with logging macros. Customize as you see fit.
use abscissa_core::{config, Command, FrameworkError, Options, Runnable};
use fs::File;
use libra_crypto::ValidCryptoMaterial;

#[derive(Command, Debug, Options)]
pub struct GenesisCmd {
    // Option for setting path for the blocks/proofs that are mined.
    #[options(help = "The home directory where the blocks will be stored")]
    home: PathBuf, 
}

impl Runnable for GenesisCmd {
    /// Start the application.
    fn run(&self) {
        let miner_configs = app_config();
        println!("Enter your 0L mnemonic:");
        let mnemonic_string = rpassword::read_password_from_tty(Some("\u{1F511} ")).unwrap();

        // Create blocks/block_0.json file.
        build_block::mine_genesis(&miner_configs);
        

        // Create val_init.json file.
        let keys = NodePubKeys::new_from_mnemonic(mnemonic_string);
        dbg!(&keys);

        let mut json_path = PathBuf::from(&miner_configs.workspace.miner_home);
        json_path.push("val_init.json");

        let mut block_json = PathBuf::from(&miner_configs.workspace.miner_home);
        block_json.push("blocks/block_0.json");
        println!("load block_0 from {:?}", &block_json);

        let f = File::open(block_json).expect("Could not open block_0 files");
        let block: Block = serde_json::from_reader(f).expect("Can not read block_0.json");

        let val_configs = ValConfigs {
            /// Block zero of the onboarded miner
            block_zero: block,
            consensus_pubkey: keys.consensus_key.to_bytes().to_vec(),
            validator_network_identity_pubkey: keys.validator_network_key.to_bytes().to_vec(),
            validator_network_address: String::from(miner_configs.profile.ip.as_ref().unwrap()),
            full_node_network_identity_pubkey: keys.fullnode_network_key.to_bytes().to_vec(),
            full_node_network_address: String::from(miner_configs.profile.ip.as_ref().unwrap()),
        };

        let mut file = File::create(json_path.as_path()).unwrap();
        let buf = serde_json::to_string(&val_configs ).expect("Config should be export to json");
        file.write(&buf.as_bytes() )
            .expect("Could not write val_init.json");

    }
}

impl config::Override<MinerConfig> for GenesisCmd {
    // Process the given command line options, overriding settings from
    // a configuration file using explicit flags taken from command-line
    // arguments.
    fn override_config(&self, config: MinerConfig) -> Result<MinerConfig, FrameworkError> {
        Ok(config)
    }
}