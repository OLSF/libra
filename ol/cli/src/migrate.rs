//! migrates the AppConfig file 0L.toml
///
/// tool to migrate config files
/// should be run after version upgrade
///
/// this version implements migration of 0L.toml from v4.2.8 to 4.3.0
///
///
/// code is not performance optimized
/// reads and writes the config file for each attribute, but it's a one time job...
///
use libra_global_constants::{CONFIG_FILE, NODE_HOME};

use chrono::{DateTime, Utc};
use fs_extra::file::{copy, CopyOptions};
use regex::Regex;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

/// Migrate the toml file
pub fn migrate(opt_home_path: Option<PathBuf>) {
    let home = opt_home_path.unwrap_or(dirs::home_dir().unwrap().join(NODE_HOME));
    let config_file = home.join(CONFIG_FILE);

    migrate_toml(config_file, home);
}

fn migrate_toml(config_file: PathBuf, node_home: PathBuf) {
    if !config_file.exists() {
        println!("config file: {:?} does not exist - no migration possible", config_file);
        return;
    }

    // step 1 : make a backup
    make_backup_file(&config_file);

    // step 2: update all attributes

    // ---------------------- udate [workspace] config start ----------------------
    let default_db_path = node_home.join("db").as_path().display().to_string();
    let default_source_path = dirs::home_dir().unwrap().join("libra").as_path().display().to_string();
    add_or_update_s(&config_file, "workspace", "db_path", default_db_path);
    add_or_update_s(&config_file, "workspace", "source_path", default_source_path);
    // ---------------------- udate [workspace] config finished ----------------------

    // ---------------------- udate [chain_info] config start ----------------------
    add_or_update_n(&config_file, "chain_info", "base_epoch", 0);
    // ---------------------- udate [workspace] config finished ----------------------

    // ---------------------- udate the tx costs config start ----------------------
    rename_section(&config_file, "tx_configs.miner_txs", "tx_configs.baseline_cost");

    // previous [tx_configs.management_txs] is renamed and value changed from 1000000 to 100000
    rename_section(&config_file, "tx_configs.management_txs", "tx_configs.management_txs_cost");
    add_or_update_n(&config_file, "tx_configs.management_txs_cost", "max_gas_unit_for_tx", 100000);

    // add [tx_configs.critical_txs_cost] if not there
    add_section(&config_file, "tx_configs.critical_txs_cost");
    add_or_update_n(&config_file, "tx_configs.critical_txs_cost", "user_tx_timeout", 5000);
    add_or_update_n(&config_file, "tx_configs.critical_txs_cost", "coin_price_per_unit", 1);
    add_or_update_n(&config_file, "tx_configs.critical_txs_cost", "max_gas_unit_for_tx", 1000000);

    // add [tx_configs.miner_txs_cost] if not there
    add_section(&config_file, "tx_configs.miner_txs_cost");
    add_or_update_n(&config_file, "tx_configs.miner_txs_cost", "user_tx_timeout", 5000);
    add_or_update_n(&config_file, "tx_configs.miner_txs_cost", "coin_price_per_unit", 1);
    add_or_update_n(&config_file, "tx_configs.miner_txs_cost", "max_gas_unit_for_tx", 10000);

    // add [tx_configs.cheap_txs_cost] if not there
    add_section(&config_file, "tx_configs.cheap_txs_cost");
    add_or_update_n(&config_file, "tx_configs.cheap_txs_cost", "user_tx_timeout", 5000);
    add_or_update_n(&config_file, "tx_configs.cheap_txs_cost", "coin_price_per_unit", 1);
    add_or_update_n(&config_file, "tx_configs.cheap_txs_cost", "max_gas_unit_for_tx", 1000);

    // ---------------------- udate the tx costs config finished ----------------------
}

/// add a new section in case it does not exist already
///
/// searches in the `filename` if `section`
/// If so, the file is not updated.
/// Otherwise an empty `section` is inserted at the end of the file
///
/// example call:
///  add_section("/root/.0L/0L.toml", "new-section");
///
pub fn add_section(filename: &PathBuf, section: &str) {
    let mut section_exists = false;

    let my_section_re = Regex::new(&format!(r"^\[{}\]$", section).as_str()).unwrap();

    // round 1: check if section already exists
    let file_content = read_file(&filename);
    for line in file_content.lines() {
        section_exists |= my_section_re.is_match(&line);
    }

    // round 2: update if necessary
    if section_exists {
        println!("{:?}: section [{}] already there (all fine)", &filename, &section);
    } else {
        let mut file = match File::create(&filename) {
            Err(why) => panic!("couldn't create {:?}: {}", filename, why),
            Ok(file) => file,
        };

        for line in file_content.lines() {
            match file.write_fmt(format_args!("{}\n", &line)) {
                Err(why) => println!("writing to file failed {:?}", why),
                _ => (),
            }
        }
        match file.write_fmt(format_args!("\n[{}]\n", &section)) {
            Err(why) => println!("writing to file failed {:?}", why),
            _ => (),
        }
        println!("{:?}: added section [{}]", &filename, &section);
    }
}

/// rename a section
///
/// searches in the `filename` if `old_section_name` exists
/// If so, the `old_section_name` is irenamed to `new_section_name`
///
/// example call:
///  rename_section("/root/.0L/0L.toml", "old-section", "new-section");
///
pub fn rename_section(filename: &PathBuf, old_section_name: &str, new_section_name: &str) {
    let mut section_exists = false;

    let old_section_re = Regex::new(&format!(r"^\[{}\]$", old_section_name).as_str()).unwrap();
    let new_section_re = Regex::new(&format!(r"^\[{}\]$", new_section_name).as_str()).unwrap();

    // round 1: check if section already exists
    let file_content = read_file(&filename);
    for line in file_content.lines() {
        section_exists |= new_section_re.is_match(&line);
    }

    // round 2: update if necessary
    if section_exists {
        println!("{:?}: section [{}] already there (all fine)", &filename, &new_section_name);
    } else {
        let mut file = match File::create(&filename) {
            Err(why) => panic!("couldn't create {:?}: {}", filename, why),
            Ok(file) => file,
        };

        for line in file_content.lines() {
            if old_section_re.is_match(&line) {
                match file.write_fmt(format_args!("[{}]\n", &new_section_name)) {
                    Err(why) => println!("writing to file failed {:?}", why),
                    _ => (),
                }
            } else {
                match file.write_fmt(format_args!("{}\n", &line)) {
                    Err(why) => println!("writing to file failed {:?}", why),
                    _ => (),
                }
            }
        }

        println!(
            "{:?}: renamed section [{}] to [{}]",
            &filename, &old_section_name, &new_section_name
        );
    }
}

/// add a attribute which should be surrounded by quote signs to a section in case it does not exist already
///
/// searches in the `filename` if `attribute`
/// exists in `section`. If so, the file is not updated.
/// Otherwise the `attribute` in inserted with `value`
///
/// example call:
///  add_or_update("/root/.0L/0L.toml", "workspace", "db_path", "/root/.0L/db");
///
pub fn add_or_update_s(filename: &PathBuf, section: &str, attribute: &str, value: String) {
    add_or_update(filename, section, attribute, format!("\"{}\"", value));
}

/// TODO:
pub fn add_or_update_n(filename: &PathBuf, section: &str, attribute: &str, value: i64) {
    add_or_update(filename, section, attribute, value.to_string());
}

/// adds or updates an attribute in a section
///
/// searches in the `filename` if `attribute`
/// exists in `section`. If so, the attribute is updated to `value`
/// Otherwise the `attribute` in inserted with `value`
///
/// example call:
///  add_or_update("/root/.0L/0L.toml", "workspace", "db_path", "/root/.0L/db");
///
pub fn add_or_update(filename: &PathBuf, section: &str, attribute: &str, value: String) {
    let mut in_my_section = false;
    let mut attribute_exists = false;

    let any_section_start_re = Regex::new(r"^\[.*\]$").unwrap();
    let my_section_re = Regex::new(&format!(r"^\[{}\]$", section).as_str()).unwrap();
    let my_attribute_re = Regex::new(&format!(r"^{}[ \t]*=.*$", attribute).as_str()).unwrap();

    // round 1: check if attribute already exists
    let file_content = read_file(&filename);
    for line in file_content.lines() {
        if any_section_start_re.is_match(&line) {
            in_my_section = my_section_re.is_match(&line);
        } else {
            attribute_exists |= in_my_section && my_attribute_re.is_match(&line);
        }
    }

    // round 2: add or update if necessary
    let mut file = match File::create(&filename) {
        Err(why) => panic!("couldn't create {:?}: {}", filename, why),
        Ok(file) => file,
    };

    in_my_section = false;
    for line in file_content.lines() {
        if any_section_start_re.is_match(&line) {
            in_my_section = my_section_re.is_match(&line);
            match file.write_fmt(format_args!("{}\n", &line)) {
                Err(why) => println!("writing to file failed {:?}", why),
                _ => (),
            }
            if in_my_section && !attribute_exists {
                // add the new attribute and value to start of section
                match file.write_fmt(format_args!("{} = {}\n", attribute, value)) {
                    Err(why) => println!("writing to file failed {:?}", why),
                    _ => println!("{:?}: added property [{}]/{}", &filename, &section, &attribute),
                }
            }
        } else {
            if in_my_section && my_attribute_re.is_match(&line) {
                // update the value
                match file.write_fmt(format_args!("{} = {}\n", attribute, value)) {
                    Err(why) => println!("writing to file failed {:?}", why),
                    _ => println!("{:?}: updated property [{}]/{} to {}", &filename, &section, &attribute, &value),
                }
            } else {
                // otherwise just write the original attribute and value
                match file.write_fmt(format_args!("{}\n", &line)) {
                    Err(why) => println!("writing to file failed {:?}", why),
                    _ => (),
                }
            }
        }
    }
}

/// creates a backup of the file
///
/// the filename of the backup file be like the original
/// filename, with an appended ".bak" and timestamp
///
/// example call
///  make_backup_file("/root/.0L/0L.toml");
/// would create a file
///  "/root/.0L/0L.toml.bak.20210428-200235"
///
fn make_backup_file(filename: &PathBuf) {
    let now: DateTime<Utc> = Utc::now();
    let backup_filename = &format!(
        "{}.bak.{}",
        filename.as_path().display().to_string(),
        now.format("%Y%m%d-%H%M%S")
    );
    match copy(&filename, PathBuf::from(backup_filename), &CopyOptions::new()) {
        Err(why) => panic!("writing backup file failed {:?} - stopping", why),
        _ => println!("created backup file: {:?}", backup_filename),
    }
}

/// read a file into a String
/// borrowed from:
/// https://www.tutorialspoint.com/file-operations-in-rust-programming
pub fn read_file(filename: &PathBuf) -> String {
    let mut file = match File::open(&filename) {
        Err(why) => panic!("unable to open {:?} - {}", filename, why),
        Ok(file) => file,
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("unable to read {:?} - {}", filename, why),
        Ok(_) => return s,
    }
}
