//! OlMiner submit_tx module
#![forbid(unsafe_code)]
use libra_wallet::{Mnemonic, key_factory::Seed, key_factory::KeyFactory, ChildNumber};
use libra_types::{waypoint::Waypoint};

use libra_types::{account_address::AccountAddress, transaction::authenticator::AuthenticationKey};
use libra_crypto::{
    test_utils::KeyPair,
    ed25519::{Ed25519PrivateKey, Ed25519PublicKey}
};
use anyhow::Error;
use cli::{libra_client::LibraClient, AccountData, AccountStatus};
use reqwest::Url;
use abscissa_core::{status_warn, status_ok};
use std::{thread, time, io::{stdout, Write}};

use libra_types::transaction::{Script, TransactionArgument, TransactionPayload};
use libra_types::{transaction::helpers::*};
use crate::{
    config::OlMinerConfig
};
use stdlib::transaction_scripts;
use libra_json_rpc_types::views::TransactionView;
use libra_types::vm_error::StatusCode;

/// All the parameters needed for a client transaction.
pub struct TxParams {
    /// User's 0L authkey used in mining.
    pub auth_key: AuthenticationKey,
    /// User's 0L account used in mining
    pub address: AccountAddress,
    /// Url
    pub url: Url,
    /// waypoint
    pub waypoint: Waypoint,
    /// KeyPair
    pub keypair: KeyPair<Ed25519PrivateKey, Ed25519PublicKey>,
    /// User's Maximum gas_units willing to run. Different than coin. 
    pub max_gas_unit_for_tx: u64,
    /// User's GAS Coin price to submit transaction.
    pub coin_price_per_unit: u64,
    /// User's transaction timeout.
    pub user_tx_timeout: u64, // for compatibility with UTC's timestamp.
}

/// Submit a miner transaction to the network.
pub fn submit_tx(
    tx_params: &TxParams,
    preimage: Vec<u8>,
    proof: Vec<u8>,
    is_onboading: bool,
) -> Result<Option<TransactionView>, Error> {

    // Create a client object
    let mut client = LibraClient::new(tx_params.url.clone(), tx_params.waypoint).unwrap();

    let (account_state,_) = client.get_account_state(tx_params.address.clone(), true).unwrap();
    let sequence_number = match account_state {
        Some(av) => av.sequence_number,
        None => 0,
    };

    let script: Script;
    // Create the unsigned MinerState transaction script
    if !is_onboading {
        script = Script::new(
            transaction_scripts::StdlibScript::MinerState.compiled_bytes().into_vec(),
            vec![],
            vec![
                TransactionArgument::U8Vector(preimage),
                TransactionArgument::U8Vector(proof),
            ],
        );
    } else {
        script = Script::new(
            transaction_scripts::StdlibScript::MinerStateOnboarding.compiled_bytes().into_vec(),
            vec![],
            vec![
                TransactionArgument::U8Vector(preimage),
                TransactionArgument::U8Vector(proof),
            ],
        );
    }

    // sign the transaction script
    let txn = create_user_txn(
        &tx_params.keypair,
        TransactionPayload::Script(script),
        tx_params.address,
        sequence_number,
        tx_params.max_gas_unit_for_tx,
        tx_params.coin_price_per_unit,
        "GAS".parse()?,
        tx_params.user_tx_timeout as i64, // for compatibility with UTC's timestamp.
    )?;

    // get account_data struct
    let mut sender_account_data = AccountData {
        address: tx_params.address,
        authentication_key: Some(tx_params.auth_key.to_vec()),
        key_pair: Some(tx_params.keypair.clone()),
        sequence_number,
        status: AccountStatus::Persisted,
    };
    
    // Submit the transaction with libra_client
    match client.submit_transaction(
        Some(&mut sender_account_data),
        txn
    ){
        Ok(_) => {
            Ok( wait_for_tx(tx_params.address, sequence_number, &mut client) )
        }
        Err(err) => Err(err)
    }

}

/// Wait for the response from the libra RPC.
pub fn wait_for_tx (
    sender_address: AccountAddress,
    sequence_number: u64,
    client: &mut LibraClient) -> Option<TransactionView>{
        println!(
            "Awaiting tx status \nSubmitted from account: {} with sequence number: {}",
            sender_address, sequence_number
        );

        loop {
            thread::sleep(time::Duration::from_millis(1000));
            // prevent all the logging the client does while it loops through the query.
            stdout().flush().unwrap();
            
            match &mut client.get_txn_by_acc_seq(sender_address, sequence_number, false){
                Ok( Some(txn_view)) => {
                    return Some(txn_view.to_owned());
                },
                Err(e) => {
                    println!("Response with error: {:?}", e);

                },
                _ => {
                    print!(".");
                }
            }

        }
}

/// Evaluate the response of a submitted miner transaction.
pub fn eval_tx_status (result: Result<Option<TransactionView>, Error>) -> bool {
    match result {
        Ok(tx_view) => {
            // We receive a tx object.
            match tx_view {
                Some(tx_view) => {
                    if tx_view.vm_status != StatusCode::EXECUTED {
                        status_warn!("Transaction failed");
                        println!("rejected with code:{:?}", tx_view.vm_status);
                        return false
                    } else {
                        status_ok!("Success:", "proof committed to chain");
                        return true
                    }
                }
                //did not receive tx_object but it wasn't in error. This is likely because it's the first sequence number and we are skipping.
                None => {
                    status_warn!("No tx_view returned");
                    return false
                }
            }

        },
        // A tx_view was not returned because of timeout or client connection not established, or other unrelated to vm execution.
        Err(e) => {
            status_warn!("Transaction err: {:?}", e);
            return false
        }

    }
}

/// Form tx parameters struct 
pub fn get_params (
    mnemonic: &str, 
    waypoint: Waypoint,
    config: &OlMinerConfig
) -> TxParams {
    let seed = Seed::new(&Mnemonic::from(&mnemonic).unwrap(), "0L");
    let kf = KeyFactory::new(&seed).unwrap();
    let child_0 = kf.private_child(ChildNumber::new(0)).unwrap();
    let private_key = child_0.export_priv_key();
    let keypair: KeyPair<Ed25519PrivateKey, Ed25519PublicKey> = KeyPair::from(private_key);
    let url_str = config.chain_info.node.as_ref().unwrap();

    TxParams {
        auth_key: child_0.get_authentication_key(),
        address: child_0.get_authentication_key().derived_address(),
        url: Url::parse(url_str).unwrap(),
        waypoint,
        keypair,
        max_gas_unit_for_tx: 1_000_000,
        coin_price_per_unit: 0,
        user_tx_timeout: 5_000,
    }
}

#[test]
fn test_make_params() {
    use crate::config::{
        Workspace,
        Profile,
        ChainInfo
    };

    let mnemonic = "average list time circle item couch resemble tool diamond spot winter pulse cloth laundry slice youth payment cage neutral bike armor balance way ice";
    let waypoint: Waypoint =  "0:3e4629ba1e63114b59a161e89ad4a083b3a31b5fd59e39757c493e96398e4df2".parse().unwrap();
    let configs_fixture = OlMinerConfig {
        workspace: Workspace{
            miner_home: PathBuf::from("."),
            node_home: PathBuf::from("."),
        },
        profile: Profile {
            auth_key: "3e4629ba1e63114b59a161e89ad4a083b3a31b5fd59e39757c493e96398e4df2"
                .to_owned(),
            account: None,
            operator_private_key: None,
            ip: None,
            statement: "Protests rage across the Nation".to_owned(),
        },
        chain_info: ChainInfo {
            chain_id: "0L testnet".to_owned(),
            block_dir: "test_blocks_temp_2".to_owned(),
            base_waypoint: None,
            node: Some("http://localhost:8080".to_string()),
        },

    };

    let p = get_params(&mnemonic, waypoint, &configs_fixture);
    assert_eq!("http://localhost:8080/".to_string(), p.url.to_string());
    // debug!("{:?}", p.url);
    //make_params
}