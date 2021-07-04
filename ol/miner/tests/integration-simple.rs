
#![forbid(unsafe_code)]

use std::{
    path::{Path},
    time::{self, Duration},
    fs,
    thread,
    process::{Command, Stdio},
};


#[test]
#[ignore]
pub fn integration() {

    // PREPARE FIXTURES
    // the transactions will always abort if the fixtures are incorrect.
    // in swarm, all validators in genesis used NodeConfig.defaul() preimage and proofs.
    // these are equivalent to fixtures/block_0.json.test.alice 
    // for the test to work:

    // the miner needs to start producing block_1.json. If block_1.json is not successful, then block_2 cannot be either, because it depends on certain on-chain state from block_1 correct submission.

    // TODO: Assert that block_0.json is in blocks folder.
    std::env::set_var("RUST_LOG", "debug");
    let mut swarm_cmd = Command::new("cargo");
    swarm_cmd.current_dir("../");
    swarm_cmd.arg("run")
            .arg("-p").arg("diem-swarm")
            .arg("--")
            .arg("-n").arg("1")
            .arg("--diem-node").arg("../target/debug/diem-node ")
            .arg("-c").arg("swarm_temp");
    let cmd = swarm_cmd.stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn();
    match cmd {
        // Swarm has started
        Ok(mut swarm_child) => {
            // need to wait for swarm to start-up before we have the configs needed to connect to it. Check stdout.
            block_until_swarm_ready();
            println!("READY!");
            // wait a bit more, because the previous command only checks for config fils creation.
            let test_timeout = Duration::from_secs(30);
            thread::sleep(test_timeout);

            // copy fixtures
            fs::create_dir(&"../swarm_temp/blocks").unwrap();
            // copy fixtures/block_0.json.test.alice -> blocks/block_0.json
            fs::copy("../fixtures/blocks/test/alice/block_0.json", "../swarm_temp/blocks/block_0.json").unwrap();

            // start the miner swarm test helper.
            let mut miner_cmd = Command::new("cargo");
            miner_cmd.arg("run")
                    .arg("swarm");
            let mut miner_child = miner_cmd.stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn()
                    .unwrap();

            // set a timeout
            let test_timeout = Duration::from_secs(120);
            thread::sleep(test_timeout);
            swarm_child.kill().unwrap();
            miner_child.kill().unwrap();

            // TODO: get output and evaluate with assert
            // assert_eq!()
        }
        Err(err) => println!("Swarm child process did not start: {}", err)
    }
}

fn block_until_swarm_ready() -> bool {
    let _swarm_configs_path = Path::new("../swarm_temp/");
    let mut timeout = 100;
    let one_second = time::Duration::from_secs(1);

    loop {
        if timeout == 0 { 
            return false
        }
        if Path::new("../swarm_temp/").exists() {
            return true
        }

        thread::sleep(one_second);
        timeout -= 1;
    }
}
