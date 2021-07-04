//! Txs App submit_tx module
#![forbid(unsafe_code)]
use crate::{
    config::AppCfg,
    entrypoint::{self, EntryPointTxsCmd},
    prelude::app_config,
    save_tx::save_tx,
    sign_tx::sign_tx,
};
use abscissa_core::{status_ok, status_warn};
use anyhow::Error;
use cli::{diem_client::DiemClient, AccountData, AccountStatus};
use ol_keys::{wallet, scheme::KeyScheme};
use diem_crypto::{
    ed25519::{Ed25519PrivateKey, Ed25519PublicKey},
    test_utils::KeyPair,
};
use diem_global_constants::OPERATOR_KEY;
use diem_json_rpc_types::views::{TransactionView, VMStatusView};
use diem_secure_storage::{CryptoStorage, NamespacedStorage, OnDiskStorage, Storage};
use diem_types::{account_address::AccountAddress, waypoint::Waypoint};
use diem_types::{
    chain_id::ChainId,
    transaction::{authenticator::AuthenticationKey, SignedTransaction, TransactionPayload},
};

use diem_wallet::WalletLibrary;
use ol_types::{
    self,
    config::{TxCost, TxType},
};
use reqwest::Url;
use std::{
    io::{stdout, Write},
    path::PathBuf,
    thread, time,
};
/// All the parameters needed for a client transaction.
#[derive(Debug)]
pub struct TxParams {
    /// User's 0L authkey used in mining.
    pub auth_key: AuthenticationKey,
    /// Address of the signer of transaction, e.g. owner's operator
    pub signer_address: AccountAddress,
    /// Optional field for Miner, for operator to send owner
    // TODO: refactor so that this is not par of the TxParams type
    pub owner_address: AccountAddress,
    /// Url
    pub url: Url,
    /// waypoint
    pub waypoint: Waypoint,
    /// KeyPair
    pub keypair: KeyPair<Ed25519PrivateKey, Ed25519PublicKey>,
    /// tx cost and timeout info
    pub tx_cost: TxCost,
    // /// User's Maximum gas_units willing to run. Different than coin.
    // pub max_gas_unit_for_tx: u64,
    // /// User's GAS Coin price to submit transaction.
    // pub coin_price_per_unit: u64,
    // /// User's transaction timeout.
    // pub user_tx_timeout: u64, // for compatibility with UTC's timestamp.
    /// Chain id
    pub chain_id: ChainId,
}

// pub struct TxParams {
//     /// Sender's 0L authkey, may be the operator.
//     pub sender_auth_key: AuthenticationKey,
//     /// User's operator sender account if different than the owner account, used to send transactions
//     pub sender_address: AccountAddress,
//     /// User's 0L owner address, where the mining proofs go to.
//     pub owner_address: AccountAddress,
//     /// Url
//     pub url: Url,
//     /// waypoint
//     pub waypoint: Waypoint,
//     /// KeyPair
//     pub keypair: KeyPair<Ed25519PrivateKey, Ed25519PublicKey>,
//     /// User's Maximum gas_units willing to run. Different than coin.
//     pub max_gas_unit_for_tx: u64,
//     /// User's GAS Coin price to submit transaction.
//     pub coin_price_per_unit: u64,
//     /// User's transaction timeout.
//     pub user_tx_timeout: u64, // for compatibility with UTC's timestamp.
// }
/// wrapper which checks entry point arguments before submitting tx, possibly saving the tx script
pub fn maybe_submit(
    script: TransactionPayload,
    tx_params: &TxParams,
    no_send: bool,
    save_path: Option<PathBuf>,
) -> Result<SignedTransaction, Error> {
    let mut client = DiemClient::new(tx_params.url.clone(), tx_params.waypoint).unwrap();

    let (mut account_data, txn) = stage(script, tx_params, &mut client);
    if let Some(path) = save_path {
        // TODO: This will not work with batch operations like autopay_batch, last one will overwrite the file.
        save_tx(txn.clone(), path);
    }

    if no_send {
        return Ok(txn);
    }

    match submit_tx(client, txn.clone(), &mut account_data) {
        Ok(res) => match eval_tx_status(res) {
            Ok(_) => Ok(txn),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}
/// convenience for wrapping multiple transactions
pub fn batch_wrapper(
    batch: Vec<TransactionPayload>,
    tx_params: &TxParams,
    no_send: bool,
    save_path: Option<PathBuf>,
) {
    batch.into_iter().enumerate().for_each(|(i, s)| {
        // TODO: format path for batch scripts

        let new_path = if save_path.is_some() {
            Some(save_path.clone().unwrap().join(i.to_string()))
        } else {
            None
        };

        maybe_submit(s, tx_params, no_send, new_path).unwrap();
        // TODO: handle saving of batches to file.
    });
}

fn stage(
    script: TransactionPayload,
    tx_params: &TxParams,
    client: &mut DiemClient,
) -> (AccountData, SignedTransaction) {
    // let mut client = DiemClient::new(tx_params.url.clone(), tx_params.waypoint).unwrap();

    let chain_id = ChainId::new(client.get_metadata().unwrap().chain_id);
    let account_state = client
        .get_account(&tx_params.signer_address)
        .unwrap();

    let sequence_number = match account_state {
        Some(av) => av.sequence_number,
        None => 0,
    };
    // Sign the transaction script
    let txn = sign_tx(script, tx_params, sequence_number, chain_id).unwrap();

    // Get account_data struct
    let signer_account_data = AccountData {
        address: tx_params.signer_address,
        authentication_key: Some(tx_params.auth_key.to_vec()),
        key_pair: Some(tx_params.keypair.clone()),
        sequence_number,
        status: AccountStatus::Persisted,
    };
    (signer_account_data, txn)
}
/// Submit a transaction to the network.
pub fn submit_tx(
    mut client: DiemClient,
    txn: SignedTransaction,
    mut _signer_account_data: &mut AccountData,
) -> Result<TransactionView, Error> {
    // let mut client = DiemClient::new(tx_params.url.clone(), tx_params.waypoint).unwrap();
    // Submit the transaction with diem_client
    match client.submit_transaction(&txn) {
        Ok(_) => match wait_for_tx(txn.sender(), txn.sequence_number(), &mut client) {
            Some(res) => Ok(res),
            None => Err(Error::msg("No Transaction View returned")),
        },
        Err(err) => Err(err),
    }
}

/// Main get tx params logic based on the design in this URL:
/// https://github.com/OLSF/libra/blob/tx-sender/txs/README.md#txs-logic--usage
pub fn tx_params_wrapper(tx_type: TxType) -> Result<TxParams, Error> {
    let EntryPointTxsCmd {
        url,
        waypoint,
        swarm_path,
        swarm_persona,
        is_operator,
        use_upstream_url,
        ..
    } = entrypoint::get_args();
    let app_config = app_config().clone();
    tx_params(
        app_config,
        url,
        waypoint,
        swarm_path,
        swarm_persona,
        tx_type,
        is_operator,
        use_upstream_url,
    )
}

/// tx_parameters format
pub fn tx_params(
    config: AppCfg,
    url_opt: Option<Url>,
    waypoint: Option<Waypoint>,
    swarm_path: Option<PathBuf>,
    swarm_persona: Option<String>,
    tx_type: TxType,
    is_operator: bool,
    use_upstream_url: bool,
) -> Result<TxParams, Error> {
    let url = if url_opt.is_some() {
        url_opt.unwrap()
    } else {
        config.what_url(use_upstream_url)
    };

    let mut tx_params: TxParams = if swarm_path.is_some() {
        get_tx_params_from_swarm(
            swarm_path.clone().expect("needs a valid swarm temp dir"),
            swarm_persona.expect("need a swarm 'persona' with credentials in fixtures."),
            is_operator,
        )
        .unwrap()
    } else {
        if is_operator {
            get_oper_params( &config, tx_type, url, waypoint)
        } else {
            // Get from 0L.toml e.g. ~/.0L/0L.toml, or use Profile::default()
            get_tx_params_from_toml(config.clone(), tx_type, None, url, waypoint).unwrap()
        }
    };

    if waypoint.is_some() {
        tx_params.waypoint = waypoint.unwrap();
    }

    Ok(tx_params)
}

/// Extract params from a local running swarm
pub fn get_tx_params_from_swarm(
    swarm_path: PathBuf,
    swarm_persona: String,
    is_operator: bool,
) -> Result<TxParams, Error> {
    let (url, waypoint) = ol_types::config::get_swarm_configs(swarm_path);
    let mnem = ol_fixtures::get_persona_mnem(&swarm_persona.as_str());
    let keys = KeyScheme::new_from_mnemonic(mnem);

    let keypair = if is_operator {
        KeyPair::from(keys.child_1_operator.get_private_key())
    } else {
        KeyPair::from(keys.child_0_owner.get_private_key())
    };

    let pubkey = keys.child_0_owner.get_public();
    let auth_key = AuthenticationKey::ed25519(&pubkey);
    let address = auth_key.derived_address();

    let tx_params = TxParams {
        auth_key,
        signer_address: address,
        owner_address: address,
        url,
        waypoint,
        keypair,
        tx_cost: TxCost {
            max_gas_unit_for_tx: 1_000_000,
            coin_price_per_unit: 1, // in micro_gas
            user_tx_timeout: 5_000,
        },

        chain_id: ChainId::new(4),
    };

    println!("Info: Got tx params from swarm");
    Ok(tx_params)
}

/// Form tx parameters struct
pub fn get_oper_params(
    config: &AppCfg,
    tx_type: TxType,
    url: Url,
    wp: Option<Waypoint>,

    // // url_opt overrides all node configs, takes precedence over use_backup_url
    // url_opt: Option<Url>,
    // upstream_url: bool,
) -> TxParams {
    let orig_storage = Storage::OnDiskStorage(OnDiskStorage::new(
        config.workspace.node_home.join("key_store.json").to_owned(),
    ));
    let storage = Storage::NamespacedStorage(NamespacedStorage::new(
        orig_storage,
        format!("{}-oper", &config.profile.auth_key),
    ));
    // export_private_key_for_version
    let privkey = storage
        .export_private_key(OPERATOR_KEY)
        .expect("could not parse operator key in key_store.json");

    let keypair = KeyPair::from(privkey);
    let pubkey = &keypair.public_key; // keys.child_0_owner.get_public();
    let auth_key = AuthenticationKey::ed25519(pubkey);

    let waypoint = wp.unwrap_or_else(|| {
      config
          .get_waypoint(None)
          .clone()
          .expect("could not get waypoint")
    });

    let tx_cost = config.tx_configs.get_cost(tx_type);
    TxParams {
        auth_key,
        signer_address: auth_key.derived_address(),
        owner_address: config.profile.account, // address of sender
        url,
        waypoint,
        keypair,
        tx_cost,
        chain_id: ChainId::new(1),
    }
}

/// Gets transaction params from the 0L project root.
pub fn get_tx_params_from_toml(
    config: AppCfg,
    tx_type: TxType,
    wallet_opt: Option<&WalletLibrary>,
    url: Url,
    wp: Option<Waypoint>,
) -> Result<TxParams, Error> {
    // let url = config.profile.default_node.clone().unwrap();
    let (auth_key, address, wallet) = if let Some(wallet) = wallet_opt {
        wallet::get_account_from_wallet(wallet)
    } else {
        wallet::get_account_from_prompt()
    };

    let waypoint = wp.unwrap_or_else(|| {
        config
            .get_waypoint(None)
            .clone()
            .expect("could not get waypoint")
    });

    let keys = KeyScheme::new_from_mnemonic(wallet.mnemonic());
    let keypair = KeyPair::from(keys.child_0_owner.get_private_key());
    let tx_cost = config.tx_configs.get_cost(tx_type);
    let tx_params = TxParams {
        auth_key,
        signer_address: address,
        owner_address: address,
        url,
        waypoint,
        keypair,
        tx_cost: tx_cost.to_owned(),
        // max_gas_unit_for_tx: config.tx_configs.management_txs.max_gas_unit_for_tx,
        // coin_price_per_unit: config.tx_configs.management_txs.coin_price_per_unit, // in micro_gas
        // user_tx_timeout: config.tx_configs.management_txs.user_tx_timeout,
        chain_id: ChainId::new(1),
    };

    Ok(tx_params)
}

/// Wait for the response from the libra RPC.
pub fn wait_for_tx(
    signer_address: AccountAddress,
    sequence_number: u64,
    client: &mut DiemClient,
) -> Option<TransactionView> {
    println!(
        "\nAwaiting tx status \n\
       Submitted from account: {} with sequence number: {}",
        signer_address, sequence_number
    );

    loop {
        thread::sleep(time::Duration::from_millis(1000));
        // prevent all the logging the client does while
        // it loops through the query.
        stdout().flush().unwrap();

        match &mut client.get_txn_by_acc_seq(&signer_address, sequence_number, false) {
            Ok(Some(txn_view)) => {
                return Some(txn_view.to_owned());
            }
            Err(e) => {
                println!("Response with error: {:?}", e);
            }
            _ => {
                print!(".");
            }
        }
    }
}

/// Evaluate the response of a submitted txs transaction.
pub fn eval_tx_status(result: TransactionView) -> Result<(), Error> {
    match result.vm_status == VMStatusView::Executed {
        true => {
            status_ok!("\nSuccess:", "transaction executed");
            Ok(())
        }
        false => {
            status_warn!("Transaction failed");
            let msg = format!("Rejected with code:{:?}", result.vm_status);
            Err(Error::msg(msg))
        }
    }
}

impl TxParams {
    /// creates params for unit tests
    pub fn test_fixtures() -> TxParams {
        // This mnemonic is hard coded into the swarm configs. see configs/config_builder
        // let mnem_path = format!("./fixtures/mnemonic/{}.mnem", persona);
        let mnemonic = "talent sunset lizard pill fame nuclear spy noodle basket okay critic grow sleep legend hurry pitch blanket clerk impose rough degree sock insane purse".to_string();
        let keys = KeyScheme::new_from_mnemonic(mnemonic);
        let keypair = KeyPair::from(keys.child_0_owner.get_private_key());
        let pubkey = keys.child_0_owner.get_public();
        let signer_auth_key = AuthenticationKey::ed25519(&pubkey);
        let signer_address = signer_auth_key.derived_address();

        let url = Url::parse("http://localhost:8080").unwrap();
        let waypoint: Waypoint =
            "0:732ea2e1c3c5ee892da11abcd1211f22c06b5cf75fd6d47a9492c21dbfc32a46"
                .parse()
                .unwrap();

        TxParams {
            auth_key: signer_auth_key,
            signer_address,
            owner_address: signer_address,
            url,
            waypoint,
            keypair,
            tx_cost: TxCost::new(5_000),
            // max_gas_unit_for_tx: 5_000,
            // coin_price_per_unit: 1, // in micro_gas
            // user_tx_timeout: 5_000,
            chain_id: ChainId::new(4), // swarm/testnet
        }
    }
}
