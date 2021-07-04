//! Acceptance test: runs the application as a subprocess and asserts its
//! output for given argument combinations matches what is expected.
//!
//! Modify and/or delete these as you see fit to test the specific needs of
//! your application.
//!
//! For more information, see:
//! <https://docs.rs/abscissa_core/latest/abscissa_core/testing/index.html>

// Tip: Deny warnings with `RUSTFLAGS="-D warnings"` environment variable in CI

#![forbid(unsafe_code)]
#![warn(
    missing_docs,
    rust_2018_idioms,
    trivial_casts,
    unused_lifetimes,
    unused_qualifications
)]

use abscissa_core::testing::prelude::*;
use libra_types::waypoint::Waypoint;
use once_cell::sync::Lazy;
use std::time::Duration;

/// Executes your application binary via `cargo run`.
///
/// Storing this value as a [`Lazy`] static ensures that all instances of
/// the runner acquire a mutex when executing commands and inspecting
/// exit statuses, serializing what would otherwise be multithreaded
/// invocations as `cargo test` executes tests in parallel by default.
pub static RUNNER: Lazy<CmdRunner> = Lazy::new(|| CmdRunner::default());

/// Use `OlCliConfig::default()` value if no config or args
#[test]
#[ignore]
fn start_no_args() {
    let mut runner = RUNNER.clone();
    let mut cmd = runner
        .arg("start")
        .timeout(Duration::new(1, 0))
        .capture_stdout()
        .run();
    cmd.stdout().expect_line("Enter your 0L mnemonic");

    // TODO: This test fails because the miner runs in a loop, the process doesn't exit as expected.
    //cmd.wait().unwrap().expect_success();
}

#[test]
fn test_waypoint_parsing() {
    // let mut runner = RUNNER.clone();
    // let mut cmd = runner
    //     .arg("start")
    //     .arg("--waypoint=0:8859e663dfc13a44d2b67b11bfa4bf7679c61691de5fb0c483c4874b4edae35b")
    //     .timeout(Duration::new(1, 0))
    //     .capture_stdout()
    //     .run();
    // cmd.stdout().expect_line("Enter your 0L mnemonic");

    let waypoint_str = "0:8859e663dfc13a44d2b67b11bfa4bf7679c61691de5fb0c483c4874b4edae35b";

    println!("waypoint \n{:?}", waypoint_str);
    let parsed_waypoint: Waypoint = waypoint_str.parse().unwrap();
    assert_eq!(waypoint_str, parsed_waypoint.to_string());
}
