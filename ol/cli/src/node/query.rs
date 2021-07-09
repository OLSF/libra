//! 'query'
use diem_json_rpc_client::{AccountAddress/*, views::TransactionView*/};
use num_format::{Locale, ToFormattedString};
use super::node::Node;

#[derive(Debug)]
/// What query do we want to return
pub enum QueryType {
  /// Account balance
  Balance,
  /// Epoch and waypoint
  Epoch,
  /// Network block height
  BlockHeight,
  /// All account resources
  Resources,
  /// How far behind the local is from the upstream nodes
  SyncDelay,
  /// Get transaction history
  Txs { 
    /// account to query txs of
    account: Option<AccountAddress>,
    /// get transactions after this height
    txs_height: Option<u64>,
    /// limit how many txs
    txs_count: Option<u64>, 
    /// filter by type
    txs_type: Option<String>,
  },
}

/// Get data from a client, with a query type. Will connect to local only if in sync.
impl Node {
  /// run a query
  pub fn get(&mut self, query_type: QueryType) -> String {
    use QueryType::*;
    match query_type {
      Balance => {
        // TODO: get scaling factor from chain.
        let scaling_factor = 1_000_000;
        match self.get_account_view() {
            Some(account_view) => {
              for av in account_view.balances.iter() {
                if av.currency == "GAS" {
                  
                  let amount = av.amount/scaling_factor ;
                  return amount.to_formatted_string(&Locale::en);
                }
              }
            },
            None => {}
        }
        "0".to_string()
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
      SyncDelay => {
       match self.sync_state(){
           Ok(sync) => {
             format!("is synced: {}, local height: {}, upstream delay: {}", sync.is_synced, sync.sync_height, sync.sync_delay)
           },
           Err(e) => e.to_string()
       }
      },
      Resources => {
        let resources = self.get_annotate_account_blob(self.conf.profile.account)
          .unwrap()
          .0
          .unwrap();

        format!("{:#?}", resources).to_string()
      }      
      // 0L todo: https://github.com/OLSF/libra/issues/530
      // Txs { account, txs_height, txs_count, txs_type } => {
      //   let (chain, _) = self.refresh_chain_info();
      //   let current_height = chain.unwrap().height;
      //   let query_height = if current_height > 100_000 { current_height - 100_000 }
      //   else { 0 };

      //   let txs = self.client.get_txn_by_acc_range(
      //     account.unwrap_or(self.conf.profile.account),
      //     txs_height.unwrap_or(query_height),
      //     txs_count.unwrap_or(100), 
      //     true
      //   ).unwrap();

      //   if let Some(t) = txs_type {
      //     let filter: Vec<TransactionView> = txs.into_iter()
      //     .filter(|tv|{
      //       match &tv.transaction {
      //           diem_json_rpc_client::views::TransactionDataView::UserTransaction {  script, .. } => {
      //             return  script.r#type == t;
      //           },
      //           _ => false
      //       }
      //     })
      //     .collect();
      //     format!("{:#?}", filter)
      //   } else {
      //     format!("{:#?}", txs)
      //   }
      // }
      _ => String::new()
    }

  }
}
