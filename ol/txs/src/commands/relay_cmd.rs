//! `CreateAccount` subcommand

#![allow(clippy::never_loop)]

use std::path::PathBuf;

use abscissa_core::{Command, Options, Runnable};
use crate::relay::relay_from_file;

/// `CreateAccount` subcommand
#[derive(Command, Debug, Options)]
pub struct RelayCmd {
  /// File with a signed transaction which the relayer will send
    #[options(short = "f", help = "Path to relay file")]
    relay_file: PathBuf,
}


impl Runnable for RelayCmd {    
    fn run(&self) {
        relay_from_file(self.relay_file.clone()).unwrap();
    }
}