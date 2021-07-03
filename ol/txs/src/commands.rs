//! TxsApp Subcommands
//!
//! This is where you specify the subcommands of your application.
//!
//! The default application comes with two subcommands:
//!
//! - `start`: launches the application
//! - `version`: print application version
//!
//! See the `impl Configurable` below for how to specify the path to the
//! application's configuration file.

mod create_account_cmd;
mod create_validator_cmd;
mod oracle_upgrade_cmd;
mod version_cmd;
pub mod autopay_batch_cmd;
mod demo_cmd;
mod relay_cmd;
mod valset_cmd;
mod autopay_cmd;

use abscissa_core::{Command, Configurable, Help, Options, Runnable};
use ol::commands::CONFIG_FILE;
use crate::config::AppCfg;
use crate::entrypoint;
use self::{
    create_account_cmd::CreateAccountCmd,
    create_validator_cmd::CreateValidatorCmd,
    oracle_upgrade_cmd::OracleUpgradeCmd,
    version_cmd::VersionCmd,
    autopay_batch_cmd::AutopayBatchCmd,
    autopay_cmd::AutopayCmd,
    demo_cmd::DemoCmd,
    relay_cmd::RelayCmd,
    valset_cmd::ValSetCmd,
};
use std::path::PathBuf;


/// TxsApp Subcommands
#[derive(Command, Debug, Options, Runnable)]
pub enum TxsCmd {
    /// The `create-account` subcommand
    #[options(help = "submit tx to create a user account from account.json file")]
    CreateAccount(CreateAccountCmd),

    /// The `create-validator` subcommand
    #[options(help = "submit tx to create a validator from account.json file")]
    CreateValidator(CreateValidatorCmd),

    /// The `oracle-upgrade` subcommand
    #[options(help = "submit an oracle transaction to upgrade stdlib")]
    OracleUpgrade(OracleUpgradeCmd),     

    /// The `autopay` subcommand
    #[options(help = "enable or disable autopay")]
    Autopay(AutopayCmd),

    /// The `autopay-batch` subcommand
    #[options(help = "batch autopay transactions from json file")]
    AutopayBatch(AutopayBatchCmd),   

    // --- End of STDLIB SCRIPT COMMANDS ---

    /// The `help` subcommand
    #[options(help = "get usage information")]
    Help(Help<Self>),

    /// The `version` subcommand
    #[options(help = "display version information")]
    Version(VersionCmd),
    
    /// The `demo` subcommand
    #[options(help = "noop demo transaction, prints `hello world` in move")]
    Demo(DemoCmd),  

     /// The `relay` subcommand
    #[options(help = "submit a saved transaction from file")]
    Relay(RelayCmd),

    /// The `valset` subcommand
    #[options(help = "join or leave the validator universe, i.e. candidate for validator set")]
    ValSet(ValSetCmd),
}

/// This trait allows you to define how application configuration is loaded.
impl Configurable<AppCfg> for TxsCmd {
    /// Location of the configuration file
    fn config_path(&self) -> Option<PathBuf> {
        // Check if the config file exists, and if it does not, ignore it.
        // If you'd like for a missing configuration file to be a hard error
        // instead, always return `Some(CONFIG_FILE)` here.

        let mut config_path = entrypoint::get_node_home();

        config_path.push(CONFIG_FILE);
        if config_path.exists() {
            // println!("initializing from config file: {:?}", config_path);
            Some(config_path)
        } else {
            // println!("config file not yet existing: {:?}", config_path);
            None
        }
    }
}
