//! MinerApp Subcommands
mod files_cmd;
mod version_cmd;
mod wizard_fn_cmd;
mod wizard_user_cmd;
mod wizard_val_cmd;
mod keygen_cmd;
mod fix_cmd;

use self::{
    version_cmd::VersionCmd, wizard_fn_cmd::FnWizardCmd, wizard_user_cmd::UserWizardCmd,
    wizard_val_cmd::ValWizardCmd,
    keygen_cmd::KeygenCmd,
    fix_cmd::FixCmd,
};
use abscissa_core::{Command, Configurable, Help, Options, Runnable};
use std::path::PathBuf;
use ol_types::config::AppCfg;
use crate::entrypoint;

/// MinerApp Configuration Filename
pub const CONFIG_FILE: &str = "0L.toml";

/// MinerApp Subcommands
#[derive(Command, Debug, Options, Runnable)]
pub enum WizCmd {
    /// The `help` subcommand
    #[options(help = "get usage information")]
    Help(Help<Self>),

    /// The `version` subcommand
    #[options(help = "display version information")]
    Version(VersionCmd),

    /// The `user` subcommand
    #[options(help = "wizard to create accounts and local configs")]
    User(UserWizardCmd),

    /// The `val` subcommand
    #[options(help = "create all files for validator onboarding")]
    Val(ValWizardCmd),

    /// The `fullnode` subcommand
    #[options(help = "create all files for fullnode config")]
    Fullnode(FnWizardCmd),

    /// The `keygen` subcommand
    #[options(help = "create new account and mnemonic")]
    Keygen(KeygenCmd),

    /// The `fix` subcommand
    #[options(help = "migrate account.json")]
    Fix(FixCmd),
    
}

/// This trait allows you to define how application configuration is loaded.
impl Configurable<AppCfg> for WizCmd {
    /// Location of the configuration file
    fn config_path(&self) -> Option<PathBuf> {
        // Check if the config file exists, and if it does not, ignore it.
        // If you'd like for a missing configuration file to be a hard error
        // instead, always return `Some(CONFIG_FILE)` here.

        // let mut config_path = dirs::home_dir().unwrap();
        // config_path.push(NODE_HOME);
        // config_path.push(CONFIG_FILE);

        let mut config_path = entrypoint::get_node_home();
        config_path.push(CONFIG_FILE);

        if config_path.exists() {
            println!("initializing from config file: {:?}", config_path);
            Some(config_path)
        } else {
            println!("NO config file found at: {:?}", config_path);
            None
        }
    }

    // /// Apply changes to the config after it's been loaded, e.g. overriding
    // /// values in a config file using command-line options.
    // ///
    // /// This can be safely deleted if you don't want to override config
    // /// settings from command-line options.
    // fn process_config(&self, config: OlCliConfig) -> Result<OlCliConfig, FrameworkError> {
    //     match self {
    //         MinerCmd::Start(cmd) => cmd.override_config(config),
    //         _ => Ok(config),
    //     }
    // }
}
