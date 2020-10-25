// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

use std::{path::PathBuf, fs};

use libra_config::{config::{ NetworkConfig, SecureBackend, DiscoveryMethod, NodeConfig}, config::WaypointConfig, network_id::NetworkId, config::OnDiskStorageConfig};
use libra_crypto::ed25519::Ed25519PublicKey;
use libra_global_constants::OWNER_KEY;
use libra_management::{
    config::ConfigPath, error::Error, secure_backend::ValidatorBackend,
    storage::StorageWrapper as Storage,
};
use structopt::StructOpt;

/// Prints the public information within a store
#[derive(Debug, StructOpt)]
pub struct Files {
    #[structopt(flatten)]
    config: ConfigPath,
    #[structopt(flatten)]
    backend: ValidatorBackend,
    #[structopt(long)]
    namespace: String,
    /// If specified, compares the internal state to that of a
    /// provided genesis. Note, that a waypont might diverge from
    /// the provided genesis after execution has begun.
    #[structopt(long)]
    data_path: PathBuf,
    #[structopt(long, verbatim_doc_comment)]
    genesis_path: Option<PathBuf>,
}

impl Files {
    pub fn execute(self) -> Result<String, Error> {
        let output_dir = self.data_path;

        // Get the Owner and Operator Keys
        let tbd_cfg = self
            .config
            .load()?
            .override_validator_backend(&self.backend.validator_backend)?;
        let validator_storage = tbd_cfg.validator_backend();
        // let mut buffer = String::new();
        let test = get_ed25519_key(&validator_storage,OWNER_KEY).expect("Could not extract OWNER public key");
        dbg!(&test);

        // Get node configs template
        let mut config = NodeConfig::default();
        // config.logger.level = Level::Debug;

        dbg!(&config);
        // Set network configs
        let mut network = NetworkConfig::network_with_id(NetworkId::Validator);
        dbg!(&network);
        
        // if let Some(network) = config.validator_network.as_mut() {
        //     network.listen_address = self.validator_listen_address;
        //     network.advertised_address = self.validator_address;
        //     network.identity = Identity::from_storage(
        //         libra_global_constants::VALIDATOR_NETWORK_KEY.into(),
        //         libra_global_constants::OPERATOR_ACCOUNT.into(),
        //         self.backend.backend.clone().try_into().unwrap(),
        //     );
        //     network.discovery_method = DiscoveryMethod::Gossip;

        //     // network.seed_peers_file = path.join("seed_peers.toml") ;
        // }

        // Get Upstream and Seed Peers info.
        network.discovery_method = DiscoveryMethod::Onchain;
        config.validator_network = Some(network);

        // let upstream = AuthenticationKey::ed25519(&key.public_key).derived_address();
        // config.upstream = UpstreamConfig::default();
        // config.upstream.primary_networks.push(upstream);

        // let peers = Seeds {
        //     genesis_path: path.join("genesis.blob")
        // };
        // for (acc, _network_addresses) in peers.get_seed_info().unwrap().seed_peers.iter() {
        //     if upstream != *acc{
        //     config.upstream.upstream_peers.insert(PeerNetworkId(upstream,acc.clone()));
        //     }
        // }

        // Set Consensus settings
        // config.consensus.safety_rules.backend = self.backend.backend.clone().try_into().unwrap();
        // config.consensus.round_initial_timeout_ms = 1000;

        let mut disk_storage = OnDiskStorageConfig::default();
        disk_storage.set_data_dir(output_dir.clone());
        disk_storage.path = output_dir.clone().join("key_store.json");
        disk_storage.namespace = Some(self.namespace);

        config.base.waypoint = WaypointConfig::FromStorage(SecureBackend::OnDiskStorage(disk_storage.clone()));
        
        config.execution.backend = SecureBackend::OnDiskStorage(disk_storage.clone());
        config.consensus.safety_rules.backend = SecureBackend::OnDiskStorage(disk_storage.clone());

        // safety_rules

        config.execution.genesis_file_location = output_dir.join("genesis.blob");

        // Misc
        config.storage.prune_window=Some(20_000);

        // Write file

        fs::create_dir_all(&output_dir).expect("Unable to create output directory");
        config
            .save(&output_dir.join("node.configs.yaml"))
            .expect("Unable to save node configs");

        Ok("node.configs.yaml created".to_string())
    }
}

fn get_ed25519_key(storage: &Storage, key: &'static str) -> Result<Ed25519PublicKey, Error> {
    storage.ed25519_public_from_private(key)
}

// fn _get_x25519_key(storage: &Storage, buffer: &mut String, key: &'static str) {
//     let value = storage
//         .x25519_public_from_private(key)
//         .map(|v| v.to_string())
//         .unwrap_or_else(|e| e.to_string());
// }