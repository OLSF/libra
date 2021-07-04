//! `version` subcommand

#![allow(clippy::never_loop)]

use std::{fs::File, path::{PathBuf}};

use crate::{application::app_config};
use abscissa_core::{Command, Options, Runnable};
use diem_genesis_tool::node_files;
use std::io::Write;

use ol_types::config::AppCfg;
/// `files` subcommand
#[derive(Command, Debug, Default, Options)]
pub struct FilesCmd {
    #[options(help = "id of the chain")]
    chain_id: Option<u8>,
    #[options(help = "github org of genesis repo")]
    github_org: Option<String>,
    #[options(help = "repo with with genesis transactions")]
    repo: Option<String>,   
    #[options(help = "build genesis from ceremony repo")]
    rebuild_genesis: bool, 
    #[options(help = "only make fullnode config files")]
    fullnode_only: bool,
}


impl Runnable for FilesCmd {
    /// Print version message
    fn run(&self) {
        let miner_configs = app_config().to_owned();
        genesis_files(
            &miner_configs.clone(),
            &self.chain_id,
            &self.github_org,
            &self.repo,
            &self.rebuild_genesis,
            &self.fullnode_only,
        ) 
    }
}

pub fn genesis_files(
    miner_config: &AppCfg,
    chain_id: &Option<u8>,
    github_org: &Option<String>,
    repo: &Option<String>,
    rebuild_genesis: &bool,
    fullnode_only: &bool,
) {
    let home_dir = miner_config.workspace.node_home.to_owned();
    // 0L convention is for the namespace of the operator to be appended by '-oper'
    let namespace = miner_config.profile.auth_key.clone() + "-oper";
    
    node_files::write_node_config_files(
        home_dir.clone(), 
        chain_id.unwrap_or(1),
        &github_org.clone().unwrap_or("OLSF".to_string()),
        &repo.clone().unwrap_or("experimetal-genesis".to_string()),
        &namespace,
        rebuild_genesis,
        fullnode_only
    ).unwrap();

    println!("validator configurations initialized, file saved to: {:?}", &home_dir.join("validator.node.yaml"));

}

pub fn get_files(
    home_dir: PathBuf,
    github_org: &Option<String>,
    repo: &Option<String>
) {
    let github_org = github_org.clone().unwrap_or("OLSF".to_string());
    let repo = repo.clone().unwrap_or("genesis-archive".to_string());


    let base_url = format!("https://raw.githubusercontent.com/{github_org}/{repo}/main/genesis/", github_org=github_org, repo=repo);

    let w_res = reqwest::blocking::get(&format!("{}genesis_waypoint", base_url));

    let w_path = &home_dir.join("genesis_waypoint");
    let mut w_file = File::create(&w_path).expect("couldn't create file");
    let w_content =  w_res.unwrap().text().unwrap();
    w_file.write_all(w_content.as_bytes()).unwrap();
    println!("genesis waypoint fetched, file saved to: {:?}", w_path);

    let g_res = reqwest::blocking::get(&format!("{}genesis.blob", base_url));
    let g_path = &home_dir.join("genesis.blob");
    let mut g_file = File::create(&g_path).expect("couldn't create file");
    let g_content =  g_res.unwrap().bytes().unwrap().to_vec(); //.text().unwrap();
    g_file.write_all(g_content.as_slice()).unwrap();

    println!("genesis transactions fetched, file saved to: {:?}", g_path);
}