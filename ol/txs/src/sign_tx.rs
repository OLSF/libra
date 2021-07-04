//! `sign tx`

use diem_types::{
  chain_id::ChainId,
  transaction::{helpers::create_user_txn, SignedTransaction, TransactionPayload},
};

use crate::submit_tx::TxParams;
use anyhow::Error;

/// sign a raw transaction script, and return a SignedTransaction
pub fn sign_tx(
  script: TransactionPayload,
  tx_params: &TxParams,
  sequence_number: u64,
  chain_id: ChainId,
) -> Result<SignedTransaction, Error> {
  // sign the transaction script
  create_user_txn(
    &tx_params.keypair,
    script,
    tx_params.signer_address,
    sequence_number,
    tx_params.tx_cost.max_gas_unit_for_tx,
    tx_params.tx_cost.coin_price_per_unit,
    "GAS".parse().unwrap(),
    tx_params.tx_cost.user_tx_timeout as i64, // for compatibility with UTC's timestamp.
    chain_id,
  )
}
