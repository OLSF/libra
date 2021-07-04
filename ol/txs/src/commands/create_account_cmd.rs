//! `CreateAccount` subcommand

#![allow(clippy::never_loop)]

use abscissa_core::{Command, Options, Runnable};
use ol_types::config::TxType;
use crate::{entrypoint, submit_tx::{tx_params_wrapper, maybe_submit}};
use diem_types::transaction::TransactionPayload;
use diem_transaction_builder::stdlib as transaction_builder;
use std::{fs, path::PathBuf};

/// `CreateAccount` subcommand
#[derive(Command, Debug, Default, Options)]
pub struct CreateAccountCmd {
    #[options(short = "f", help = "path of account.json")]
    account_json_path: PathBuf,
}

pub fn create_user_account_script_function(account_json_path: &str) -> TransactionPayload {
    let file = fs::File::open(account_json_path)
        .expect("file should open read only");
    let json: serde_json::Value = serde_json::from_reader(file)
        .expect("file should be proper JSON");
    let block = json.get("block_zero")
        .expect("file should have block_zero and preimage key");

    let preimage = block
        .as_object().unwrap()
        .get("preimage").unwrap()
        .as_str().unwrap();
    
    let pre_hex = hex::decode(preimage).unwrap();

    let proof = block
        .as_object().unwrap()
        .get("proof").unwrap()
        .as_str().unwrap();
    
    let proof_hex = hex::decode(proof).unwrap();
    
    transaction_builder::encode_create_acc_user_script_function(pre_hex, proof_hex)    
}

impl Runnable for CreateAccountCmd {    
    fn run(&self) {
        let entry_args = entrypoint::get_args();
        let account_json = self.account_json_path.to_str().unwrap();
        let tx_params = tx_params_wrapper(TxType::Mgmt).unwrap();
        maybe_submit(
          create_user_account_script_function(account_json),
          &tx_params,
          entry_args.no_send,
          entry_args.save_path,
        ).unwrap();
        // match submit_tx(
        //     &tx_params, 
        //     create_user_account_script_function(account_json)
        // ) {
        //     Err(err) => { println!("{:?}", err) }
        //     Ok(res)  => {
        //         eval_tx_status(res);
        //     }
        // }
    }
}