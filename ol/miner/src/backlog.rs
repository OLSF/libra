//! Miner resubmit backlog transactions module
#![forbid(unsafe_code)]

// use abscissa_core::status_info;
// use cli::{diem_client::DiemClient};
// use ol_types::block::Block;
use txs::submit_tx::{TxParams, /*eval_tx_status*/};
// use std::{fs::File, path::PathBuf};
use ol_types::config::AppCfg;
// use crate::commit_proof::commit_proof_tx;
// use std::io::BufReader;
// use crate::block::parse_block_height;

/// Submit a backlog of blocks that may have been mined while network is offline. 
/// Likely not more than 1. 
pub fn process_backlog(_config: &AppCfg, _tx_params: &TxParams, _is_operator: bool) {
    // 0L todo: get_miner_state requires missing struct JsonRpcBatch
    // // Getting remote miner state
    // let mut client = DiemClient::new(tx_params.url.clone(), tx_params.waypoint).unwrap();
    // println!("Fetching remote tower height");
    // let remote_state  = match client.get_miner_state(tx_params.owner_address.clone()) {
    //     Ok( s ) => { match s {
    //         Some(state) => {
    //             state
    //         },
    //         None => {
    //             println!("Info: Received response but no remote state found. Exiting.");
    //             return
    //         }
    //     } },
    //     Err( e) => {
    //         println!("Error fetching remote state: {:?}", e);
    //         return
    //     },
    // };

    // let remote_height = remote_state.verified_tower_height;

    // println!("Remote tower height: {}", remote_height);
    // // Getting local state height
    // let mut blocks_dir = config.workspace.node_home.clone();
    // blocks_dir.push(&config.workspace.block_dir);
    // let (current_block_number, _current_block_path) = parse_block_height(&blocks_dir);

    // println!("Local tower height: {:?}", current_block_number.unwrap());
    // if current_block_number.unwrap() <= remote_height { return };
    // status_info!("Backlog:","resubmitting missing blocks.");

    // let mut i = remote_height + 1;
    // while i <= current_block_number.unwrap() {
    //     let path = PathBuf::from(format!("{}/block_{}.json", blocks_dir.display(), i));
    //     let file = File::open(&path).expect("Could not open block file");
    //     let reader = BufReader::new(file);
    //     let block: Block = serde_json::from_reader(reader).unwrap();
    //     match commit_proof_tx(&tx_params, block.preimage, block.proof, is_operator) {
    //         Ok(res) => {
    //             if eval_tx_status(res).is_err(){
    //                 break;
    //             }
    //         },
    //         Err(err) => println!("Submit backlog failed with: {}", err)
    //     }
    //     i = i + 1;
    // }
}