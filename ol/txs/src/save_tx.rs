//! `save tx`
use anyhow::Error;
use diem_types::transaction::SignedTransaction;
use std::{
  fs::{self, File},
  io::{BufReader, Write},
  path::PathBuf,
};

/// Save signed transaction to file
pub fn save_tx(txn: SignedTransaction, path: PathBuf) {
  let vec = vec![txn];
  save_batch_tx(vec, path);
}

/// TODO: save a batch of txs in one file
pub fn save_batch_tx(vec_tx: Vec<SignedTransaction>, path: PathBuf) {
  let mut file = fs::File::create(path).unwrap();
  let ser = serde_json::to_vec(&vec_tx).expect("could not serialize tx to json");
  file.write_all(&ser).expect("Could not write json");
}

/// deserializes the SignedTransaction from json file
pub fn read_tx_from_file(path: PathBuf) -> Result<Vec<SignedTransaction>, Error> {
  // Open the file in read-only mode with buffer.
  let file = File::open(path)?;
  let reader = BufReader::new(file);
  let tx: Vec<SignedTransaction> = serde_json::from_reader(reader)?;
  Ok(tx)
}

#[test]
fn test_sign_tx() {
  use crate::sign_tx::sign_tx;
  use crate::submit_tx::TxParams;
  use diem_types::{account_address::AccountAddress, chain_id::ChainId};

  let script = transaction_builder::encode_demo_e2e_script_function(42);

  let signed = sign_tx(script, &TxParams::test_fixtures(), 1, ChainId::new(1)).unwrap();
  assert_eq!(
    signed.sender(),
    "4C613C2F4B1E67CA8D98A542EE3F59F5"
      .parse::<AccountAddress>()
      .unwrap()
  );
}

#[test]
fn test_save_tx() {

  use crate::sign_tx::sign_tx;
  use crate::submit_tx::TxParams;
  use diem_types::{chain_id::ChainId, account_address::AccountAddress};

  let script = transaction_builder::encode_demo_e2e_script_function(42);
  let test_path = PathBuf::from("./signed_tx.json");
  let txn = sign_tx(script, &TxParams::test_fixtures(), 0, ChainId::new(1)).unwrap();
  save_tx(txn, test_path.clone());

  let deserialized = read_tx_from_file(test_path.clone()).unwrap().pop().unwrap();

  assert_eq!(
    deserialized.sender(),
    "4C613C2F4B1E67CA8D98A542EE3F59F5"
      .parse::<AccountAddress>()
      .unwrap()
  );
  fs::remove_file(test_path).unwrap();
}
