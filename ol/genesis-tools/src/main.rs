use backup_cli::storage::{FileHandle, FileHandleRef};
use serde::de::DeserializeOwned;
use std::{fs::File};

use std::io::Read;

use libra_types::{
    proof::TransactionInfoWithProof, ledger_info::LedgerInfoWithSignatures,
    account_state_blob::AccountStateBlob
};
use libra_types::{
    access_path::AccessPath,
    account_address::AccountAddress,
    account_config::{
        coin1_tmp_tag, from_currency_code_string, testnet_dd_account_address,
        treasury_compliance_account_address, BalanceResource, COIN1_NAME,
    },
    account_state::AccountState,
    contract_event::ContractEvent,
    on_chain_config,
    on_chain_config::{config_address, ConfigurationResource, OnChainConfig, ValidatorSet},
    proof::SparseMerkleRangeProof,
    transaction::{
        authenticator::AuthenticationKey, ChangeSet, Transaction, Version, WriteSetPayload,
        PRE_GENESIS_VERSION,
    },
    trusted_state::TrustedState,
    validator_signer::ValidatorSigner,
    waypoint::Waypoint,
    write_set::{WriteOp, WriteSetMut},
};
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};
use libra_crypto::HashValue;
use libra_config::utils::get_available_port;

use backup_cli::backup_types::state_snapshot::manifest::StateSnapshotBackup;

use anyhow::{ensure, Result};

use tokio::{
    fs::{OpenOptions},
    io::{AsyncRead}
};

use libradb::LibraDB;
use libra_temppath::TempPath;

use backup_cli::utils::read_record_bytes::ReadRecordBytes;

use tokio::runtime::Runtime;
use backup_service::start_backup_service;

mod generate_genesis;
use libra_genesis_tool::{verify::compute_genesis};
use storage_interface::{DbReader, DbReaderWriter};
use executor::{
    db_bootstrapper::{generate_waypoint, maybe_bootstrap, get_balance},
    Executor,
};
use libra_vm::LibraVM;
// use backup_cli::utils::test_utils::{start_local_backup_service, tmp_db_with_random_content};

fn get_runtime() -> (Runtime, u16) {
    let port = get_available_port();
    let path = TempPath::new();
    let rt = start_backup_service(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), port), Arc::new(LibraDB::new_for_test(&path)));
    (rt, port)
}


async fn open_for_read(
    file_handle: &FileHandleRef,
) -> Result<Box<dyn AsyncRead + Send + Unpin>> {
    let home = dirs::home_dir().unwrap();
    let mut path: String = home.join("libra/ol/fixtures/state-snapshot/194/").into_os_string().into_string().unwrap().to_owned();
    path.push_str(&file_handle.to_string());
    let file = OpenOptions::new().read(true).open(path).await?;
    Ok(Box::new(file))
}

fn read_from_file(path: &str) -> Result<Vec<u8>> {
    let mut data = Vec::<u8>::new();
    let mut f = File::open(path).expect("Unable to open file");
    f.read_to_end(&mut data).expect("Unable to read data");
    // println!("{}", data.len())?;
    Ok(data)
}

fn read_from_json(path: &str) -> Result<StateSnapshotBackup>{
    let config = std::fs::read_to_string(path)?;
    let map: StateSnapshotBackup = serde_json::from_str(&config)?;
    Ok(map)
}

fn load_lcs_file<T: DeserializeOwned>(file_handle: &str) -> Result<T> {
    let x = read_from_file(&file_handle)?;
    Ok(lcs::from_bytes(&x)?)
}

async fn read_account_state_chunk(file_handle: FileHandle) -> Result<Vec<(HashValue, AccountStateBlob)>> {
    let mut file = open_for_read(&file_handle).await?;

    let mut chunk = vec![];

    while let Some(record_bytes) = file.read_record_bytes().await? {
        chunk.push(lcs::from_bytes(&record_bytes)?);
    }
    Ok(chunk)
}

async fn run_impl(manifest: StateSnapshotBackup) -> Result<()>{
    let mut account_state_blobs: Vec<AccountStateBlob> = Vec::new();
    for chunk in manifest.chunks {
        
        let blobs = read_account_state_chunk(chunk.blobs).await?;
        // let proof = load_lcs_file(&chunk.proof)?;
        println!("{:?}", blobs);
        // TODO(Venkat) -> Here's the blob
        // println!("{:?}", proof);
        for (_key, blob) in blobs {
            account_state_blobs.push(blob)
        }

    }


    let genesis = vm_genesis::test_genesis_change_set_and_validators(Some(1));
    let genesis_txn = Transaction::GenesisTransaction(WriteSetPayload::Direct(genesis.0));
    let tmp_dir = TempPath::new();
    let db_rw = DbReaderWriter::new(LibraDB::new_for_test(&tmp_dir));

    // Executor won't be able to boot on empty db due to lack of StartupInfo.
    assert!(db_rw.reader.get_startup_info().unwrap().is_none());

    // Bootstrap empty DB.
    let waypoint = generate_waypoint::<LibraVM>(&db_rw, &genesis_txn).expect("Should not fail.");
    maybe_bootstrap::<LibraVM>(&db_rw, &genesis_txn, waypoint).unwrap();
    let startup_info = db_rw
        .reader
        .get_startup_info()
        .expect("Should not fail.")
        .expect("Should not be None.");
    assert_eq!(
        Waypoint::new_epoch_boundary(startup_info.latest_ledger_info.ledger_info()).unwrap(),
        waypoint
    );
    let (li, epoch_change_proof, _) = db_rw.reader.get_state_proof(waypoint.version()).unwrap();
    let trusted_state = TrustedState::from(waypoint);
    trusted_state
        .verify_and_ratchet(&li, &epoch_change_proof)
        .unwrap();

    // `maybe_bootstrap()` does nothing on non-empty DB.
    assert!(!maybe_bootstrap::<LibraVM>(&db_rw, &genesis_txn, waypoint).unwrap());

    let genesis_txn = generate_genesis::generate_genesis_from_snapshot(&account_state_blobs, &db_rw).unwrap();
    generate_genesis::write_genesis_blob(genesis_txn)?;
    generate_genesis::test_genesis_from_blob(&account_state_blobs, db_rw)?;
    Ok(())
}

fn main() -> Result<()>{
    let home = dirs::home_dir().unwrap();
    
    let path = home.join("libra/ol/fixtures/state-snapshot/194/state_ver_74694920.0889/state.manifest");
    let path2 = home.join("libra/ol/fixtures/state-snapshot/194/state_ver_74694920.0889/state.proof");

    let manifest = read_from_json(&path.into_os_string().into_string().unwrap()).unwrap();

    let (mut rt, _port) = get_runtime();



    let (txn_info_with_proof, li): (TransactionInfoWithProof, LedgerInfoWithSignatures) = 
            load_lcs_file(&path2.into_os_string().into_string().unwrap()).unwrap();



    txn_info_with_proof.verify(li.ledger_info(), manifest.version)?;

    ensure!(
        txn_info_with_proof.transaction_info().state_root_hash() == manifest.root_hash,
        "Root hash mismatch with that in proof. root hash: {}, expected: {}",
        manifest.root_hash,
        txn_info_with_proof.transaction_info().state_root_hash(),
    );

    let future = run_impl(manifest); // Nothing is printed
    rt.block_on(future)?;

    Ok(())
    
}

