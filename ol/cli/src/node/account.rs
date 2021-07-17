//! `account`

use crate::node::node::Node;
use anyhow::{Error, Result};
use libra_json_rpc_client::{views::{AccountView, EventView}, AccountAddress};
use libra_types::{account_state::AccountState, event::{EventHandle, EventKey}, transaction::Version};
use resource_viewer::{AnnotatedAccountStateBlob, MoveValueAnnotator, NullStateView};
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom};
use ol_types::{autopay::{AutoPayResource, AutoPayView}, validator_config::{ValidatorConfigResource, ValidatorConfigView}};

#[derive(Clone, Debug, Deserialize, Serialize)]
/// information on the owner account of this node.
pub struct OwnerAccountView {
    /// account address of this node
    address: AccountAddress,
    /// balance of this node
    balance: f64,
    /// if is jailed
    is_in_validator_set: bool,
    /// auto pay
    autopay: Option<AutoPayView>,
    /// operator account
    operator_account: Option<AccountAddress>,
    /// operator balance
    operator_balance: Option<f64>,    
}

impl OwnerAccountView {
    /// create AccountCli
    pub fn new(address: AccountAddress) -> Self {
        OwnerAccountView {
            address,
            balance: 0_f64,
            is_in_validator_set: false,
            autopay: None,
            operator_account: None,
            operator_balance: None,
        }
    }

    /// query if account has auto pay settings, and not empty
    pub fn has_autopay_not_empty(&self) -> bool {
        match &self.autopay {
            Some(autopay) => autopay.payments.len() > 0,
            None => false
        }
    }

    /// query if validator has an operator account
    pub fn has_operator(&self) -> bool {
        self.operator_account.is_some()
    }

    /// query if operator has balance greater than zero
    pub fn has_operator_positive_balance(&self) -> bool {
        match self.operator_balance {
            Some(balance) => balance > 0.0,
            None => false
        }
    }
}

impl Node {
    /// fetch new account info
    pub fn refresh_account_info(&mut self) -> Option<&OwnerAccountView>{
        match self.get_account_view() {
            Some(av) => {
                self.vitals.account_view.balance = get_balance(av);
                self.vitals.account_view.is_in_validator_set = self.is_in_validator_set();
                self.vitals.account_view.autopay = self.get_autopay_view(self.vitals.account_view.address);
                let operator = self.get_validator_operator_account(self.vitals.account_view.address);
                self.vitals.account_view.operator_account = operator;
                if operator.is_some() {
                    self.vitals.account_view.operator_balance = self.get_account_balance(operator.unwrap());
                }
                Some(&self.vitals.account_view)
            }
            None => None
        }
    }

    /// Get the account view struct
    pub fn get_account_view(&mut self) -> Option<AccountView> {
        let account = self.app_conf.profile.account;
        match self.client.get_account(account, true) {
            Ok((account_view, _)) => account_view,
            Err(_) => None
        }
    }

    /// Get account auto pay resource
    pub fn get_autopay_view(&mut self, account: AccountAddress) -> Option<AutoPayView> {
        let state = self.get_account_state(account);
        match state {
            Ok(state) => match state.get_resource::<AutoPayResource>(
                AutoPayResource::resource_path().as_slice()
            ) {
                Ok(Some(res)) => Some(self.enrich_note(res.get_view())),
                Ok(None) => None,
                Err(_) => None
            }
            Err(_) => None
        }
    }

    /// Enrich with notes freom dictionary
    fn enrich_note(&mut self, mut autopay: AutoPayView) -> AutoPayView {
        let dic= self.load_account_dictionary();
        for payment in autopay.payments.iter_mut()  {
            payment.note = Some(dic.get_note_for_address(payment.payee));
        }        
        autopay
    }

    /// Get validator config view
    pub fn get_validator_config(&mut self, address: AccountAddress) -> Option<ValidatorConfigView> {
        let state = self.get_account_state(address);
        match state {
            Ok(state) => match state.get_resource::<ValidatorConfigResource>(
                ValidatorConfigResource::resource_path().as_slice()
            ) {
                Ok(Some(res)) => {
                    let mut view = res.get_view();
                    let operator = view.operator_account;
                    if operator.is_some() {
                        view.operator_has_balance = Some(self.has_positive_balance(operator.unwrap()))
                    }
                    Some(view)
                },
                Ok(None) => None,
                Err(_) => None
            }
            Err(_) => None
        }
    }

    /// Query if valid account has balance greater than zero
    pub fn has_positive_balance(&mut self, address: AccountAddress) -> bool {
        self.get_account_balance(address).unwrap() > 0.0
    }

    /// Get operator account addres from validator
    pub fn get_validator_operator_account(&mut self, address: AccountAddress) -> Option<AccountAddress> {
        match self.get_validator_config(address) {
            Some(config) => config.operator_account,
            None => None
        }
    }

    /// Get account balance
    pub fn get_account_balance(&mut self, address: AccountAddress) -> Option<f64> {
        match self.client.get_account(address, true) {
            Ok((account_view, _)) => Some(get_balance(account_view.unwrap())),
            Err(_) => None
        }
    }

    /// Return a full Move-annotated account resource struct
    pub fn get_annotate_account_blob(
        &mut self,
        account: AccountAddress,
    ) -> Result<(Option<AnnotatedAccountStateBlob>, Version)> {
        let (blob, ver) = self.client.get_account_state_blob(account)?;
        if let Some(account_blob) = blob {
            let state_view = NullStateView::default();
            let annotator = MoveValueAnnotator::new(&state_view);
            let annotate_blob =
                annotator.view_account_state(&AccountState::try_from(&account_blob)?)?;
            Ok((Some(annotate_blob), ver))
        } else {
            Ok((None, ver))
        }
    }
    
    /// get any account state with client
    pub fn get_account_state(&mut self, address: AccountAddress) -> Result<AccountState, Error> {
        let (blob, _ver) = self.client.get_account_state_blob(address)?;
        if let Some(account_blob) = blob {
            Ok(AccountState::try_from(&account_blob).unwrap())
        } else {
            Err(Error::msg("connection to client"))
        }
    }

    /// Get event handles associated with payments
    pub fn get_payment_event_handles(
        &mut self,
        account: AccountAddress,
    ) -> Result<Option<(EventHandle, EventHandle)>, Error> {
        match self.get_account_state(account) {
            Ok(account_state) => {
              let handles = account_state
              .get_account_resource()?
              .map(|resource| {
                (
                    resource.sent_events().clone(),
                    resource.received_events().clone(),
                )
              });
              Ok(handles)
            },
            Err(_) =>  Err(Error::msg("cannot get payment event handles"))
        }
    }

    /// Get events associated with an event handle's key
    pub fn get_events(
        &mut self,
        event_key: &EventKey,
        start: u64,
        limit: u64,
    ) -> Result<Vec<EventView>> {
        let key = hex::encode(event_key.as_bytes());
        
        self.client.get_events(key, start, limit)
    }

    /// get all events associated with an EventHandle
    // change this to async and do paging.
    pub fn get_handle_events(&mut self, event_handle: &EventHandle, seq_start: Option<u64>) -> Result<Vec<EventView>> {
        if event_handle.count() == 0 {
            return Ok(vec![]);
        }
        // TODO: how to get the highest sequence number available in the database.
        self.get_events(
          event_handle.key(), 
          seq_start.unwrap_or(0), 
          event_handle.count()
        )
    }
}

/// get balance from AccountView
fn get_balance(account_view: AccountView) -> f64 {
    for av in account_view.balances.iter() {
        if av.currency == "GAS" {
            return av.amount as f64 / 1_000_000_f64; // apply scaling factor
        }
    }
    0_f64
}