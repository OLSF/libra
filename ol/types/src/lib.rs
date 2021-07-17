//! `ol-util`

#![forbid(unsafe_code)]
#![warn(
    missing_docs,
    rust_2018_idioms,
    trivial_casts,
    unused_lifetimes,
    unused_qualifications
)]

pub mod account;
pub mod pay_instruction;
pub mod block;
pub mod config;
pub mod oracle_upgrade;
pub mod miner_state;
pub mod dialogue;
pub mod autopay;
pub mod validator_config;
