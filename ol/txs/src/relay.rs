//! `relay`

#![forbid(unsafe_code)]
use std::path::PathBuf;

use crate::{
  save_tx,
  submit_tx::{tx_params_wrapper, wait_for_tx, TxParams},
};
use anyhow::Error;
use cli::libra_client::LibraClient;
use libra_json_rpc_types::views::TransactionView;
use libra_types::transaction::SignedTransaction;
use ol_types::config::TxType;

/// submit a previously signed tx, perhaps to be submitted by a different account than the signer account.
pub fn relay_tx(
  tx_params: &TxParams,
  txn: SignedTransaction,
  // original_signer: AccountAddress,
) -> Result<TransactionView, Error> {
  let mut client = LibraClient::new(tx_params.url.to_owned(), tx_params.waypoint).unwrap();

  let original_signer = txn.sender();
  // let chain_id = ChainId::new(client.get_metadata().unwrap().chain_id);
  let (account_state, _) = client.get_account(original_signer, true).unwrap();

  let original_signer_sequence_number = match account_state {
    Some(av) => av.sequence_number,
    None => 0,
  };
  // Submit the transaction with libra_client
  match client.submit_transaction(None, txn) {
    Ok(_) => {
      match wait_for_tx(
        original_signer,
        original_signer_sequence_number,
        &mut client,
      ) {
        Some(res) => Ok(res),
        None => Err(Error::msg("No Transaction View returned")),
      }
    }
    Err(err) => Err(err),
  }
}

/// submit transaction from a file with batch of signed transactions
pub fn relay_batch(batch_tx: &Vec<SignedTransaction>, tx_params: &TxParams) -> Result<(), Error> {
 
    batch_tx.into_iter()
    .for_each(|tx| {
      match relay_tx(&tx_params, tx.to_owned()) {
          Ok(_) => {},
          Err(e) => {
            let txt = format!("Transaction in batch failed unexpectedly. Other transactions in batch may have executed! Aborting.\n Tx: {:?}\n Error:\n {:?}", tx, e);
            panic!(txt);
          }
          
      }
    });
    Ok(())
}

/// submit transaction from a file with batch of signed transactions
pub fn relay_from_file(path: PathBuf) -> Result<(), Error> {
  //NOTE: Cost does not affect relaying, that's determined in original tx
  let tx_params = tx_params_wrapper(TxType::Mgmt).expect("could not get tx parameters");
  match save_tx::read_tx_from_file(path) {
    Ok(batch) => {
      batch.into_iter().for_each(|tx| {
        relay_tx(&tx_params, tx).unwrap();
      });

      Ok(())
    }
    Err(e) => Err(e),
  }
}
