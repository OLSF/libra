//! Toplevel entrypoint command.

use abscissa_core::{
    Command, command::Usage, Config, Configurable, FrameworkError, 
    Options, Runnable    
};
use libra_types::{account_address::AccountAddress, waypoint::Waypoint};
use libra_global_constants::NODE_HOME;
use reqwest::Url;
use std::path::PathBuf;

use crate::commands;

/// Toplevel entrypoint command.
///
/// Handles obtaining toplevel help as well as verbosity settings.
#[derive(Debug, Options)]
pub struct EntryPoint<Cmd>
where
    Cmd: Command + Runnable,
{
    /// Path to the configuration file
    #[options(short = "c", help = "path to configuration file")]
    pub config: Option<PathBuf>,

    /// Obtain help about the current command
    #[options(short = "h", help = "print help message")]
    pub help: bool,

    /// Increase verbosity setting
    #[options(short = "v", help = "be verbose")]
    pub verbose: bool,

    /// Subcommand to execute.
    ///
    /// The `command` option will delegate option parsing to the command type,
    /// starting at the first free argument.
    #[options(command)]
    pub command: Option<Cmd>,

    /// --- Customizing EntryPoint --- ///

    /// Account Address
    #[options(short = "a", help = "account address")]
    pub account: Option<AccountAddress>,

    /// URL to send tx
    #[options(short = "u", help = "URL to send tx")]    
    pub url: Option<Url>,

    /// Use the upstream URL in configs for sending transaction
    #[options(help = "Use the upstream URL in configs for sending transaction")]
    pub use_upstream_url: bool,

    /// Override waypoint to connect to
    #[options(short = "w", help = "waypoint to connect to")]
    pub waypoint: Option<Waypoint>,

        /// Save the tx to file
    #[options(short = "s", help = "save the signed tx to file")]
    pub save_path: Option<PathBuf>,

    /// Only save, don't send transaction
    #[options(short = "n", help = "don't send the transaction, to be used with --save_path")]
    pub no_send: bool,

    /// Swarm path - get tx params from swarm
    #[options(help = "swarm path to override tx params, testing only")]
    pub swarm_path: Option<PathBuf>,

    /// Swarm persona - what fixtures to use
    #[options(help = "use the fixtures of a persona, e.g. alice, eve")]
    pub swarm_persona: Option<String>,

    /// The operator is sending the transaction, used in miner.
    #[options(short = "o", help = "the operator is signing and sending the transaction")]
    pub is_operator: bool,


    

}

impl<Cmd> EntryPoint<Cmd>
where
    Cmd: Command + Runnable,
{
    /// Borrow the underlying command type or print usage info and exit
    fn command(&self) -> &Cmd {
        self.command
            .as_ref()
            .unwrap_or_else(|| Cmd::print_usage_and_exit(&[]))
    }
}

impl<Cmd> Runnable for EntryPoint<Cmd>
where
    Cmd: Command + Runnable,
{
    fn run(&self) {
        self.command().run()
    }
}

impl<Cmd> Command for EntryPoint<Cmd>
where
    Cmd: Command + Runnable,
{
    /// Name of this program as a string
    fn name() -> &'static str {
        Cmd::name()
    }

    /// Description of this program
    fn description() -> &'static str {
        Cmd::description()
    }

    /// Version of this program
    fn version() -> &'static str {
        Cmd::version()
    }

    /// Authors of this program
    fn authors() -> &'static str {
        Cmd::authors()
    }

    /// Get usage information for a particular subcommand (if available)
    fn subcommand_usage(command: &str) -> Option<Usage> {
        Cmd::subcommand_usage(command)
    }
}

impl<Cfg, Cmd> Configurable<Cfg> for EntryPoint<Cmd>
where
    Cmd: Command + Configurable<Cfg> + Runnable,
    Cfg: Config,
{
    /// Path to the command's configuration file
    fn config_path(&self) -> Option<PathBuf> {
        match &self.config {
            // Use explicit `-c`/`--config` argument if passed
            Some(cfg) => Some(cfg.clone()),

            // Otherwise defer to the toplevel command's config path logic
            None => self.command.as_ref().and_then(|cmd| cmd.config_path()),
        }
    }

    /// Process the configuration after it has been loaded, potentially
    /// modifying it or returning an error if options are incompatible
    fn process_config(&self, config: Cfg) -> Result<Cfg, FrameworkError> {
        match &self.command {
            Some(cmd) => cmd.process_config(config),
            None => Ok(config),
        }
    }
}

/// the entry point args
pub type EntryPointTxsCmd = EntryPoint<commands::TxsCmd>;
/// get arguments passed in the entrypoin of this app, not the subcommands
pub fn get_args() -> EntryPointTxsCmd {
  let test: EntryPointTxsCmd = Command::from_env_args();
  test
}

/// returns node_home
/// usually something like "/root/.0L"
/// in case of swarm like "....../swarm_temp/0" for alice
/// in case of swarm like "....../swarm_temp/1" for bob
pub fn get_node_home() -> PathBuf {
    let mut config_path = dirs::home_dir().unwrap();
    config_path.push(NODE_HOME);

    let entry_args = get_args();

    if entry_args.swarm_path.is_some() {
        config_path = PathBuf::from(entry_args.swarm_path.unwrap());
        if entry_args.swarm_persona.is_some() {
            let persona = &entry_args.swarm_persona.unwrap();
            let all_personas = vec!["alice", "bob", "carol", "dave"];
            let index = all_personas.iter().position(|&r| r == persona).unwrap();
            config_path.push(index.to_string());
        } else {
            config_path.push("0"); // default
        }
    }

    return config_path;
}
