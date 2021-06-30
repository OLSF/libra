// Case 1: Validators are compliant. 
// This test is to check if validators are present after the first epoch.
// Here EPOCH-LENGTH = 15 Blocks.
// NOTE: This test will fail with Staging and Production Constants, only for Debug - due to epoch length.

//! account: alice, 1000000, 0, validator
//! account: bob, 1000000, 0, validator
//! account: carol, 1000000, 0, validator
//! account: dave, 1000000, 0, validator
//! account: eve, 1000000, 0, validator

//! block-prologue
//! proposer: alice
//! block-time: 1
//! NewBlockEvent

//! new-transaction
//! sender: diemroot
script {
    
    use 0x1::DiemSystem;
    fun main(_account: signer) {
        // Tests on initial size of validators 
        assert(DiemSystem::validator_set_size() == 5, 7357000180101);
        assert(DiemSystem::is_validator({{alice}}) == true, 7357000180102);
        assert(DiemSystem::is_validator({{bob}}) == true, 7357000180103);
    }
}
// check: EXECUTED


//! new-transaction
//! sender: diemroot
script {
    
    use 0x1::DiemSystem;
    fun main(_account: signer) {
        // Tests on initial size of validators 
        assert(DiemSystem::validator_set_size() == 5, 7357000180104);
        assert(DiemSystem::is_validator({{alice}}) == true, 7357000180105);
        assert(DiemSystem::is_validator({{bob}}) == true, 7357000180106);
    }
}
//check: EXECUTED

//! new-transaction
//! sender: diemroot
script {
    use 0x1::Vector;
    use 0x1::Stats;
    // This is the the epoch boundary.
    fun main(vm: signer) {
        let voters = Vector::empty<address>();
        Vector::push_back<address>(&mut voters, {{alice}});
        Vector::push_back<address>(&mut voters, {{bob}});
        Vector::push_back<address>(&mut voters, {{carol}});
        Vector::push_back<address>(&mut voters, {{dave}});
        Vector::push_back<address>(&mut voters, {{eve}});

        // Overwrite the statistics to mock that all have been validating.
        let i = 1;
        while (i < 16) {
            // Mock the validator doing work for 15 blocks, and stats being updated.
            Stats::process_set_votes(vm, &voters);
            i = i + 1;
        };
    }
}
//check: EXECUTED


//////////////////////////////////////////////
///// Trigger reconfiguration at 61 seconds ////
//! block-prologue
//! proposer: alice
//! block-time: 61000000
//! round: 15

///// TEST RECONFIGURATION IS HAPPENING ////
// check: NewEpochEvent
//////////////////////////////////////////////

//! new-transaction
//! sender: diemroot
script {
    
    use 0x1::DiemSystem;
    use 0x1::DiemConfig;
    fun main(_account: signer) {
        assert(DiemSystem::validator_set_size() == 5, 7357180103011000);
        assert(DiemConfig::get_current_epoch() == 2, 7357180103021000);
    }
}
// check: EXECUTED