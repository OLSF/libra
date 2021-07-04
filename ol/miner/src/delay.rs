//! MinerApp delay module
#![forbid(unsafe_code)]

/// Functions for running the VDF.
use vdf::{VDFParams, WesolowskiVDFParams, VDF};
use std::env;
use libra_global_constants::VDF_SECURITY_PARAM;

/// Switch settings between production and testing
pub fn delay_difficulty() -> u64 {
    let node_env = match env::var("NODE_ENV") {
        Ok(val) => val,
        _ => "prod".to_string() // default to "prod" if not set
    };
    // must explicitly set env to prod to use production difficulty.
    if node_env == "prod" {
        return 5000000
    }
    return 100 // difficulty for test suites and on local for debugging purposes.
}

/// Runs the VDF
pub fn do_delay(preimage: &[u8]) -> Vec<u8> {
    let delay_length = delay_difficulty();
    // Functions for running the VDF.
    let vdf: vdf::WesolowskiVDF = WesolowskiVDFParams(VDF_SECURITY_PARAM).new();
    vdf.solve(preimage, delay_length)
        .expect("iterations should have been valiated earlier")
}

/// Verifies a proof
pub fn verify(preimage: &[u8], proof: &[u8]) -> bool{
    let vdf: vdf::WesolowskiVDF = WesolowskiVDFParams(VDF_SECURITY_PARAM).new();
    
    match vdf.verify(preimage, delay_difficulty(), proof) {
       Ok(_) => true,
       Err(e) => {
        println!("Proof is not valid. {:?}", e);
        false
       }
    }
}