//! `version` subcommand

#![allow(clippy::never_loop)]

use std::fs;

use crate::{commands::wizard_val_cmd::write_account_json, prelude::app_config};
use super::wizard_val_cmd::get_autopay_batch;
use abscissa_core::{Command, Options, Runnable, status_info, time::{DateTime, Utc}};
use ol_keys::wallet;
use ol::config::AppCfg;
use ol_types::autopay::{InstructionType, PayInstruction, write_batch_file};

/// `val-wizard` subcommand
#[derive(Command, Debug, Default, Options)]
pub struct FixCmd {}

impl Runnable for FixCmd {
    /// Print version message
    fn run(&self) {
        status_info!("\nOnboard fix", "migrating account.json");
        migrate_account_json(&app_config().clone());
    }
  }

/// fixes account json
pub fn migrate_account_json(cfg: &AppCfg) {
  let (_, _, wallet) = wallet::get_account_from_prompt();
  let home_path = cfg.workspace.node_home.clone();
  println!("Reading autopay configs");
  let (autopay_batch, autopay_signed) = get_autopay_batch(
        &None,
        &None,
        &home_path,
        &cfg,
        &wallet,
    );

    let account_json_path = cfg.workspace.node_home.clone().join("account.json");
    if account_json_path.exists() {
      let now: DateTime<Utc> = Utc::now();
      let filename = now.format("%Y-%m-%d").to_string() + ".account.json";
      let backup_path = cfg.workspace.node_home.clone().join(filename);
      println!("backing up {:?} to {:?}", &account_json_path, &backup_path);
      fs::copy(&account_json_path, &backup_path).expect("could not backup account.json");
    }

    migrate_autopay_json_4_3_0(cfg, autopay_batch.clone().unwrap());

    println!("writing account.json to {:?}", cfg.workspace.node_home.clone());

    // Write account manifest
    write_account_json(
        &None,
        wallet,
        Some(cfg.to_owned()),
        autopay_batch,
        autopay_signed,
    );
}

/// migrate autopay.json for archive purposes
pub fn migrate_autopay_json_4_3_0(cfg: &AppCfg, instructions: Vec<PayInstruction>) {
  let file_path = cfg.workspace.node_home.clone().join("back.autopay_batch.json");

  println!("\nmigrating autopay_batch.json to {:?}\n", &file_path);

  let vec_instr: Vec<PayInstruction> = instructions.into_iter()
  .map(|mut i| {
    if i.type_of == InstructionType::PercentOfChange {
      i.type_of = InstructionType::PercentOfBalance;
      i.type_move = None;
      i.value_move = None;
      i.duration_epochs = Some(1);
    }
    i
  })
  .collect();

  write_batch_file(file_path, vec_instr).expect("could not save autopay_batch file");
}