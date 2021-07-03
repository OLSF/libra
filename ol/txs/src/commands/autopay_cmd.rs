//! `CreateAccount` subcommand

#![allow(clippy::never_loop)]

use abscissa_core::{Command, Options, Runnable};

use crate::{entrypoint, submit_tx::{ maybe_submit, tx_params_wrapper}};


use ol_types::{config::TxType};

/// command to submit a batch of autopay tx from file
#[derive(Command, Debug, Default, Options)]
pub struct AutopayCmd {
    #[options(help = "enable autopay on account")]
    enable: bool,
    #[options(help = "disable autopay on account")]
    disable: bool,
}


impl Runnable for AutopayCmd {
    fn run(&self) {
        let entry_args = entrypoint::get_args();
        let tx_params = tx_params_wrapper(TxType::Mgmt).unwrap();
        let script = if *&self.enable {
          transaction_builder::encode_autopay_enable_script()
        } else if *&self.disable {
          transaction_builder::encode_autopay_disable_script()
        } else {
          panic!("must choose --enable or --disable");
        };
        maybe_submit(
          script,
          &tx_params,
          entry_args.no_send,
          entry_args.save_path,
        ).unwrap();
            


    }
}
