//! MinerApp Subcommands

mod start_cmd;
mod version_cmd;
mod zero_cmd;

use self::{
    start_cmd::StartCmd,
    version_cmd::VersionCmd,
    zero_cmd::ZeroCmd,
};
use ol_types::config::AppCfg;
use abscissa_core::{
    config::Override, Command, Configurable, FrameworkError, Help, Options, Runnable,
};
use std::path::PathBuf;
use crate::entrypoint;

/// MinerApp Configuration Filename
pub const CONFIG_FILE: &str = "0L.toml";

/// MinerApp Subcommands
#[derive(Command, Debug, Options, Runnable)]
pub enum MinerCmd {
    /// The `help` subcommand
    #[options(help = "get usage information")]
    Help(Help<Self>),

    /// The `genesis` subcommand
    #[options(help = "mine the 0th block of the tower")]
    Zero(ZeroCmd),

    /// The `start` subcommand
    #[options(help = "start mining blocks")]
    Start(StartCmd),

    /// The `version` subcommand
    #[options(help = "display version information")]
    Version(VersionCmd),

}

/// This trait allows you to define how application configuration is loaded.
impl Configurable<AppCfg> for MinerCmd {
    /// Location of the configuration file
    fn config_path(&self) -> Option<PathBuf> {
        // Check if the config file exists, and if it does not, ignore it.
        // If you'd like for a missing configuration file to be a hard error
        // instead, always return `Some(CONFIG_FILE)` here.

        let mut config_path = entrypoint::get_node_home();
        config_path.push(CONFIG_FILE);

        if config_path.exists() {
            println!("initializing miner from config file: {:?}", config_path);
            Some(config_path)
        } else {
            println!("miner config file not existing: {:?}", config_path);
            None
        }
    }

    /// Apply changes to the config after it's been loaded, e.g. overriding
    /// values in a config file using command-line options.
    ///
    /// This can be safely deleted if you don't want to override config
    /// settings from command-line options.
    fn process_config(&self, config: AppCfg) -> Result<AppCfg, FrameworkError> {
        match self {
            MinerCmd::Start(cmd) => cmd.override_config(config),
            _ => Ok(config),
        }
    }
}
