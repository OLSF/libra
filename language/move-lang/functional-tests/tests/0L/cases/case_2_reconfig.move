
// This tests consensus Case 2.
// ALICE is a validator.
// DID validate successfully.
// DID NOT mine above the threshold for the epoch. 

//! account: alice, 1, 0, validator
//! account: bob, 1, 0, validator
//! account: carol, 1, 0, validator
//! account: dave, 1, 0, validator
//! account: eve, 1, 0, validator

//! block-prologue
//! proposer: alice
//! block-time: 1
//! NewBlockEvent

//! new-transaction
//! sender: bob
script {
    use 0x1::DiemSystem;
    use 0x1::MinerState;
    use 0x1::NodeWeight;
    use 0x1::GAS::GAS;
    use 0x1::DiemAccount;

    fun main(_sender: signer) {
        // Tests on initial size of validators 
        assert(DiemSystem::validator_set_size() == 5, 7357000180101);
        assert(DiemSystem::is_validator({{bob}}) == true, 7357000180102);
        assert(DiemSystem::is_validator({{eve}}) == true, 7357000180103);
        assert(MinerState::test_helper_get_height({{bob}}) == 0, 7357000180104);

        //// NO MINING ////

        assert(DiemAccount::balance<GAS>({{bob}}) == 1, 7357000180106);
        assert(NodeWeight::proof_of_weight({{bob}}) == 0, 7357000180107);  
        assert(MinerState::test_helper_get_height({{bob}}) == 0, 7357000180108);
    }
}
// check: EXECUTED


//! new-transaction
//! sender: diemroot
script {
    use 0x1::Vector;
    use 0x1::Stats;
    use 0x1::FullnodeState;
    // This is the the epoch boundary.
    fun main(vm: signer) {
        // This is not an onboarding case, steady state.
        FullnodeState::test_set_fullnode_fixtures(&vm, {{bob}}, 0, 0, 0, 200, 200, 1000000);

        let voters = Vector::empty<address>();
        Vector::push_back<address>(&mut voters, {{alice}});
        Vector::push_back<address>(&mut voters, {{bob}});
        Vector::push_back<address>(&mut voters, {{carol}});
        Vector::push_back<address>(&mut voters, {{dave}});
        Vector::push_back<address>(&mut voters, {{eve}});

        /// NOTE: BOB DOES NOT MINE

        // Overwrite the statistics to mock that all have been validating.
        let i = 1;
        while (i < 16) {
            // Mock the validator doing work for 15 blocks, and stats being updated.
            Stats::process_set_votes(&vm, &voters);
            i = i + 1;
        };

    }
}

//! new-transaction
//! sender: diemroot
script {
    use 0x1::Cases;
    fun main(vm: signer) {
        // We are in a new epoch.
        // Check Bob is in the the correct case during reconfigure
        assert(Cases::get_case(&vm, {{bob}}, 0, 15) == 2, 7357000180109);
    }
}

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
    use 0x1::NodeWeight;
    use 0x1::GAS::GAS;
    use 0x1::DiemAccount;
    use 0x1::Debug::print;

    fun main(_account: signer) {
        // We are in a new epoch.

        // Check the validator set is at expected size
        // case 2 does not reject Alice.
        assert(DiemSystem::validator_set_size() == 5, 7357000180110);

        assert(DiemSystem::is_validator({{bob}}) == true, 7357000180111);
        
        //case 2 does not get rewards.
        print(&DiemAccount::balance<GAS>({{bob}}));
        assert(DiemAccount::balance<GAS>({{bob}}) == 1, 7357000180112);  

        //case 2 does not increment weight.
        assert(NodeWeight::proof_of_weight({{bob}}) == 0, 7357000180113);  
    }
}