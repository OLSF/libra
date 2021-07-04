//! `version` subcommand

#![allow(clippy::never_loop)]

use super::files_cmd;
use crate::prelude::app_config;
use crate::entrypoint;
use abscissa_core::{status_info, status_ok, Command, Options, Runnable};
use diem_genesis_tool::node_files;
use diem_types::{transaction::SignedTransaction, waypoint::Waypoint};
use diem_wallet::WalletLibrary;
use ol::{commands::init_cmd, config::AppCfg};
use ol_keys::{scheme::KeyScheme, wallet};
use ol_types::block::Block;
use ol_types::{account::ValConfigs, autopay::PayInstruction, config::TxType};
use reqwest::Url;
use serde_json::Value;
use std::{fs::File, io::Write, path::PathBuf};
use txs::{commands::autopay_batch_cmd, submit_tx};
/// `val-wizard` subcommand
#[derive(Command, Debug, Default, Options)]
pub struct ValWizardCmd {
    #[options(
        short = "a",
        help = "where to output the account.json file, defaults to node home"
    )]
    account_path: Option<PathBuf>,
    #[options(help = "id of the chain")]
    chain_id: Option<u8>,
    #[options(help = "github org of genesis repo")]
    github_org: Option<String>,
    #[options(help = "repo with with genesis transactions")]
    repo: Option<String>,
    #[options(help = "build genesis from ceremony repo")]
    rebuild_genesis: bool,
    #[options(help = "skip fetching genesis blob")]
    skip_fetch_genesis: bool,
    #[options(help = "skip mining a block zero")]
    skip_mining: bool,
    #[options(short = "u", help = "template account.json to configure from")]
    template_url: Option<Url>,
    #[options(help = "autopay file if instructions are to be sent")]
    autopay_file: Option<PathBuf>,
    #[options(help = "An upstream peer to use in 0L.toml")]
    upstream_peer: Option<Url>,
}

impl Runnable for ValWizardCmd {
    /// Print version message
    fn run(&self) {
        status_info!("\nValidator Config Wizard.", "Next you'll enter your mnemonic and some other info to configure your validator node and on-chain account. If you haven't yet generated keys, run the standalone keygen tool with 'ol keygen'.\n\nYour first 0L proof-of-work will be mined now. Expect this to take up to 15 minutes on modern CPUs.\n");

        // Get credentials from prompt
        let (authkey, account, wallet) = wallet::get_account_from_prompt();

        let cfg = app_config().clone(); // read 0L.toml in case it exists
 
        let upstream = match cfg.profile.upstream_nodes {
            Some(url) => url[0].to_owned(),   
            _ => Url::parse("http://localhost:8080").unwrap(),  // default if not configured otherwise
        };

        println!("staring validator wizard with upstream URL: {:?}", &upstream);

        let mut app_config =
            AppCfg::init_app_configs(authkey, account, &Some(upstream), &Some(entrypoint::get_node_home()));

        let home_path = &app_config.workspace.node_home;
        
        status_ok!("\nMiner config written", "\n...........................\n");

        if let Some(url) = &self.template_url {
            
            let mut url = url.to_owned();
            url.set_port(Some(3030)).unwrap(); //web port
            save_template(&url.join("account.json").unwrap(), home_path);
            let (epoch, wp) = get_epoch_info(&url.join("epoch.json").unwrap());

            app_config.chain_info.base_epoch = epoch;
            app_config.chain_info.base_waypoint = wp;
            // get autopay
            status_ok!("\nTemplate saved", "\n...........................\n");
        }

        // Use any autopay instructions
        // TODO: simplify signature
        let (autopay_batch, autopay_signed) = get_autopay_batch(
            &self.template_url,
            &self.autopay_file,
            home_path,
            &app_config,
            &wallet,
        );
        status_ok!(
            "\nAutopay transactions signed",
            "\n...........................\n"
        );

        // Initialize Validator Keys
        init_cmd::initialize_validator(&wallet, &app_config).unwrap();
        status_ok!("\nKey file written", "\n...........................\n");

        // fetching the genesis files from genesis-archive
        // unless we are skipping it, or unless we intend to rebuild.
        if !self.skip_fetch_genesis {
            // if we are rebuilding genesis then we should skip fetching files
            if !self.rebuild_genesis {
                files_cmd::get_files(home_path.to_owned(), &self.github_org, &self.repo);
                status_ok!(
                    "\nDownloaded genesis files",
                    "\n...........................\n"
                );
            }
        }

        let home_dir = app_config.workspace.node_home.to_owned();
        // 0L convention is for the namespace of the operator to be appended by '-oper'
        let namespace = app_config.profile.auth_key.clone() + "-oper";

        // TODO: use node_config to get the seed peers and then write upstream_node vec in 0L.toml from that.
        node_files::write_node_config_files(
            home_dir.clone(),
            self.chain_id.unwrap_or(1),
            &self.github_org.clone().unwrap_or("OLSF".to_string()),
            &self
                .repo
                .clone()
                .unwrap_or("experimental-genesis".to_string()),
            &namespace,
            &self.rebuild_genesis,
            &false,
        )
        .unwrap();

        status_ok!("\nNode config written", "\n...........................\n");

        if !self.skip_mining {
            // Mine Block
            miner::block::write_genesis(&app_config);
            status_ok!(
                "\nGenesis proof complete",
                "\n...........................\n"
            );
        }

        // Write account manifest
        write_account_json(
            &self.account_path,
            wallet,
            Some(app_config.clone()),
            autopay_batch,
            autopay_signed,
        );
        status_ok!(
            "\nAccount manifest written",
            "\n...........................\n"
        );

        status_info!("Your validator node and miner app are now configured.", &format!("\nStart your node with `ol start`, and then ask someone with GAS to do this transaction `txs create-validator -u http://{}`", &app_config.profile.ip));
    }
}

/// get autopay instructions from file
pub fn get_autopay_batch(
    template: &Option<Url>,
    file_path: &Option<PathBuf>,
    home_path: &PathBuf,
    cfg: &AppCfg,
    wallet: &WalletLibrary,
) -> (Option<Vec<PayInstruction>>, Option<Vec<SignedTransaction>>) {
    let file_name = if template.is_some() {
        // assumes the template was downloaded from URL
        "template.json"
    } else {
        "autopay_batch.json"
    };

    let starting_epoch = cfg.chain_info.base_epoch.unwrap();
    let instr_vec = PayInstruction::parse_autopay_instructions(
        &file_path.clone().unwrap_or(home_path.join(file_name)),
        Some(starting_epoch.clone()),
    )
    .unwrap();
    let script_vec = autopay_batch_cmd::process_instructions(instr_vec.clone(), &starting_epoch);
    let url = cfg.what_url(false);
    let mut tx_params =
        submit_tx::get_tx_params_from_toml(cfg.to_owned(), TxType::Miner, Some(wallet), url, None)
            .unwrap();
    // give the tx a very long expiration, 1 day.
    let tx_expiration_sec = 24 * 60 * 60;
    tx_params.tx_cost.user_tx_timeout = tx_expiration_sec;
    let txn_vec = autopay_batch_cmd::sign_instructions(script_vec, 0, &tx_params);
    (Some(instr_vec), Some(txn_vec))
}

pub fn save_template(url: &Url, home_path: &PathBuf) -> PathBuf {
    let g_res = reqwest::blocking::get(&url.to_string());
    let g_path = home_path.join("template.json");
    let mut g_file = File::create(&g_path).expect("couldn't create file");
    let g_content = g_res.unwrap().bytes().expect("cannot connect to upstream node").to_vec(); //.text().unwrap();
    g_file.write_all(g_content.as_slice()).unwrap();
    g_path
}

fn get_epoch_info(url: &Url) -> (Option<u64>, Option<Waypoint>) {
    let g_res = reqwest::blocking::get(&url.to_string());
    let string = g_res.unwrap().text().unwrap();
    let json: Value = string.parse().unwrap();
    let epoch = json
        .get("epoch")
        .unwrap()
        .as_u64()
        .expect("should have epoch number");
    let waypoint = json
        .get("waypoint")
        .unwrap()
        .as_str()
        .expect("should have epoch number");

    (Some(epoch), waypoint.parse().ok())
}

/// Creates an account.json file for the validator
pub fn write_account_json(
    json_path: &Option<PathBuf>,
    wallet: WalletLibrary,
    wizard_config: Option<AppCfg>,
    autopay_batch: Option<Vec<PayInstruction>>,
    autopay_signed: Option<Vec<SignedTransaction>>,
) {
    let cfg = wizard_config.unwrap_or(app_config().clone());
    let json_path = json_path.clone().unwrap_or(cfg.workspace.node_home.clone());
    let keys = KeyScheme::new(&wallet);
    let block = Block::parse_block_file(cfg.get_block_dir().join("block_0.json").to_owned());

    ValConfigs::new(
        block,
        keys,
        cfg.profile.ip.to_string(),
        autopay_batch,
        autopay_signed,
    )
    .create_manifest(json_path);
}
