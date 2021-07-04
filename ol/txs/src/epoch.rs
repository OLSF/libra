//! `epoch`

use std::convert::TryFrom;

use cli::diem_client::DiemClient;
use diem_types::{account_address::AccountAddress, account_state::AccountState};

use crate::submit_tx::TxParams;

/// convenience to get the epoch
pub fn get_epoch(tx_params: &TxParams) -> u64 {
    let client = DiemClient::new(tx_params.url.clone(), tx_params.waypoint).unwrap();

    let (blob, _version) = client.get_account_state_blob(&AccountAddress::ZERO).unwrap();
    if let Some(account_blob) = blob {
        let account_state = AccountState::try_from(&account_blob).unwrap();
        return account_state
            .get_configuration_resource()
            .unwrap()
            .unwrap()
            .epoch();
    }
    0
}