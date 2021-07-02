// This test is to check if new epoch is triggered at end of 15 blocks.
// Here EPOCH-LENGTH = 15 Blocks.
// TO DO: Genesis function call to have 15 round epochs.
// NOTE: This test will fail in test-net and Production, only for Debug - due to epoch length.

//! account: alice, 1000000, 0, validator
//! account: bob, 1000000, 0, validator
//! account: carol, 1000000, 0, validator
//! account: dave, 1000000, 0, validator


//! new-transaction
//! sender: diemroot
script {
    
    use 0x1::DiemSystem;
    use 0x1::NodeWeight;
    fun main(_account: signer) {
        // Tests on initial size of validators 
        assert(DiemSystem::validator_set_size() == 4, 7357220101011000);
        assert(DiemSystem::is_validator({{alice}}) == true, 7357220101021000);
        assert(NodeWeight::proof_of_weight({{alice}}) == 0, 7357220101031000);

    }
}
// check: EXECUTED


//! new-transaction
//! sender: diemroot
script {
    use 0x1::Vector;
    use 0x1::Stats;

    fun main(vm: signer) {
        let voters = Vector::empty<address>();
        Vector::push_back<address>(&mut voters, {{alice}});
        Vector::push_back<address>(&mut voters, {{bob}});
        Vector::push_back<address>(&mut voters, {{carol}});
        Vector::push_back<address>(&mut voters, {{dave}});

        let i = 1;
        while (i < 16) {
            // Mock the validator doing work for 15 blocks, and stats being updated.
            Stats::process_set_votes(&vm, &voters);
            i = i + 1;
        };
    }
}

//////////////////////////////////////////////
///// Trigger reconfiguration 61 seconds ////
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
    use 0x1::NodeWeight;
    fun main(_account: signer) {
        // Tests on initial size of validators 
        assert(DiemSystem::validator_set_size() == 4, 7357220101041000);
        assert(DiemSystem::is_validator({{alice}}) == true, 7357220101051000);
        //no mining was done by Alice.
        assert(NodeWeight::proof_of_weight({{alice}}) == 0, 7357220101061000);
    }
}
// check: EXECUTED
