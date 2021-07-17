//! `bal` subcommand

use std::process::exit;

use abscissa_core::{Command, Options, Runnable, status_info};
use crate::{
    entrypoint,
    prelude::app_config,
    node::query::QueryType,
    node::client,
    node::node::Node
};

/// `bal` subcommand
///
/// The `Options` proc macro generates an option parser based on the struct
/// definition, and is defined in the `gumdrop` crate. See their documentation
/// for a more comprehensive example:
///
/// <https://docs.rs/gumdrop/>
#[derive(Command, Debug, Default, Options)]
pub struct QueryCmd {
    #[options(short = "b", help = "balance")]
    balance: bool,

    #[options(no_short, help = "blockheight")]
    blockheight: bool,
    
    #[options(help = "sync delay from upstream")]
    sync: bool,
    
    #[options(help = "resources")]
    resources: bool,

    #[options(help = "epoch and waypoint")]
    epoch: bool,

    #[options(help = "get last transactions, defaults to last 100")]
    txs: bool,

    #[options(help = "get last payment events SENT, defaults to last 100")]
    events_sent: bool,
    
    #[options(help = "get last payment events RECEIVED, defaults to last 100")]
    events_received: bool,

    #[options(help = "height to start txs query from, defaults to -100_000 blocks")]
    txs_height: Option<u64>,

    #[options(help = "number of txs to return, defaults to 100 txs")]
    txs_count: Option<u64>,

    #[options(help = "filter by type of transaction, e.g. 'ol_miner_state_commit'")]
    txs_type: Option<String>,

    #[options(help = "move value")]
    move_state: bool,

    #[options(help = "move module name")]
    move_module: Option<String>,

    #[options(help = "move struct name")]
    move_struct: Option<String>,

    #[options(help = "move value name")]
    move_value: Option<String>,
}

impl Runnable for QueryCmd {
    fn run(&self) {
        let args = entrypoint::get_args();
        let is_swarm = *&args.swarm_path.is_some();
        let mut cfg = app_config().clone();
        let account = 
            if args.account.is_some() { args.account.unwrap() }
            else { cfg.profile.account };
            
        let client = client::pick_client(args.swarm_path.clone(), &mut cfg).unwrap_or_else(|e| {
          println!("ERROR: Cannot connect to a client. Message: {}", e);
          exit(1);
        });
        let mut node = Node::new(client, cfg, is_swarm);
        let mut info = String::new();
        let mut display = "";

        if self.balance {
            info = node.query(QueryType::Balance{account});
            display = "BALANCE";
        }
        else if self.blockheight {
            info = node.query(QueryType::BlockHeight);
            display = "BLOCK HEIGHT";
        }
        else if self.sync {
            info = node.query(QueryType::SyncDelay);
            display = "SYNC";
        }
        else if self.resources {
            info = node.query(QueryType::Resources{account});
            display = "RESOURCES";
        }
        else if self.move_state {
            info = node.query(QueryType::MoveValue{
              account,
              module_name: self.move_module.clone().unwrap(),
              struct_name: self.move_struct.clone().unwrap(),
              key_name: self.move_value.clone().unwrap(),
            });
            display = "RESOURCES";
        }
        else if self.epoch {
            info = node.query(QueryType::Epoch);
            display = "EPOCH";
        } else if self.events_received {
            
            info = node.query(QueryType::Events{account, sent_or_received: false, seq_start: self.txs_height});
            display = "EVENTS";
        } else if self.events_sent {
            info = node.query(QueryType::Events{account, sent_or_received: true, seq_start: self.txs_height});
            display = "EVENTS";
        }
        else if self.txs {
            info = node.query(
              QueryType::Txs {
                account,
                txs_height: self.txs_height,
                txs_count: self.txs_count, 
                txs_type: self.txs_type.to_owned(),
              }
            );
            display = "TRANSACTIONS";
        }
        status_info!(display, format!("{}", info));
    }
}
