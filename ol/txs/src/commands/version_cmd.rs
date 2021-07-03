//! `version` subcommand

#![allow(clippy::never_loop)]
use abscissa_core::{Command, Options, Runnable};
use super::TxsCmd;

/// `version` subcommand
#[derive(Command, Debug, Default, Options)]
pub struct VersionCmd {}

impl Runnable for VersionCmd {
    /// Print version message
    fn run(&self) {
        println!("{} {}", TxsCmd::name(), TxsCmd::version());        
    }
}
