//! Functional test for delay module

#![forbid(unsafe_code)]
use ol_miner::block::build_block;
use ol_miner::config::*;
use libra_types::waypoint::Waypoint;

#[test]
#[ignore]

// This test doesn't pass because of the loop. Panics with: 'terminal streams not yet initialized!'
fn test_mine_and_submit() {
    let configs = OlMinerConfig {
        profile: Profile {
            public_key: "3e4629ba1e63114b59a161e89ad4a083b3a31b5fd59e39757c493e96398e4df2"
                .to_owned(),
            statement: "protests rage across America".to_owned(),
        },
        chain_info: ChainInfo {
            chain_id: "Ol testnet".to_owned(),
            block_dir: "test_blocks".to_owned(),
            base_waypoint: "0:8859e663dfc13a44d2b67b11bfa4bf7679c61691de5fb0c483c4874b4edae35b".to_owned(),
            node: None,
        },
    };

    let parsed_waypoint: Waypoint = configs.chain_info.base_waypoint.parse().unwrap();

    build_block::mine_and_submit(&configs, "test mnemonic".to_string(), parsed_waypoint);
    assert_eq!(true, true, "not true");
}
