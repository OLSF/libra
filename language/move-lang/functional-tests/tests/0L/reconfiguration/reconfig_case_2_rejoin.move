// Testing if FRANK a CASE 2 Validator gets rejoined after doing work

// ALICE is CASE 1
//! account: alice, 1000000, 0, validator
// BOB is CASE 1
//! account: bob, 1000000, 0, validator
// CAROL is CASE 1
//! account: carol, 1000000, 0, validator
// DAVE is CASE 1
//! account: dave, 1000000, 0, validator
// EVE is CASE 1
//! account: eve, 1000000, 0, validator
// FRANK is CASE 2
//! account: frank, 1000000, 0, validator

//! block-prologue
//! proposer: alice
//! block-time: 1
//! NewBlockEvent

//! new-transaction
//! sender: alice
script {
    use 0x1::MinerState;

    fun main(sender: signer) {
        // Miner is the only one that can update their mining stats. Hence this first transaction.

        MinerState::test_helper_mock_mining(sender, 5);
        assert(MinerState::test_helper_get_count({{alice}}) == 5, 7357300101011000);
    }
}
//check: EXECUTED

//! new-transaction
//! sender: bob
script {
    use 0x1::MinerState;

    fun main(sender: signer) {
        // Miner is the only one that can update their mining stats. Hence this first transaction.

        MinerState::test_helper_mock_mining(sender, 5);
        assert(MinerState::test_helper_get_count({{bob}}) == 5, 7357300101011000);
    }
}
//check: EXECUTED

//! new-transaction
//! sender: carol
script {
    use 0x1::MinerState;

    fun main(sender: signer) {
        // Miner is the only one that can update their mining stats. Hence this first transaction.

        MinerState::test_helper_mock_mining(sender, 5);
        assert(MinerState::test_helper_get_count({{carol}}) == 5, 7357300101011000);
    }
}
//check: EXECUTED

//! new-transaction
//! sender: dave
script {
    use 0x1::MinerState;

    fun main(sender: signer) {
        // Miner is the only one that can update their mining stats. Hence this first transaction.

        MinerState::test_helper_mock_mining(sender, 5);
        assert(MinerState::test_helper_get_count({{dave}}) == 5, 7357300101011000);
    }
}
//check: EXECUTED

//! new-transaction
//! sender: eve
script {
    use 0x1::MinerState;

    fun main(sender: signer) {
        // Miner is the only one that can update their mining stats. Hence this first transaction.

        MinerState::test_helper_mock_mining(sender, 5);
        assert(MinerState::test_helper_get_count({{eve}}) == 5, 7357300101011000);
    }
}
//check: EXECUTED

//! new-transaction
//! sender: diemroot
script {
    use 0x1::Stats;
    use 0x1::Vector;
    use 0x1::DiemSystem;

    fun main(vm: signer) {
        let voters = Vector::singleton<address>({{alice}});
        Vector::push_back<address>(&mut voters, {{bob}});
        Vector::push_back<address>(&mut voters, {{carol}});
        Vector::push_back<address>(&mut voters, {{dave}});
        Vector::push_back<address>(&mut voters, {{eve}});
        Vector::push_back<address>(&mut voters, {{frank}});

        let i = 1;
        while (i < 15) {
            // Mock the validator doing work for 15 blocks, and stats being updated.
            Stats::process_set_votes(vm, &voters);
            i = i + 1;
        };

        assert(DiemSystem::validator_set_size() == 6, 7357000180101);
        assert(DiemSystem::is_validator({{alice}}) == true, 7357000180102);
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
    use 0x1::Debug::print;

    fun main(_account: signer) {
        // We are in a new epoch.
        assert(DiemConfig::get_current_epoch() == 2, 7357180107);
        print(&DiemSystem::validator_set_size());
        // Tests on initial size of validators 
        assert(DiemSystem::validator_set_size() == 5, 7357180207);
        assert(DiemSystem::is_validator({{frank}}) == false, 7357180307);
    }
}
//check: EXECUTED


//! new-transaction
//! sender: diemroot
script {
    use 0x1::DiemSystem;
    use 0x1::DiemConfig;
    fun main(_account: signer) {
        // We are in a new epoch.
        assert(DiemConfig::get_current_epoch() == 2, 7357180105011000);
        // Tests on initial size of validators 
        assert(DiemSystem::validator_set_size() == 5, 7357180105021000);
        assert(DiemSystem::is_validator({{frank}})
         == false, 7357180105031000);
    }
}
//check: EXECUTED

//! new-transaction
//! sender: diemroot
script {
    // use 0x1::Reconfigure;
    use 0x1::Cases;
    use 0x1::Vector;
    use 0x1::Stats;
    

    fun main(vm: signer) {
        // start a new epoch.
        // Everyone except EVE validates, because she was jailed, not in validator set.
        let voters = Vector::singleton<address>({{alice}});
        Vector::push_back<address>(&mut voters, {{bob}});
        Vector::push_back<address>(&mut voters, {{carol}});
        Vector::push_back<address>(&mut voters, {{dave}});
        // Vector::push_back<address>(&mut voters, {{eve}});
        Vector::push_back<address>(&mut voters, {{frank}});

        let i = 1;
        while (i < 15) {
            // Mock the validator doing work for 15 blocks, and stats being updated.
            Stats::process_set_votes(vm, &voters);
            i = i + 1;
        };

        // Even though Eve will be considered a case 2, it was because she was jailed. She will rejoin next epoch.
        assert(Cases::get_case(vm, {{eve}}, 0, 15) == 2, 7357180106011000);
        // Reconfigure::reconfigure(vm, 30);
    }
}
//check: EXECUTED


//! new-transaction
//! sender: alice
script {
    use 0x1::MinerState;

    fun main(sender: signer) {
        // Alice is the only one that can update her mining stats. Hence this first transaction.

        MinerState::test_helper_mock_mining(sender, 5);
        assert(MinerState::test_helper_get_count({{alice}}) == 5, 7357180101011000);
    }
}
//check: EXECUTED

//! new-transaction
//! sender: bob
script {
    use 0x1::MinerState;

    fun main(sender: signer) {
        // Miner is the only one that can update their mining stats. Hence this first transaction.

        MinerState::test_helper_mock_mining(sender, 5);
        assert(MinerState::test_helper_get_count({{bob}}) == 5, 7357300101011000);
    }
}
//check: EXECUTED

//! new-transaction
//! sender: carol
script {
    use 0x1::MinerState;

    fun main(sender: signer) {
        // Miner is the only one that can update their mining stats. Hence this first transaction.

        MinerState::test_helper_mock_mining(sender, 5);
        assert(MinerState::test_helper_get_count({{carol}}) == 5, 7357300101011000);
    }
}
//check: EXECUTED

//! new-transaction
//! sender: dave
script {
    use 0x1::MinerState;

    fun main(sender: signer) {
        // Miner is the only one that can update their mining stats. Hence this first transaction.

        MinerState::test_helper_mock_mining(sender, 5);
        assert(MinerState::test_helper_get_count({{dave}}) == 5, 7357300101011000);
    }
}
//check: EXECUTED

//! new-transaction
//! sender: eve
script {
    use 0x1::MinerState;

    fun main(sender: signer) {
        // Alice is the only one that can update her mining stats. Hence this first transaction.

        MinerState::test_helper_mock_mining(sender, 5);
        assert(MinerState::test_helper_get_count({{eve}}) == 5, 7357180102011000);
    }
}
//check: EXECUTED

//! new-transaction
//! sender: frank
script {
    use 0x1::MinerState;

    fun main(sender: signer) {
        // Alice is the only one that can update her mining stats. Hence this first transaction.

        MinerState::test_helper_mock_mining(sender, 5);
        assert(MinerState::test_helper_get_count({{frank}}) == 5, 7357180102011000);
    }
}
//check: EXECUTED

///////////////////////////////////////////////
///// Trigger reconfiguration at 4 seconds ////
//! block-prologue
//! proposer: alice
//! block-time: 122000000
//! round: 30

///// TEST RECONFIGURATION IS HAPPENING ////
// check: NewEpochEvent
//////////////////////////////////////////////

//! new-transaction
//! sender: diemroot
script {
    use 0x1::DiemSystem;
    use 0x1::DiemConfig;
    fun main(_account: signer) {
        assert(DiemConfig::get_current_epoch() == 3, 7357180107011000);
        assert(DiemSystem::validator_set_size() == 6, 7357180105021000);
        assert(DiemSystem::is_validator({{frank}}), 7357180107031000);
    }
}
//check: EXECUTED
