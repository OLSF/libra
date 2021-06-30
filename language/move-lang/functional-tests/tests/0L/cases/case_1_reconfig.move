// This tests consensus Case 1.
// ALICE is a validator.
// DID validate successfully.
// DID mine above the threshold for the epoch.

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
//! sender: alice
script {
    use 0x1::DiemSystem;
    use 0x1::MinerState;
    use 0x1::NodeWeight;
    use 0x1::GAS::GAS;
    use 0x1::DiemAccount;

    fun main(sender: signer) {
        // Tests on initial size of validators
        assert(DiemSystem::validator_set_size() == 5, 7357300101011000);
        assert(DiemSystem::is_validator({{alice}}) == true, 7357300101021000);
        assert(DiemSystem::is_validator({{eve}}) == true, 7357300101031000);

        assert(MinerState::get_count_in_epoch({{alice}}) == 1, 7357300101041000);
        assert(DiemAccount::balance<GAS>({{alice}}) == 1, 7357300101051000);
        assert(NodeWeight::proof_of_weight({{alice}}) == 0, 7357300101051000);

        // Alice continues to mine after genesis.
        // This test is adapted from chained_from_genesis.move
        MinerState::test_helper_mock_mining(&sender, 5);
        assert(MinerState::get_count_in_epoch({{alice}}) == 5, 7357300101071000);
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
        FullnodeState::test_set_fullnode_fixtures(
            &vm, {{alice}}, 0, 0, 0, 200, 200, 1000000
        );

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
            Stats::process_set_votes(&vm, &voters);
            i = i + 1;
        };
    }
}
//check: EXECUTED

//! new-transaction
//! sender: diemroot
script {
    use 0x1::Cases;
    
    fun main(vm: signer) {
        // We are in a new epoch.
        // Check alice is in the the correct case during reconfigure
        assert(Cases::get_case(&vm, {{alice}}, 0, 15) == 1, 7357000180109);
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
        assert(DiemSystem::validator_set_size() == 5, 7357000180110);
        assert(DiemSystem::is_validator({{alice}}) == true, 7357000180111);
        print(&DiemAccount::balance<GAS>({{alice}}));
        
        let starting_balance = 1;
        let expected_subsidy = 295000000; //294978321
        let operator_refund = 4336 * 5; // BASELINE_TX_COST * proofs = 21680
        let ending_balance = starting_balance + expected_subsidy - operator_refund;
        assert(DiemAccount::balance<GAS>({{alice}}) == ending_balance, 7357000180112);  
        assert(NodeWeight::proof_of_weight({{alice}}) == 1, 7357000180113);  
    }
}
//check: EXECUTED