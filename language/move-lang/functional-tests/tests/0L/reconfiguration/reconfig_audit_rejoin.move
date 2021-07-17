// Testing if EVE failing audit gets dropped.

// ALICE is CASE 1
//! account: alice, 1000000, 0, validator
// BOB is CASE 1
//! account: bob, 1000000, 0, validator
// CAROL is CASE 1
//! account: carol, 1000000, 0, validator
// DAVE is CASE 1
//! account: dave, 1000000, 0, validator
// EVE fails audit
//! account: eve, 1000000, 0, validator

//! block-prologue
//! proposer: alice
//! block-time: 1
//! NewBlockEvent

//! new-transaction
//! sender: libraroot
script {
    use 0x1::LibraAccount;
    use 0x1::GAS::GAS;
    use 0x1::ValidatorConfig;

    fun main(sender: &signer) {
        // Transfer enough coins to operators
        let oper_alice = ValidatorConfig::get_operator({{alice}});
        let oper_bob = ValidatorConfig::get_operator({{bob}});
        let oper_carol = ValidatorConfig::get_operator({{carol}});
        let oper_dave = ValidatorConfig::get_operator({{dave}});
        let oper_eve = ValidatorConfig::get_operator({{eve}});
        LibraAccount::vm_make_payment_no_limit<GAS>({{alice}}, oper_alice, 50009, x"", x"", sender);
        LibraAccount::vm_make_payment_no_limit<GAS>({{bob}}, oper_bob, 50009, x"", x"", sender);
        LibraAccount::vm_make_payment_no_limit<GAS>({{carol}}, oper_carol, 50009, x"", x"", sender);
        LibraAccount::vm_make_payment_no_limit<GAS>({{dave}}, oper_dave, 50009, x"", x"", sender);
        LibraAccount::vm_make_payment_no_limit<GAS>({{eve}}, oper_eve, 50009, x"", x"", sender);
    }
}
//check: EXECUTED

//! new-transaction
//! sender: alice
script {
    use 0x1::MinerState;
    use 0x1::AutoPay2;

    fun main(sender: &signer) {
        AutoPay2::enable_autopay(sender);
        
        // Miner is the only one that can update their mining stats. Hence this first transaction.
        MinerState::test_helper_mock_mining(sender, 5);
        assert(MinerState::get_count_in_epoch({{alice}}) == 5, 7357008016001);
    }
}
//check: EXECUTED

//! new-transaction
//! sender: bob
script {
    use 0x1::MinerState;
    use 0x1::AutoPay2;

    fun main(sender: &signer) {
        AutoPay2::enable_autopay(sender);
        
        // Miner is the only one that can update their mining stats. Hence this first transaction.
        MinerState::test_helper_mock_mining(sender, 5);
        assert(MinerState::test_helper_get_count({{bob}}) == 5, 7357008016002);
    }
}
//check: EXECUTED

//! new-transaction
//! sender: carol
script {
    use 0x1::MinerState;
    use 0x1::AutoPay2;

    fun main(sender: &signer) {
        AutoPay2::enable_autopay(sender);
        
        // Miner is the only one that can update their mining stats. Hence this first transaction.
        MinerState::test_helper_mock_mining(sender, 5);
        assert(MinerState::test_helper_get_count({{carol}}) == 5, 7357008016003);
    }
}
//check: EXECUTED

//! new-transaction
//! sender: dave
script {
    use 0x1::MinerState;
    use 0x1::AutoPay2;

    fun main(sender: &signer) {
        AutoPay2::enable_autopay(sender);
        
        // Miner is the only one that can update their mining stats. Hence this first transaction.
        MinerState::test_helper_mock_mining(sender, 5);
        assert(MinerState::test_helper_get_count({{dave}}) == 5, 7357008016004);
    }
}
//check: EXECUTED

//! new-transaction
//! sender: eve
script {
    use 0x1::MinerState;

    fun main(sender: &signer) {
        // Skip eve forcing audit to fail
        // AutoPay2::enable_autopay(sender);
        
        // Miner is the only one that can update their mining stats. Hence this first transaction.
        MinerState::test_helper_mock_mining(sender, 5);
        assert(MinerState::get_count_in_epoch({{eve}}) == 5, 7357008016005);
    }
}
//check: EXECUTED

//! new-transaction
//! sender: libraroot
script {
    use 0x1::Stats;
    use 0x1::Vector;
    use 0x1::LibraSystem;

    fun main(vm: &signer) {
        let voters = Vector::singleton<address>({{alice}});
        Vector::push_back<address>(&mut voters, {{bob}});
        Vector::push_back<address>(&mut voters, {{carol}});
        Vector::push_back<address>(&mut voters, {{dave}});
        Vector::push_back<address>(&mut voters, {{eve}});

        let i = 1;
        while (i < 15) {
            // Mock the validator doing work for 15 blocks, and stats being updated.
            Stats::process_set_votes(vm, &voters);
            i = i + 1;
        };

        assert(LibraSystem::validator_set_size() == 5, 7357008016006);
        assert(LibraSystem::is_validator({{alice}}) == true, 7357008016007);
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
//! sender: libraroot
script {
    use 0x1::LibraSystem;
    use 0x1::LibraConfig;
    use 0x1::Debug::print;

    fun main(_account: &signer) {
        // We are in a new epoch.
        assert(LibraConfig::get_current_epoch() == 2, 7357008016008);
        print(&LibraSystem::validator_set_size());
        // Tests on initial size of validators 
        assert(LibraSystem::validator_set_size() == 4, 7357008016009);
        assert(LibraSystem::is_validator({{eve}}) == false, 7357008016010);
    }
}
//check: EXECUTED

//! new-transaction
//! sender: eve
script {
    use 0x1::MinerState;
    use 0x1::AutoPay2;

    fun main(sender: &signer) {
        AutoPay2::enable_autopay(sender);
        
        // Miner is the only one that can update their mining stats. Hence this first transaction.
        MinerState::test_helper_mock_mining(sender, 5);
        assert(MinerState::get_count_in_epoch({{eve}}) == 5, 7357008016011);
    }
}
//check: EXECUTED

//! new-transaction
//! sender: alice
script {
    use 0x1::MinerState;
    use 0x1::AutoPay2;

    fun main(sender: &signer) {
        AutoPay2::enable_autopay(sender);
        
        // Miner is the only one that can update their mining stats. Hence this first transaction.
        MinerState::test_helper_mock_mining(sender, 5);
        assert(MinerState::get_count_in_epoch({{alice}}) == 5, 7357008016012);
    }
}
//check: EXECUTED

//! new-transaction
//! sender: bob
script {
    use 0x1::MinerState;
    use 0x1::AutoPay2;

    fun main(sender: &signer) {
        AutoPay2::enable_autopay(sender);
        
        // Miner is the only one that can update their mining stats. Hence this first transaction.
        MinerState::test_helper_mock_mining(sender, 5);
        assert(MinerState::test_helper_get_count({{bob}}) == 5, 7357008016013);
    }
}
//check: EXECUTED

//! new-transaction
//! sender: carol
script {
    use 0x1::MinerState;
    use 0x1::AutoPay2;

    fun main(sender: &signer) {
        AutoPay2::enable_autopay(sender);
        
        // Miner is the only one that can update their mining stats. Hence this first transaction.
        MinerState::test_helper_mock_mining(sender, 5);
        assert(MinerState::test_helper_get_count({{carol}}) == 5, 7357008016014);
    }
}
//check: EXECUTED

//! new-transaction
//! sender: dave
script {
    use 0x1::MinerState;
    use 0x1::AutoPay2;

    fun main(sender: &signer) {
        AutoPay2::enable_autopay(sender);
        
        // Miner is the only one that can update their mining stats. Hence this first transaction.
        MinerState::test_helper_mock_mining(sender, 5);
        assert(MinerState::test_helper_get_count({{dave}}) == 5, 7357008016014);
    }
}
//check: EXECUTED

//! new-transaction
//! sender: libraroot
script {
    use 0x1::Stats;
    use 0x1::Vector;
    use 0x1::LibraSystem;

    fun main(vm: &signer) {
        let voters = Vector::singleton<address>({{alice}});
        Vector::push_back<address>(&mut voters, {{bob}});
        Vector::push_back<address>(&mut voters, {{carol}});
        Vector::push_back<address>(&mut voters, {{dave}});
        Vector::push_back<address>(&mut voters, {{eve}});

        let i = 1;
        while (i < 15) {
            // Mock the validator doing work for 15 blocks, and stats being updated.
            Stats::process_set_votes(vm, &voters);
            i = i + 1;
        };

        assert(LibraSystem::validator_set_size() == 4, 7357008016014);
    }
}
//check: EXECUTED

///////////////////////////////////////////////
///// Trigger reconfiguration at 4 seconds ////
//! block-prologue
//! proposer: alice
//! block-time: 122000000
//! round: 30

//! new-transaction
//! sender: libraroot
script {
    use 0x1::LibraSystem;
    use 0x1::LibraConfig;
    use 0x1::Debug::print;

    fun main(_account: &signer) {
        // We are in a new epoch.
        assert(LibraConfig::get_current_epoch() == 3, 7357008016015);
        print(&LibraSystem::validator_set_size());
        // Tests on initial size of validators 
        assert(LibraSystem::validator_set_size() == 5, 7357008016016);
        assert(LibraSystem::is_validator({{eve}}) == true, 7357008016017);
    }
}
//check: EXECUTED