//! account: alice, 100, 0, validator

//! new-transaction
// Check if genesis subsidies have been distributed
//! sender: diemroot
script {
    use 0x1::Subsidy;
    
    fun main(_vm: signer) {
        let ceiling = 101u64; // note the rounding
        let baseline_auction_units =  10u64;
        let current_proofs_verified = 5u64;
        
        let value = Subsidy::calc_auction(
            ceiling,
            baseline_auction_units,
            current_proofs_verified
        );
        assert(value == 20, 735703);
    }
}
//check: EXECUTED