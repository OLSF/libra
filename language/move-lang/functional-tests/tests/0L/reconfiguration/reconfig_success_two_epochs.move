// This test is to check if two epochs succesfully happen with all validators being CASE 1.


//! account: alice, 1000000, 0, validator
//! account: bob, 1000000, 0, validator
//! account: carol, 1000000, 0, validator
//! account: dave, 1000000, 0, validator
//! account: eve, 1000000, 0, validator
//! account: frank, 1000000, 0, validator


//! block-prologue
//! proposer: alice
//! block-time: 1

//! new-transaction
//! sender: alice
script {
    use 0x1::MinerState;

    fun main(sender: &signer) {
        // Alice is the only one that can update her mining stats. Hence this first transaction.

        MinerState::test_helper_mock_mining(sender, 5);
        assert(MinerState::get_count_in_epoch({{alice}}) == 5, 7357008013001);
    }
}
//check: EXECUTED

//! new-transaction
//! sender: bob
script {
    use 0x1::MinerState;

    fun main(sender: &signer) {
        // Alice is the only one that can update her mining stats. Hence this first transaction.

        MinerState::test_helper_mock_mining(sender, 5);
        assert(MinerState::get_count_in_epoch({{bob}}) == 5, 7357008013002);
    }
}
//check: EXECUTED


//! new-transaction
//! sender: carol
script {
    use 0x1::MinerState;

    fun main(sender: &signer) {
        // Alice is the only one that can update her mining stats. Hence this first transaction.

        MinerState::test_helper_mock_mining(sender, 5);
        assert(MinerState::get_count_in_epoch({{carol}}) == 5, 7357008013003);
    }
}
//check: EXECUTED


//! new-transaction
//! sender: dave
script {
    use 0x1::MinerState;

    fun main(sender: &signer) {
        MinerState::test_helper_mock_mining(sender, 5);
        assert(MinerState::get_count_in_epoch({{dave}}) == 5, 7357008013004);
    }
}
// //check: EXECUTED

//! new-transaction
//! sender: eve
script {
    use 0x1::MinerState;
    fun main(sender: &signer) {
        MinerState::test_helper_mock_mining(sender, 5);
        assert(MinerState::get_count_in_epoch({{eve}}) == 5, 7357008013005);
    }
}
//check: EXECUTED

//! new-transaction
//! sender: frank
script {
    use 0x1::MinerState;

    fun main(sender: &signer) {
        MinerState::test_helper_mock_mining(sender, 5);
        assert(MinerState::get_count_in_epoch({{frank}}) == 5, 7357008013006);
    }
}
//check: EXECUTED

//! new-transaction
//! sender: libraroot
script {
    use 0x1::Vector;
    use 0x1::Stats;
    use 0x1::LibraSystem;
    

    fun main(vm: &signer) {
        assert(LibraSystem::validator_set_size() == 6, 7357008013007);

        let voters = Vector::empty<address>();
        Vector::push_back<address>(&mut voters, {{alice}});
        Vector::push_back<address>(&mut voters, {{bob}});
        Vector::push_back<address>(&mut voters, {{carol}});
        Vector::push_back<address>(&mut voters, {{dave}});
        Vector::push_back<address>(&mut voters, {{eve}});
        Vector::push_back<address>(&mut voters, {{frank}});

        let i = 1;
        while (i < 16) {
            // Mock the validator doing work for 15 blocks, and stats being updated.
            Stats::process_set_votes(vm, &voters);
            i = i + 1;
        };
    }
}
// check: EXECUTED

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
//! sender: alice
script {
    use 0x1::MinerState;

    fun main(sender: &signer) {
        // Alice is the only one that can update her mining stats. Hence this first transaction.

        MinerState::test_helper_mock_mining(sender, 5);
        assert(MinerState::get_count_in_epoch({{alice}}) == 5, 7357008013008);
    }
}
//check: EXECUTED

//! new-transaction
//! sender: bob
script {
    use 0x1::MinerState;

    fun main(sender: &signer) {
        // Alice is the only one that can update her mining stats. Hence this first transaction.

        MinerState::test_helper_mock_mining(sender, 5);
        assert(MinerState::get_count_in_epoch({{bob}}) == 5, 7357008013009);
    }
}
//check: EXECUTED


//! new-transaction
//! sender: carol
script {
    use 0x1::MinerState;

    fun main(sender: &signer) {
        // Alice is the only one that can update her mining stats. Hence this first transaction.

        MinerState::test_helper_mock_mining(sender, 5);
        assert(MinerState::get_count_in_epoch({{carol}}) == 5, 7357008013010);
    }
}
//check: EXECUTED


//! new-transaction
//! sender: dave
script {
    use 0x1::MinerState;

    fun main(sender: &signer) {
        MinerState::test_helper_mock_mining(sender, 5);
        assert(MinerState::get_count_in_epoch({{dave}}) == 5, 7357008013011);
    }
}
// //check: EXECUTED

//! new-transaction
//! sender: eve
script {
    use 0x1::MinerState;
    fun main(sender: &signer) {
        MinerState::test_helper_mock_mining(sender, 5);
        assert(MinerState::get_count_in_epoch({{eve}}) == 5, 7357008013012);
    }
}
//check: EXECUTED

//! new-transaction
//! sender: frank
script {
    use 0x1::MinerState;

    fun main(sender: &signer) {
        MinerState::test_helper_mock_mining(sender, 5);
        assert(MinerState::get_count_in_epoch({{frank}}) == 5, 7357008013013);
    }
}
//check: EXECUTED



//! new-transaction
//! sender: libraroot
script {
    use 0x1::Vector;
    use 0x1::Stats;

    fun main(vm: &signer) {
        let voters = Vector::empty<address>();
        Vector::push_back<address>(&mut voters, {{alice}});
        Vector::push_back<address>(&mut voters, {{bob}});
        Vector::push_back<address>(&mut voters, {{carol}});
        Vector::push_back<address>(&mut voters, {{dave}});
        Vector::push_back<address>(&mut voters, {{eve}});
        Vector::push_back<address>(&mut voters, {{frank}});


        let i = 16;
        while (i < 31) {
            // Mock the validator doing work for 15 blocks, and stats being updated.
            Stats::process_set_votes(vm, &voters);
            i = i + 1;
        };
    }
}
// check: EXECUTED

///////////////////////////////////////////////
///// Trigger reconfiguration at 4 seconds ////
//! block-prologue
//! proposer: alice
//! block-time: 122000000
//! round: 15

///// TEST RECONFIGURATION IS HAPPENING ////
// check: NewEpochEvent
//////////////////////////////////////////////



//! new-transaction
//! sender: libraroot
script {
    
    use 0x1::LibraSystem;
    use 0x1::LibraConfig;
    fun main(_account: &signer) {
        assert(LibraSystem::validator_set_size() == 6, 7357008013014);
        assert(LibraConfig::get_current_epoch() == 3, 7357008013015);
    }
}
// check: EXECUTED