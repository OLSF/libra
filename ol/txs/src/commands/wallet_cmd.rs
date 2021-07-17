//! `CreateAccount` subcommand

#![allow(clippy::never_loop)]

use std::process::exit;

use abscissa_core::{Command, Options, Runnable};
use ol_types::config::TxType;
use crate::{entrypoint, submit_tx::{tx_params_wrapper, maybe_submit}};

/// `CreateAccount` subcommand
#[derive(Command, Debug, Default, Options)]
pub struct WalletCmd {
    #[options(short = "c", help = "set this address as a community wallet")]
    community: bool,
    #[options(short = "s", help = "set this address as a slow wallet")]
    slow: bool,
}


impl Runnable for WalletCmd {    
    fn run(&self) {
        let entry_args = entrypoint::get_args();

        let type_int = if self.community {
          1u8
        } else if self.slow {
          0u8
        } else {
          println!("Must pass flag --community or --slow");
          exit(1);
        };

        let tx_params = tx_params_wrapper(TxType::Cheap).unwrap();
        maybe_submit(
          transaction_builder::encode_set_wallet_type_script(type_int),
          &tx_params,
          entry_args.no_send,
          entry_args.save_path
        ).unwrap();
    }
}