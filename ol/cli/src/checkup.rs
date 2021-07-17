//! check the AppConfig file 0L.toml
/// 
/// Shows hints if any field is missing or if config entries point to a wrong place
/// 

use libra_global_constants::{CONFIG_FILE, NODE_HOME};

use regex::Regex;
use std::path::PathBuf;
use walkdir::WalkDir;
use crate::migrate;
use rand::Rng;
use std::fs::File;
use std::fs;

/// check the 0l.toml file
pub fn checkup(opt_home_path: Option<PathBuf>) {
    let home = opt_home_path.unwrap_or(dirs::home_dir().unwrap().join(NODE_HOME));
    let config_file = home.join(CONFIG_FILE);

    check_toml(config_file, home);

}

fn check_toml(config_file: PathBuf, node_home: PathBuf) {
    if !config_file.exists() {
        println!("CRITICAL: config file: {:?} does not exist - seems like system is not set up at all", config_file);
        return;
    } else {
        println!("INFO: your 0L.toml file which is now checked is here: {:?}", config_file);
    }

    // workspace section could look like this, for a user "val":
    // 
    // [workspace]
    // source_path = "/home/val/libra"
    // node_home = "/home/val/.0L/"
    // block_dir = "blocks"
    // db_path = "/home/val/.0L/db"

    check_source_path(&config_file);
    check_node_home(&config_file);
    check_block_dir(&config_file, &node_home);
    check_db_dir(&config_file);
}

/// check if source_path dir exists seems like source code
fn check_source_path(config_file: &PathBuf) {
    let maybe_source_path = get_setting(&config_file, "workspace", "source_path");
    if maybe_source_path.is_none() {
        println!("CRITICAL: attribute source_path is not set in 0L.toml");
    } else {
        let source_path = PathBuf::from(maybe_source_path.unwrap()); 
        if ! source_path.is_dir() {
            println!("CRITICAL: attribute source_path points to a place which is not a directory: {:?}", source_path);
        } else  {
            let num_cargo_files = get_num_files_matching(&source_path, Regex::new(r"^Cargo").unwrap());
            if num_cargo_files == 0 {
                println!("CRITICAL: attribute source_path points to a place which seems not to be the project source directory {:?}", source_path);
            }               
        }
    }
}

/// check if node_home dir exists and is writable
fn check_node_home(config_file: &PathBuf) {
    let maybe_nodehome_dir = get_setting(&config_file, "workspace", "node_home");
    if maybe_nodehome_dir.is_none() {
        println!("CRITICAL: attribute node_home is not set in 0L.toml");
    } else {
        let nodehome_dir = PathBuf::from(maybe_nodehome_dir.unwrap()); 
        if ! nodehome_dir.is_dir() {
            println!("CRITICAL: attribute node_home points to a place which is not a directory: {:?}", nodehome_dir);
        } else  {
            if ! is_writable(&nodehome_dir) {
                println!("CRITICAL: directory {:?} is not writable for current user", nodehome_dir);
            }
        }
    }
}

/// check if blocks dir exists and is writable
/// print some info about already mined blocks
fn check_block_dir(config_file: &PathBuf, node_home: &PathBuf) {
    let maybe_blockdir = get_setting(&config_file, "workspace", "block_dir");
    if maybe_blockdir.is_none() {
        println!("CRITICAL: attribute block_dir is not set in 0L.toml");
    } else {
        let block_dir = node_home.join(maybe_blockdir.unwrap()); 
        if ! block_dir.is_dir() {
            println!("CRITICAL: attribute block_dir points to a place which is not a directory: {:?}", block_dir);
        } else  {
            if ! is_writable(&block_dir) {
                println!("CRITICAL: directory {:?} is not writable for current user", block_dir);
            } else {
                let num_mined_blocks = get_num_files_matching(&block_dir, Regex::new(r"^block_[0-9]*.json$").unwrap());
                println!("INFO: directory {:?} contains {:?} mined blocks", block_dir, num_mined_blocks);
            }
        }
    } 
}

/// check if database dir exists and is writable
/// print some info about existing sst files in subdirs consensusdb and libradb
fn check_db_dir(config_file: &PathBuf) {
    let maybe_dbdir = get_setting(&config_file, "workspace", "db_path");
    if maybe_dbdir.is_none() {
        println!("CRITICAL: attribute db_path is not set in 0L.toml");
    } else {
        let dbdir = maybe_dbdir.unwrap(); 
        let dbdir_path = PathBuf::from(dbdir);

        if ! dbdir_path.is_dir() {
            println!("CRITICAL: attribute db_path points to a place which is not a directory: {:?}", dbdir_path);
        } else  {
            if ! is_writable(&dbdir_path) {
                println!("CRITICAL: directory {:?} is not writable for current user", dbdir_path);
            } else {
                let consensusdb_path = dbdir_path.join("consensusdb");
                let libradb_path = dbdir_path.join("libradb");
                let num_sst_files_consensus = get_num_files_matching(&consensusdb_path, Regex::new(r".sst$").unwrap());
                println!("INFO: your consensus db ({:?}) contains {:?} sst-files", consensusdb_path, num_sst_files_consensus);  
                let num_sst_files_libradb = get_num_files_matching(&libradb_path, Regex::new(r".sst$").unwrap());
                println!("INFO: your libradb ({:?}) contains {:?} sst-files", libradb_path, num_sst_files_libradb);  
            }
        }
    } 
}

/// check if given directory is writable for current user
fn is_writable(some_directory: &PathBuf) -> bool {
    // Rust's readonly() check is not sufficient - so create a temp file and acheck if it works
    // ! some_directory.metadata().unwrap().permissions().readonly()

    let mut rng = rand::thread_rng();
    let random_filename = some_directory.join(rng.gen::<u32>().to_string());
    if File::create(&random_filename).is_err() {
        return false;
    }
    fs::remove_file(&random_filename).unwrap();
    true
}

/// counts how many files in the directory match the given pattern
fn get_num_files_matching(dirname: &PathBuf, pattern: Regex) -> u32 {
    let mut counter = 0;
    for entry in WalkDir::new(dirname)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok()) {
        if pattern.is_match(&entry.file_name().to_string_lossy()) {
            counter+=1;
        }
    }
    counter
}

/// searches in the `filename` if `attribute`
/// exists in `section`.
/// 
/// example call:
///  get_setting("/home/val/.0L/0L.toml", "workspace", "db_path");
///
pub fn get_setting(filename: &PathBuf, section: &str, attribute: &str) -> Option<String> {
    let mut in_my_section = false;

    let any_section_start_re = Regex::new(r"^\[.*\]$").unwrap();
    let my_section_re = Regex::new(&format!(r"^\[{}\]$", section).as_str()).unwrap();
    let my_attribute_re = Regex::new(&format!(r#"^{}[ \t]*= *"([^"]*)"#, attribute).as_str()).unwrap();

    // round 1: check if attribute already exists
    let file_content = migrate::read_file(&filename);
    for line in file_content.lines() {
        if any_section_start_re.is_match(&line) {
            in_my_section = my_section_re.is_match(&line);
        } else {
            if in_my_section && my_attribute_re.is_match(&line) {
                let caps = my_attribute_re.captures(&line).unwrap();
                return Some(String::from(caps.get(1).unwrap().as_str()));
            }
        }
    }
    return None;
}


