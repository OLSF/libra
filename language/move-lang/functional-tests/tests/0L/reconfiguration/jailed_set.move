// Test that both cases 3 and 4 are jailed.

//! account: alice, 1000000, 0, validator
//! account: bob, 1000000, 0, validator
//! account: carol, 1000000, 0, validator
//! account: dave, 1000000, 0, validator
//! account: eve, 1000000, 0, validator
//! account: frank, 1000000, 0, validator

//! block-prologue
//! proposer: alice
//! block-time: 1
//! NewBlockEvent

//! new-transaction
//! sender: alice
script {

    use 0x1::MinerState;

    fun main(sender: &signer) {
        // Alice mines (case 1)
        // "Sender" is the only one that can update her mining stats. Hence this first transaction.

        MinerState::test_helper_mock_mining(sender, 5);
        assert(MinerState::get_count_in_epoch({{alice}}) == 5, 7357008003001);
    }
}
//check: EXECUTED

//! new-transaction
//! sender: eve
script {
    use 0x1::MinerState;
    fun main(sender: &signer) {
        // Eve mines (case 3)
        MinerState::test_helper_mock_mining(sender, 5);
        assert(MinerState::get_count_in_epoch({{eve}}) == 5, 7357008003002);
    }
}
//check: EXECUTED

//! new-transaction
//! sender: libraroot
script {
    use 0x1::Stats;
    use 0x1::Vector;
    use 0x1::Cases;
    use 0x1::LibraSystem;

    fun main(vm: &signer) {
        let voters = Vector::singleton<address>({{alice}});
        let i = 1;
        while (i < 15) {
            // Mock the validator doing work for 15 blocks, and stats being updated.
            Stats::process_set_votes(vm, &voters);
            i = i + 1;
        };

        assert(Cases::get_case(vm, {{alice}}, 0, 15) == 1, 7357008003003);
        assert(Cases::get_case(vm, {{eve}}, 0, 15) == 3, 7357008003004);
        assert(Cases::get_case(vm, {{frank}}, 0, 15) == 4, 7357008003005);

        let jailed = LibraSystem::get_jailed_set(vm, 0, 15);
        assert(Vector::length<address>(&jailed) == 5, 7357008003006);
    }
}
//check: EXECUTED