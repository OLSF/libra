//! `CreateAccount` subcommand

#![allow(clippy::never_loop)]

use abscissa_core::{Command, Options, Runnable};
use libra_types::transaction::{Script, SignedTransaction};
use crate::{entrypoint, sign_tx::sign_tx, submit_tx::{tx_params_wrapper, batch_wrapper, TxParams}};
use dialoguer::Confirm;
use std::path::PathBuf;
use ol_types::{autopay::PayInstruction, config::{TxType, IS_CI}};

/// command to submit a batch of autopay tx from file
#[derive(Command, Debug, Default, Options)]
pub struct AutopayBatchCmd {
    #[options(short = "f", help = "path of autopay_batch_file.json")]
    autopay_batch_file: PathBuf,
}


impl Runnable for AutopayBatchCmd {
    fn run(&self) {
        // Note: autopay batching needs to have id numbers to each instruction.
        // will not increment automatically, since this can lead to user error.
        let entry_args = entrypoint::get_args();
        let tx_params = tx_params_wrapper(TxType::Cheap).unwrap();

        let epoch = crate::epoch::get_epoch(&tx_params);
        println!("The current epoch is: {}", epoch);
        let instructions = PayInstruction::parse_autopay_instructions(&self.autopay_batch_file, Some(epoch)).unwrap();
        let scripts = process_instructions(instructions, &epoch);
        batch_wrapper(scripts, &tx_params, entry_args.no_send, entry_args.save_path)

    }
}

/// Process autopay instructions into scripts
pub fn process_instructions(instructions: Vec<PayInstruction>, starting_epoch: &u64) -> Vec<Script> {
    // TODO: Check instruction IDs are sequential.
    instructions.into_iter().filter_map(|i| {
        assert!(i.type_move.unwrap() <= 3);

        let warning = if i.type_move.unwrap() == 0 {
          format!(
              "Instruction {uid}: {note}\nSend {percent_balance:.2?}% of your total balance every day {count_epochs} times (until epoch {epoch_ending}) to address: {destination}?",
              uid = &i.uid,
              percent_balance = *&i.value_move.unwrap() as f64 /100f64,
              count_epochs = &i.duration_epochs.unwrap_or_else(|| {
                 &i.end_epoch.unwrap() - starting_epoch 
                }),
              note = &i.note.clone().unwrap(),
              epoch_ending = &i.end_epoch.unwrap(),
              destination = &i.destination,
          )
        } else if i.type_move.unwrap() == 1 {
          format!(
            "Instruction {uid}: {note}\nSend {percent_balance:.2?}% new incoming funds every day {count_epochs} times (until epoch {epoch_ending}) to address: {destination}?",
            uid = &i.uid,
            percent_balance = *&i.value_move.unwrap() as f64 /100f64,
            count_epochs = &i.duration_epochs.unwrap_or_else(|| {
                 &i.end_epoch.unwrap() - starting_epoch 
                }),
            note = &i.note.clone().unwrap(),
            epoch_ending = &i.end_epoch.unwrap(),
            destination = &i.destination,
        )
        } else if i.type_move.unwrap() == 2  {
          format!(
            "Instruction {uid}: {note}\nSend {total_val} every day {count_epochs} times  (until epoch {epoch_ending}) to address: {destination}?",
            uid = &i.uid,
            total_val = *&i.value_move.unwrap(),
            count_epochs = &i.duration_epochs.unwrap_or_else(|| {
              &i.end_epoch.unwrap() - starting_epoch 
            }),
            note = &i.note.clone().unwrap(),
            epoch_ending = &i.end_epoch.unwrap(),
            destination = &i.destination,
        )
        } else {
          format!(
            "Instruction {uid}: {note}\nSend {total_val} once to address: {destination}?",
            uid = &i.uid,
            note = &i.note.clone().unwrap(),
            total_val = *&i.value_move.unwrap(),
            destination = &i.destination,
        )
        };
        println!("{}", &warning);
        // accept if CI mode.
        if *IS_CI { return Some(i) }            
        
        // check the user wants to do this.
        match Confirm::new().with_prompt("").interact().unwrap() {
          true => Some(i),
          _ =>  {
            panic!("Autopay configuration aborted. Check batch configuration file or template");
          }
        }            
    })
    .map(|i| {
      transaction_builder::encode_autopay_create_instruction_script(
        i.uid, 
        i.type_move.unwrap(), 
        i.destination, 
        i.end_epoch.unwrap(), 
        i.value_move.unwrap()
      )
    })
    .collect()
}
 
/// return a vec of signed transactions
pub fn sign_instructions(scripts: Vec<Script>, starting_sequence_num: u64, tx_params: &TxParams) -> Vec<SignedTransaction>{
  scripts.into_iter()
  .enumerate()
  .map(|(i, s)| {
    let seq = i as u64 + starting_sequence_num;
    sign_tx(&s, tx_params, seq, tx_params.chain_id).unwrap()
    })
  .collect()
}

#[test]
fn test_instruction_script_match() {
  use libra_types::account_address::AccountAddress;
  use ol_types::autopay::InstructionType;
  let script = transaction_builder::encode_autopay_create_instruction_script(
    1, 
    0, 
    AccountAddress::ZERO, 
    10, 
    1000);

  let instr = PayInstruction {
      uid: 1,
      type_of: InstructionType::PercentOfBalance,
      destination: AccountAddress::ZERO,
      end_epoch: Some(10),
      duration_epochs: None,
      note: Some("test".to_owned()),
      type_move: Some(0),
      value: 10f64,
      value_move: Some(1000u64),
  };

  instr.check_instruction_safety(script).unwrap();

}