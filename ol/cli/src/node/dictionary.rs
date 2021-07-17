//! dictionary to associate notes to account addresses

use libra_types::account_address::AccountAddress;
use serde::{Deserialize, Serialize};
use std::{path::Path, fs::File};
use super::node::Node;

///
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountDictionary {
    ///
    pub accounts: Vec<AccountDictionaryEntry>,
}

///
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountDictionaryEntry {
  ///
  pub address: AccountAddress,
  ///
  pub note: String,
}

impl Node {
    /// load account dictionary from json file
    pub fn load_account_dictionary(&mut self) -> AccountDictionary {
        let node_home = &self.app_conf.workspace.node_home;
        let dic_path = node_home.join("accounts-dictionary.json");
        match Path::new(&dic_path).exists() {
            true => {
                let file = File::open(dic_path).expect("file should open read only");
                let dict: AccountDictionary = serde_json::from_reader(file).expect("file should be proper JSON");
                dict
            }
            false => AccountDictionary { accounts: vec![] }
        }
    }   
}

impl AccountDictionary {
    /// return a note for the account address
    pub fn get_note_for_address(&self, address: AccountAddress) -> String {
        match self.accounts.iter().find(| entry | entry.address == address ) {
            Some(found) => found.note.clone(),
            None => String::from("?")
        }
    }
}