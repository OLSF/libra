//! `CreateAccount` subcommand

#![allow(clippy::never_loop)]

use crate::{
    entrypoint,
    submit_tx::{maybe_submit, tx_params_wrapper},
};
use abscissa_core::{Command, Options, Runnable};
use diem_transaction_builder::stdlib as transaction_builder;
use ol_types::config::TxType;

/// `CreateAccount` subcommand
#[derive(Command, Debug, Default, Options)]
pub struct ValSetCmd {
    #[options(help = "add node to validator universe, i.e. make a candidate for validator set")]
    join: bool,
    #[options(
        help = "remove node from validator universe, i.e. cease being a candidate for validator set"
    )]
    leave: bool,
    // #[options(help = "unjail the node, add to list, and remove jailed designation")]
    // unjail: bool,
}

impl Runnable for ValSetCmd {
    fn run(&self) {
        let entry_args = entrypoint::get_args();

        let tx_params = tx_params_wrapper(TxType::Cheap).unwrap();
        let script = if *&self.join {
           transaction_builder::encode_join_script_function()
        } else if *&self.leave {
           transaction_builder::encode_leave_script_function()
        } else {
          panic!("need to set --join or --leave flags")
        };

        maybe_submit(
            script,
            // transaction_builder::encode_demo_e2e_script_function(42),
            &tx_params,
            entry_args.no_send,
            entry_args.save_path,
        )
        .unwrap();
    }
}
