// This tests consensus Case 3.
// CAROL is a validator.
// DID NOT validate successfully.
// DID mine above the threshold for the epoch. 

//! account: alice, 100000, 0, validator
//! account: bob, 100000, 0, validator
//! account: carol, 100000, 0, validator
//! account: dave, 100000, 0, validator
//! account: eve, 100000, 0, validator
//! account: frank, 100000, 0, validator

//! block-prologue
//! proposer: carol
//! block-time: 1
//! NewBlockEvent

//! new-transaction
//! sender: libraroot
script {
    use 0x1::LibraAccount;
    use 0x1::GAS::GAS;
    use 0x1::ValidatorConfig;

    fun main(sender: &signer) {
        // tranfer enough coins to operators
        let oper_bob = ValidatorConfig::get_operator({{bob}});
        let oper_eve = ValidatorConfig::get_operator({{eve}});
        let oper_dave = ValidatorConfig::get_operator({{dave}});
        let oper_alice = ValidatorConfig::get_operator({{alice}});
        let oper_carol = ValidatorConfig::get_operator({{carol}});
        let oper_frank = ValidatorConfig::get_operator({{frank}});
        LibraAccount::vm_make_payment_no_limit<GAS>({{bob}}, oper_bob, 50009, x"", x"", sender);
        LibraAccount::vm_make_payment_no_limit<GAS>({{eve}}, oper_eve, 50009, x"", x"", sender);
        LibraAccount::vm_make_payment_no_limit<GAS>({{dave}}, oper_dave, 50009, x"", x"", sender);
        LibraAccount::vm_make_payment_no_limit<GAS>({{alice}}, oper_alice, 50009, x"", x"", sender);
        LibraAccount::vm_make_payment_no_limit<GAS>({{carol}}, oper_carol, 50009, x"", x"", sender);
        LibraAccount::vm_make_payment_no_limit<GAS>({{frank}}, oper_frank, 50009, x"", x"", sender);
    }
}
//check: EXECUTED

//! new-transaction
//! sender: alice
script {
    use 0x1::MinerState;
    use 0x1::Signer;
    use 0x1::AutoPay2;

    fun main(sender: &signer) {
        AutoPay2::enable_autopay(sender);
        MinerState::test_helper_mock_mining(sender, 5);
        assert(MinerState::get_count_in_epoch(Signer::address_of(sender)) == 5, 73570001);
    }
}
//check: EXECUTED

//! new-transaction
//! sender: bob
script {
    use 0x1::MinerState;
    use 0x1::Signer;
    use 0x1::AutoPay2;

    fun main(sender: &signer) {
        AutoPay2::enable_autopay(sender);
        MinerState::test_helper_mock_mining(sender, 5);
        assert(MinerState::get_count_in_epoch(Signer::address_of(sender)) == 5, 73570001);
    }
}
//check: EXECUTED

//! new-transaction
//! sender: carol
script {
    use 0x1::MinerState;
    use 0x1::Signer;
    use 0x1::AutoPay2;

    fun main(sender: &signer) {
        AutoPay2::enable_autopay(sender);
        MinerState::test_helper_mock_mining(sender, 5);
        assert(MinerState::get_count_in_epoch(Signer::address_of(sender)) == 5, 73570001);
    }
}
//check: EXECUTED

//! new-transaction
//! sender: dave
script {
    use 0x1::MinerState;
    use 0x1::Signer;
    use 0x1::AutoPay2;

    fun main(sender: &signer) {
        AutoPay2::enable_autopay(sender);
        MinerState::test_helper_mock_mining(sender, 5);
        assert(MinerState::get_count_in_epoch(Signer::address_of(sender)) == 5, 73570001);
    }
}
//check: EXECUTED

//! new-transaction
//! sender: eve
script {
    use 0x1::MinerState;
    use 0x1::Signer;
    use 0x1::AutoPay2;

    fun main(sender: &signer) {
        AutoPay2::enable_autopay(sender);
        MinerState::test_helper_mock_mining(sender, 5);
        assert(MinerState::get_count_in_epoch(Signer::address_of(sender)) == 5, 73570001);
    }
}
//check: EXECUTED

//! new-transaction
//! sender: frank
script {
    use 0x1::MinerState;
    use 0x1::Signer;
    use 0x1::AutoPay2;

    fun main(sender: &signer) {
        AutoPay2::enable_autopay(sender);
        MinerState::test_helper_mock_mining(sender, 5);
        assert(MinerState::get_count_in_epoch(Signer::address_of(sender)) == 5, 73570001);
    }
}
//check: EXECUTED

//! new-transaction
//! sender: libraroot
script {
    use 0x1::LibraSystem;
    use 0x1::MinerState;
    use 0x1::GAS::GAS;
    use 0x1::LibraAccount;
    use 0x1::FullnodeState;
    
    fun main(vm: &signer) {
        // This is not an onboarding case, steady state.
        FullnodeState::test_set_fullnode_fixtures(vm, {{carol}}, 0, 0, 0, 200, 200, 1000000);

        // Tests on initial size of validators 
        assert(LibraSystem::validator_set_size() == 6, 7357000180101);
        assert(LibraSystem::is_validator({{carol}}) == true, 7357000180102);
        assert(MinerState::test_helper_get_height({{carol}}) == 0, 7357000180104);
        assert(LibraAccount::balance<GAS>({{carol}}) == 49991, 7357000180106);
        assert(MinerState::test_helper_get_height({{carol}}) == 0, 7357000180108);
    }
}
// check: EXECUTED

//! new-transaction
//! sender: libraroot
script {
    use 0x1::Vector;
    use 0x1::Stats;
    use 0x1::FullnodeState;
    // This is the the epoch boundary.
    fun main(vm: &signer) {
        // This is not an onboarding case, steady state.
        FullnodeState::test_set_fullnode_fixtures(vm, {{carol}}, 0, 0, 0, 200, 200, 1000000);

        let voters = Vector::empty<address>();
        Vector::push_back<address>(&mut voters, {{alice}});
        Vector::push_back<address>(&mut voters, {{bob}});

        // Case 3 SKIP CAROL, did not validate.

        Vector::push_back<address>(&mut voters, {{dave}});
        Vector::push_back<address>(&mut voters, {{eve}});
        Vector::push_back<address>(&mut voters, {{frank}});


        // Overwrite the statistics to mock that all have been validating.
        let i = 1;
        while (i < 16) {
            // Mock the validator doing work for 15 blocks, and stats being updated.
                
            Stats::process_set_votes(vm, &voters);
            i = i + 1;
        };
    }
}

//! new-transaction
//! sender: libraroot
script {
    use 0x1::Cases;
    
    fun main(vm: &signer) {
        // We are in a new epoch.
        // Check carol is in the the correct case during reconfigure
        assert(Cases::get_case(vm, {{carol}}, 0, 15) == 3, 7357000180109);
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
//! sender: libraroot
script {
    use 0x1::LibraSystem;
    use 0x1::NodeWeight;
    use 0x1::GAS::GAS;
    use 0x1::LibraAccount;
    use 0x1::LibraConfig;

    fun main(_account: &signer) {
        // We are in a new epoch.

        // Check the validator set is at expected size
        assert(LibraSystem::validator_set_size() == 5, 7357000180110);
        assert(LibraSystem::is_validator({{carol}}) == false, 7357000180111);
        assert(LibraAccount::balance<GAS>({{carol}}) == 49991, 7357000180112);
        assert(NodeWeight::proof_of_weight({{carol}}) == 1, 7357000180113);  
        assert(LibraConfig::get_current_epoch()==2, 7357000180114);
    }
}
//check: EXECUTED