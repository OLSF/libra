//! Proof block datastructure

use ol_types::config::AppCfg;
use crate::{
    delay::*,
    error::{Error, ErrorKind},
    prelude::*,
    commit_proof::commit_proof_tx,
};
use byteorder::{LittleEndian, WriteBytesExt};
use glob::glob;
use hex::decode;
use diem_crypto::hash::HashValue;
use ol_types::block::Block;
use txs::submit_tx::{TxParams, eval_tx_status};
use std::{
    fs,
    io::{BufReader, Write},
    path::PathBuf,
    time::Instant,
};

/// writes a JSON file with the vdf proof, ordered by a blockheight
pub fn mine_genesis(config: &AppCfg) -> Block {
    println!("Mining Genesis Proof");
    let preimage = genesis_preimage(&config);
    let now = Instant::now();
    let proof = do_delay(&preimage);
    let elapsed_secs = now.elapsed().as_secs();
    println!("Delay: {:?} seconds", elapsed_secs);
    let block = Block {
        height: 0u64,
        elapsed_secs,
        preimage,
        proof,
    };

    block
}

/// Mines genesis and writes the file
pub fn write_genesis(config: &AppCfg) -> Block {
    let block = mine_genesis(config);
    //TODO: check for overwriting file...
    write_json(&block, &config.get_block_dir());
    println!(
        "block zero proof mined, file saved to: {:?}",
        &config.get_block_dir().join("block_0.json")
    );
    block
}
/// Mine one block
pub fn mine_once(config: &AppCfg) -> Result<Block, Error> {
    let (_current_block_number, current_block_path) = parse_block_height(&config.get_block_dir());
    // If there are files in path, continue mining.
    if let Some(max_block_path) = current_block_path {
        // current_block_path is Option type, check if destructures to Some.
        let block_file =
            fs::read_to_string(max_block_path).expect("Could not read latest block file in path");

        let latest_block: Block =
            serde_json::from_str(&block_file).expect("could not deserialize latest block");

        let preimage = HashValue::sha3_256_of(&latest_block.proof).to_vec();
        // Otherwise this is the first time the app is run, and it needs a genesis preimage, which comes from configs.
        let height = latest_block.height + 1;
        // TODO: cleanup this duplication with mine_genesis_once?

        let now = Instant::now();
        let data = do_delay(&preimage);
        let elapsed_secs = now.elapsed().as_secs();
        println!("Delay: {:?} seconds", elapsed_secs);

        let block = Block {
            height,
            elapsed_secs,
            preimage,
            proof: data.clone(),
        };

        write_json(&block, &config.get_block_dir());
        Ok(block)
    // Err(ErrorKind::Io.context(format!("submit_vdf_proof_tx_to_network {:?}", block_dir)).into())
    } else {
        return Err(ErrorKind::Io
            .context(format!("No files found in {:?}", &config.get_block_dir()))
            .into());
    }
}

/// Write block to file
pub fn mine_and_submit(
    config: &AppCfg,
    tx_params: TxParams,
    is_operator: bool,
) -> Result<(), Error> {
    // get the location of this miner's blocks
    let mut blocks_dir = config.workspace.node_home.clone();
    blocks_dir.push(&config.workspace.block_dir);
    let (current_block_number, _current_block_path) = parse_block_height(&blocks_dir);

    // If there are NO files in path, mine the genesis proof.
    if current_block_number.is_none() {
        status_err!("Genesis block_0.json not found. Exiting.");
        std::process::exit(0);
    } else {
        // mine continuously from the last block in the file systems
        let mut mining_height = current_block_number.unwrap() + 1;
        loop {
            status_info!(format!("Block {}", mining_height), "Mining VDF Proof");

            let block = mine_once(&config)?;
            status_info!(
                "Proof mined:",
                format!("block_{}.json created.", block.height.to_string())
            );

            if let Some(ref _node) = config.profile.default_node {
                match commit_proof_tx(&tx_params, block.preimage, block.proof, is_operator) {
                    Ok(tx_view) => match eval_tx_status(tx_view) {
                        Ok(_) => status_ok!("Success:", "Proof committed to chain"),
                        Err(_) => status_err!("Miner transaction rejected"),
                    },
                    Err(err) => status_err!("Miner transaction rejected: {}", err),
                }
            } else {
                return Err(ErrorKind::Config
                    .context("No Node for submitting transactions")
                    .into());
            }

            mining_height = block.height + 1;
        }
    }
}

fn write_json(block: &Block, blocks_dir: &PathBuf) {
    if !&blocks_dir.exists() {
        // first run, create the directory if there is none, or if the user changed the configs.
        // note: user may have blocks but they are in a different directory than what miner.toml says.
        fs::create_dir(&blocks_dir).unwrap();
    };
    // Write the file.
    let mut latest_block_path = blocks_dir.clone();
    latest_block_path.push(format!("block_{}.json", block.height));
    //println!("{:?}", &latest_block_path);
    let mut file = fs::File::create(&latest_block_path).unwrap();
    file.write_all(serde_json::to_string(&block).unwrap().as_bytes())
        .expect("Could not write block");
}

/// parse the existing blocks in the miner's path. This function receives any path. Note: the path is configured in miner.toml which abscissa Configurable parses, see commands.rs.
pub fn parse_block_height(blocks_dir: &PathBuf) -> (Option<u64>, Option<PathBuf>) {
    let mut max_block: Option<u64> = None;
    let mut max_block_path = None;

    // iterate through all json files in the directory.
    for entry in glob(&format!("{}/block_*.json", blocks_dir.display()))
        .expect("Failed to read glob pattern")
    {
        if let Ok(entry) = entry {
            let file = fs::File::open(&entry).expect("Could not open block file");
            let reader = BufReader::new(file);
            let block: Block = serde_json::from_reader(reader).unwrap();
            let blocknumber = block.height;
            if max_block.is_none() {
                max_block = Some(blocknumber);
                max_block_path = Some(entry);
            } else {
                if blocknumber > max_block.unwrap() {
                    max_block = Some(blocknumber);
                    max_block_path = Some(entry);
                }
            }
        }
    }
    (max_block, max_block_path)
}

/// Parse a block_x.json file and return a Block
pub fn parse_block_file(path: PathBuf) -> Block {
    let file =
        fs::File::open(&path).expect(&format!("Could not open block file: {:?}", path.to_str()));
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap()
}

/* ////////////// */
/* / Unit tests / */
/* ////////////// */

// Tests generate side-effects. For now run sequentially with `cargo test -- --test-threads 1`
#[allow(dead_code)]
fn test_helper_clear_block_dir(blocks_dir: &PathBuf) {
    // delete the temporary test file and directory.
    // remove_dir_all is scary: be careful with this.
    if blocks_dir.exists() {
        fs::remove_dir_all(blocks_dir).unwrap();
    }
}
#[test]
#[ignore]
//Not really a test, just a way to generate fixtures.
fn create_fixtures() {
    use diem_wallet::WalletLibrary;

    // if no file is found, the block height is 0
    //let blocks_dir = Path::new("./test_blocks");
    for i in 0..6 {
        let ns = i.to_string();
        let mut wallet = WalletLibrary::new();

        let (_auth_key, _) = wallet.new_address().expect("Could not generate address");

        let mnemonic_string = wallet.mnemonic(); //wallet.mnemonic()
        let save_to = format!("./test_fixtures_{}/", ns);
        fs::create_dir_all(save_to.clone()).unwrap();
        let mut configs_fixture = test_make_configs_fixture();
        configs_fixture.workspace.block_dir = save_to.clone();

        // mine to save_to path
        write_genesis(&configs_fixture);

        // also create mnemonic
        let mut mnemonic_path = PathBuf::from(save_to.clone());
        mnemonic_path.push("owner.mnem");
        dbg!(&mnemonic_path);
        let mut file = fs::File::create(&mnemonic_path).expect("Could not create file");
        file.write_all(mnemonic_string.as_bytes())
            .expect("Could not write mnemonic");

        // create miner.toml
        //rename the path for actual fixtures
        configs_fixture.workspace.block_dir = "blocks".to_string();
        let toml = toml::to_string(&configs_fixture).unwrap();
        let mut toml_path = PathBuf::from(save_to);
        toml_path.push("miner.toml");
        let file = fs::File::create(&toml_path);
        file.unwrap()
            .write(&toml.as_bytes())
            .expect("Could not write toml");
    }
}

#[test]
fn test_mine_once() {
    // if no file is found, the block height is 0
    let mut configs_fixture = test_make_configs_fixture();
    configs_fixture.workspace.block_dir = "test_blocks_temp_2".to_owned();
    // let configs_fixture = OlCliConfig {
    //     workspace: Workspace{
    //         node_home: PathBuf::from("."),
    //     },
    //     profile: Profile {
    //         auth_key: "3e4629ba1e63114b59a161e89ad4a083b3a31b5fd59e39757c493e96398e4df2"
    //             .to_owned(),
    //         account: PeerId::from_hex_literal("0x000000000000000000000000deadbeef").unwrap(),
    //         ip: "1.1.1.1".parse().unwrap(),
    //         statement: "Protests rage across the nation".to_owned(),
    //     },
    //     chain_info: ChainInfo {
    //         chain_id: "0L testnet".to_owned(),
    //         block_dir: "test_blocks_temp_2".to_owned(),
    //         base_waypoint: None,
    //         default_node: Some("http://localhost:8080".parse().unwrap()),
    //         upstream_nodes: None,
    //     },
    // };

    // Clear at start. Clearing at end can pollute the path when tests fail.
    test_helper_clear_block_dir(&configs_fixture.get_block_dir());

    let fixture_previous_proof = decode("005f6371e754d98dd0230d051fce8462cd64257717e988ffbff95ed9b84d130b6ee1a97bff4eedc4cd28721b1f78358f8ce1a7f0b0a2e75a4740af0f328414daad2b3c205a82bbd334b7fc9ae70b8628fb7f02247b0c6416a25662202d8c63de116876b8fb575d2cffae9ea48bd511142ea5f737a9278106093e143f8c6b8d0dd13804ca601310c059ce1db3fd58eb3068dde0658a4e330cc8e5934ab2fe41e4b757e69b2edce436ceac8b0e801b66fcf453f36a4300c286039143e36dfbc100c5d0f40cd7d74a9421b3b8e547de5e82797f365c5524d35813820de538c6ef2ef980995d071a6fa26826335626f1b1b4ee256b67603b1b7df338b4607137bd433affba8a94c6f234defb09ef6d5cc697a73a5b57caf9ef8992ccf4ab35affd997c8294be37b1cfae93fe89781062cc50435fadc9be416279e02ba2eddbdbb659fbc60d8eb76f2bed5adf4a26c6a81f39eea20d65b81e91e52a38eab6229cb975bc75f46dfa65ada848234dd362aa086091fd95a0df21cb2a59d34b155a5105aef71c1a6c7ef340194f1ea3697ec59feb5ce3ea67a00149b36af5de44d2c3863e580267cffee49b9f5ba20104d65f5333c05839e5877006de9dd4c203953cc103faf82fb50a76856333fbe5b36fb6ea76123c343f2bd56192d5c300e17699659cea5acf5991643ba05fef2e399ca68d027a74c6c7c908c03adfa1b7f5c56d163ee37b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001").unwrap();

    let fixture_block = Block {
        /// Block Height
        height: 0u64,
        elapsed_secs: 0u64,
        preimage: Vec::new(),
        proof: fixture_previous_proof,
    };

    write_json(&fixture_block, &configs_fixture.get_block_dir());
    mine_once(&configs_fixture).unwrap();
    // confirm this file was written to disk.
    let block_file = fs::read_to_string("./test_blocks_temp_2/block_1.json")
        .expect("Could not read latest block");
    let latest_block: Block =
        serde_json::from_str(&block_file).expect("could not deserialize latest block");
    // Test the file is read, and blockheight is 0
    assert_eq!(latest_block.height, 1, "Not the droid you are looking for.");

    // Test the expected proof is writtent to file correctly.
    let correct_proof = "006d5479373bd7b075fb8e55f655d62a800817b4c9dff48cbaf91c9249948c76a7ab900031d333436e10dcfa5e5e1c2c732b7ce01f603390ba43941bd49ce314f44156ca3210a1577d67f9d2517a647a387c9b0df5588139d9c48550592a1354ca457da54ee9b4371b465e22af269a2fa7545521163447ed70e291f1f9c57636a00056502b2198290840a4569859abcf08901ea4d7bd2f3a9807f053ea7ff03d3b6242aaab30c5dfa00fc51944fc96d7099311a2513a59ba1d61e7383ac9b12eaafa3fc5102c2430da354d3c00ebcf90fa7451856bac5b70ee85eceb61b7dca12d2a7c08573cc8c3ba9b39ec41249a819685c36b69aa9eef7302be0987f29363813f000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001";
    assert_eq!(
        hex::encode(&latest_block.proof),
        correct_proof,
        "Not the proof of the new block created"
    );

    test_helper_clear_block_dir(&configs_fixture.get_block_dir());
}

#[test]
fn test_mine_genesis() {
    // if no file is found, the block height is 0
    //let blocks_dir = Path::new("./test_blocks");
    let configs_fixture = test_make_configs_fixture();

    //clear from sideffects.
    test_helper_clear_block_dir(&configs_fixture.get_block_dir());

    // mine
    write_genesis(&configs_fixture);
    // read file
    let block_file =
        // TODO: make this work: let latest_block_path = &configs_fixture.chain_info.block_dir.to_string().push(format!("block_0.json"));
        fs::read_to_string("./test_blocks_temp_1/block_0.json").expect("Could not read latest block");

    let latest_block: Block =
        serde_json::from_str(&block_file).expect("could not deserialize latest block");

    // Test the file is read, and blockheight is 0
    assert_eq!(latest_block.height, 0, "test");

    // Test the expected proof is writtent to file correctly.
    let correct_proof = "004812351796d94cab9a9932ec179630d86ced7db3fd7dbd66f36b0a9c0f1398756465304908e05c6397bffc7b6ac9b158c166eaade6169ffe54ddcd9251c149c5f3cceeacc59a9044bc9b9427237ac63d189873736b8ff2970f236f541bef4cf1d789f39f97f8b87ecccf7fd34f99bc4e193986da0761f5698f9715de76f0b5b5ffd41a4dcd73c3bbc19f1047bb2862c65699a4dc5ecbdf6297383a6a4e97d346a098f0a6d83b9aa3ccb703ab008356c45fb84a6e550f06f98c55300865f9774d0dee94bfcefc79208c35e79b3a8458ac246193ccf36f5b8d74975d1b0ecb7cefc419960fa9b31bda6582a046c70ba2aaec637a5d0a95ec59cda1edcf8a51b97463000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001";
    assert_eq!(hex::encode(&latest_block.proof), correct_proof, "test");

    test_helper_clear_block_dir(&configs_fixture.get_block_dir());
}

#[test]
fn test_parse_no_files() {
    // if no file is found, the block height is 0
    let blocks_dir = PathBuf::from(".");
    assert_eq!(parse_block_height(&blocks_dir).0, None);
}

#[test]
fn test_parse_one_file() {
    // create a file temporarily in ./test_blocks with height 33
    let current_block_number = 33;
    let block = Block {
        height: current_block_number,
        elapsed_secs: 0u64,
        preimage: Vec::new(),
        proof: Vec::new(),
    };

    // write the file temporarilty
    let blocks_dir = PathBuf::from("./test_blocks_temp_3");
    // Clear at start. Clearing at end can pollute the path when tests fail.
    test_helper_clear_block_dir(&blocks_dir);

    fs::create_dir(&blocks_dir).unwrap();
    let mut latest_block_path = blocks_dir.clone();
    latest_block_path.push(format!("block_{}.json", current_block_number));
    let mut file = fs::File::create(&latest_block_path).unwrap();
    file.write_all(serde_json::to_string(&block).unwrap().as_bytes())
        .expect("Could not write block");

    // block height
    assert_eq!(parse_block_height(&blocks_dir).0, Some(33));

    test_helper_clear_block_dir(&blocks_dir)
}

/// make fixtures for file
pub fn test_make_configs_fixture() -> AppCfg {
    let mut cfg = AppCfg::default();
    cfg.workspace.node_home = PathBuf::from(".");
    cfg.workspace.block_dir = "test_blocks_temp_1".to_owned();
    cfg.chain_info.chain_id = "0L testnet".to_owned();
    cfg.profile.auth_key =
        "3e4629ba1e63114b59a161e89ad4a083b3a31b5fd59e39757c493e96398e4df2".to_string();
    cfg
}

/// Format the config file data into a fixed byte structure for easy parsing in Move/other languages
pub fn genesis_preimage(cfg: &AppCfg) -> Vec<u8> {
    const AUTH_KEY_BYTES: usize = 32;
    const CHAIN_ID_BYTES: usize = 64;
    const STATEMENT_BYTES: usize = 1008;

    let mut preimage: Vec<u8> = vec![];

    let mut padded_key_bytes = match decode(cfg.profile.auth_key.clone()) {
        Err(x) => panic!("Invalid 0L Auth Key: {}", x),
        Ok(key_bytes) => {
            if key_bytes.len() != AUTH_KEY_BYTES {
                panic!(
                    "Expected a {} byte 0L Auth Key. Got {} bytes",
                    AUTH_KEY_BYTES,
                    key_bytes.len()
                );
            }
            key_bytes
        }
    };

    preimage.append(&mut padded_key_bytes);

    let mut padded_chain_id_bytes = {
        let mut chain_id_bytes = cfg.chain_info.chain_id.clone().into_bytes();

        match chain_id_bytes.len() {
            d if d > CHAIN_ID_BYTES => panic!(
                "Chain Id is longer than {} bytes. Got {} bytes",
                CHAIN_ID_BYTES,
                chain_id_bytes.len()
            ),
            d if d < CHAIN_ID_BYTES => {
                let padding_length = CHAIN_ID_BYTES - chain_id_bytes.len() as usize;
                let mut padding_bytes: Vec<u8> = vec![0; padding_length];
                padding_bytes.append(&mut chain_id_bytes);
                padding_bytes
            }
            d if d == CHAIN_ID_BYTES => chain_id_bytes,
            _ => unreachable!(),
        }
    };

    preimage.append(&mut padded_chain_id_bytes);

    preimage
        .write_u64::<LittleEndian>(delay_difficulty())
        .unwrap();

    let mut padded_statements_bytes = {
        let mut statement_bytes = cfg.profile.statement.clone().into_bytes();

        match statement_bytes.len() {
            d if d > STATEMENT_BYTES => panic!(
                "Chain Id is longer than 1008 bytes. Got {} bytes",
                statement_bytes.len()
            ),
            d if d < STATEMENT_BYTES => {
                let padding_length = STATEMENT_BYTES - statement_bytes.len() as usize;
                let mut padding_bytes: Vec<u8> = vec![0; padding_length];
                padding_bytes.append(&mut statement_bytes);
                padding_bytes
            }
            d if d == STATEMENT_BYTES => statement_bytes,
            _ => unreachable!(),
        }
    };

    preimage.append(&mut padded_statements_bytes);

    assert_eq!(
        preimage.len(),
        (
            AUTH_KEY_BYTES // 0L Auth_Key
            + CHAIN_ID_BYTES // chain_id
            + 8 // iterations/difficulty
            + STATEMENT_BYTES
            // statement
        ),
        "Preimage is the incorrect byte length"
    );
    return preimage;
}
