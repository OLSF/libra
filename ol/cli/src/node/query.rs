//! 'query'
use std::collections::BTreeMap;

use hex::decode;
use libra_json_rpc_client::{AccountAddress, views::{BytesView, EventView, TransactionView}};
use move_core_types::{
    identifier::Identifier,
    language_storage::{StructTag, TypeTag},
};
use num_format::{Locale, ToFormattedString};
use resource_viewer::{AnnotatedAccountStateBlob, AnnotatedMoveStruct, AnnotatedMoveValue};
use super::node::Node;

const SCALING_FACTOR: u64 = 1_000_000;

#[derive(Debug)]
/// What query do we want to return
pub enum QueryType {
    /// Account balance
    Balance {
        /// account to query txs of
        account: AccountAddress,
    },
    /// Epoch and waypoint
    Epoch,
    /// Network block height
    BlockHeight,
    /// All account resources
    Resources {
        /// account to query txs of
        account: AccountAddress,
    },
    /// get a move value from account blob
    MoveValue {
      /// account to query txs of
      account: AccountAddress,
      /// move module name
      module_name: String,
      /// move struct name
      struct_name: String,
      /// move key name
      key_name: String,
    },
    /// How far behind the local is from the upstream nodes
    SyncDelay,
    /// Get transaction history
    Txs {
        /// account to query txs of
        account: AccountAddress,
        /// get transactions after this height
        txs_height: Option<u64>,
        /// limit how many txs
        txs_count: Option<u64>,
        /// filter by type
        txs_type: Option<String>,
    },
    /// Get events
    Events {
      /// account to query events
      account: AccountAddress,
      /// switch for sent or received events.
      sent_or_received: bool,
      /// what event sequence number to start querying from, if DB does not have all.
      seq_start: Option<u64>,
    }
}

/// Get data from a client, with a query type. Will connect to local only if in sync.
impl Node {
    /// run a query
    pub fn query(&mut self, query_type: QueryType) -> String {
        use QueryType::*;
        match query_type {
            Balance { account } => {
                // TODO: get scaling factor from chain.
                match self.client.get_account(account, true) {
                    Ok((Some(account_view), _)) => {
                        for av in account_view.balances.iter() {
                            if av.currency == "GAS" {
                                let amount = av.amount / SCALING_FACTOR;
                                return amount.to_formatted_string(&Locale::en);
                            }
                        }
                        return "No GAS found on account".to_owned();
                    }
                    Ok((None, _)) => format!("No account {} found on chain, account", account),
                    Err(e) => format!("Chain query error: {:?}", e),
                }
            }
            BlockHeight => {
                let (chain, _) = self.refresh_chain_info();
                chain.unwrap().height.to_string()
            }
            Epoch => {
                let (chain, _) = self.refresh_chain_info();

                format!(
                    "{} - WAYPOINT: {}",
                    chain.clone().unwrap().epoch.to_string(),
                    &chain.unwrap().waypoint.unwrap().to_string()
                )
            }
            SyncDelay => match self.check_sync() {
                Ok(sync) => format!(
                    "is synced: {}, local height: {}, upstream delay: {}",
                    sync.is_synced, sync.sync_height, sync.sync_delay
                ),
                Err(e) => e.to_string(),
            },
            Resources { account } => {
                // account
                match self.get_annotate_account_blob(account) {
                    Ok((Some(r), _)) => format!("{:#?}", r),
                    Err(e) => format!("Error querying account resource. Message: {:#?}", e),
                    _ => format!("Error, cannot find account state for {:#?}", account),
                }
            },
            MoveValue { account, module_name, struct_name, key_name } => {
                // account
                match self.get_annotate_account_blob(account) {
                    Ok((Some(r), _)) => {
                      let value = find_value_from_state(&r, module_name, struct_name, key_name);
                      format!("{:#?}", value)
                    },
                    Err(e) => format!("Error querying account resource. Message: {:#?}", e),
                    _ => format!("Error, cannot find account state for {:#?}", account),
                }
            },
            Txs {
                account,
                txs_height,
                txs_count,
                txs_type,
            } => {
                let (chain, _) = self.refresh_chain_info();
                let current_height = chain.unwrap().height;
                let query_height = if current_height > 100_000 {
                    current_height - 100_000
                } else {
                    0
                };

                let txs = self
                    .client
                    .get_txn_by_acc_range(
                        account,
                        txs_height.unwrap_or(query_height),
                        txs_count.unwrap_or(100),
                        true,
                    )
                    .unwrap();

                if let Some(t) = txs_type {
                    let filter: Vec<TransactionView> = txs.into_iter()
          .filter(|tv|{
            match &tv.transaction {
                libra_json_rpc_client::views::TransactionDataView::UserTransaction {  script, .. } => {
                  return  script.r#type == t;
                },
                _ => false
            }
          })
          .collect();
                    format!("{:#?}", filter)
                } else {
                    format!("{:#?}", txs)
                }
          },
          Events {
            account,
            sent_or_received,
            seq_start,
          } => {
            // TODO: should borrow and not create a new client.
            let mut print = "Events \n".to_string();
            let handles = self
            .get_payment_event_handles(account)
            .unwrap();

            
            if let Some((sent_handle, received_handle)) = handles {
                  for evt in self.get_handle_events(&sent_handle, seq_start).unwrap() {
                    if sent_or_received { print.push_str(&format_event_view(evt)) }
                  }
                  for evt in self.get_handle_events(&received_handle, seq_start).unwrap() {
                    if !sent_or_received { print.push_str(&format_event_view(evt)) }
                  }
              };
            print
          }
        }
    }
}


fn format_event_view(e: EventView) -> String {

  // TODO: make this more idiomatic.

  use libra_json_rpc_client::views::EventDataView::*;
  let (a, BytesView(s), BytesView(r), BytesView(m), ..) = match e.data {
    ReceivedPayment { amount, sender, receiver, metadata } => {
      (amount, sender, receiver,  metadata)
    },
    SentPayment { amount, receiver, sender, metadata } => {
      (amount, sender, receiver,  metadata)
    },
    _ => { panic!("trying to parse a payment event type, but event is not a ReceivedPayment or SentPayment")}
  };
  let scaled = a.amount / SCALING_FACTOR;
  format!(
    "id: {:?}, sender: {:?}, recipient: {:?}, amount: {:?}, metadata: {:?}\n",
    e.sequence_number,
    s,
    r,
    scaled.to_formatted_string(&Locale::en),
    String::from_utf8_lossy(&decode(m).unwrap()),
  )
}

/// check if the vec of value, is actually of other structs
pub fn is_vec_of_struct(
  move_val: &Vec<AnnotatedMoveValue>,
) -> bool {
    if let Some(e) = move_val.first() {
      match e {
        AnnotatedMoveValue::Struct(_) => return true,
        _ => return false
      }
    }
    false
}

/// get last vec
pub fn get_last_stuct_in_vec(
  move_val: &Vec<AnnotatedMoveValue>,
) -> bool {
    if let Some(e) = move_val.first() {
      match e {
        AnnotatedMoveValue::Struct(_) => return true,
        _ => return false
      }
    }
    false
}

// Ability to query Move types by walking an account blob. This is for structs which we may not have an equivalent type created in rust. For structs the core platform uses we have mappings available e.g. ol/types/src/miner_state.rs. This solves querying for resource structs that may be created by third parties.

/// check if the vec of value, is actually of other structs
pub fn unwrap_val_to_struct(
  move_val: &AnnotatedMoveValue,
) -> Option<&AnnotatedMoveStruct> {
  match move_val {
    AnnotatedMoveValue::Struct(s) => Some(s),
    _ => None
  }
}

/// find the value in a struct
pub fn find_value_in_struct(s: &AnnotatedMoveStruct, key_name: String) -> Option<&AnnotatedMoveValue> {
    match s
        .value
        .iter()
        .find(|v| v.0.clone().into_string() == key_name)
    {
        Some((_, v)) => Some(v),
        None => None,
    }
}
/// finds a value
pub fn find_value_from_state(
    blob: &AnnotatedAccountStateBlob,
    module_name: String,
    struct_name: String,
    key_name: String,
) -> Option<&AnnotatedMoveValue> {
    match blob.0.values().find(|&s| {
        s.type_.module.as_ref().to_string() == module_name
        && s.type_.name.as_ref().to_string() == struct_name
    }) {
        Some(s) => find_value_in_struct(s, key_name),
        None => None,
    }
}


/// test fixtures
pub fn test_fixture_blob() -> AnnotatedAccountStateBlob {
    let mut s = BTreeMap::new();
    let move_struct = test_fixture_struct();
    s.insert(move_struct.type_.clone(), move_struct);
    AnnotatedAccountStateBlob(s)
}

/// stuct fixture
pub fn test_fixture_struct() -> AnnotatedMoveStruct {
    let module_tag = StructTag {
        address: AccountAddress::random(),
        module: Identifier::new("TestModule").unwrap(),
        name: Identifier::new("TestStructName").unwrap(),
        type_params: vec![TypeTag::Bool],
    };

    let key = Identifier::new("test_key").unwrap();
    let value = AnnotatedMoveValue::Bool(true);

    AnnotatedMoveStruct {
        is_resource: true,
        type_: module_tag.clone(),
        value: vec![(key, value)],
    }
}

#[test]
fn test_find_annotated_move_value() {
    let s = test_fixture_blob();
    match find_value_from_state(
        &s,
        "TestModule".to_owned(),
        "TestStructName".to_owned(),
        "test_key".to_owned(),
    ) {
        // NOTE: This is gross, but I don't see a way to use assert_eq! on AnnotatedMoveValue
        Some(v) => {
            match v {
                // TODO: For some reason can't use assert
                AnnotatedMoveValue::Bool(b) => assert!(*b == true),
                _ => panic!("not the right value"),
            }
        }
        None => panic!("not the right value"),
    }
}
