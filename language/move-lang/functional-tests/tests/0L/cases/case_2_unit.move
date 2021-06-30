//! account: alice, 1, 0, validator

//! new-transaction
//! sender: alice
script {
    use 0x1::MinerState;

    fun main(sender: signer) {
        // Alice is the only one that can update her mining stats. 
        // Hence this first transaction.

        MinerState::test_helper_mock_mining(&sender, 0);
        assert(MinerState::get_count_in_epoch({{alice}}) == 0, 7357300101011000);
    }
}
//check: EXECUTED


//! new-transaction
//! sender: diemroot
script {
    use 0x1::Stats;
    use 0x1::Vector;
    use 0x1::Cases;
    
    fun main(sender: signer) {
        // todo: change name to Mock epochs
        // MinerState::test_helper_set_epochs(sender, 5);
        let voters = Vector::singleton<address>({{alice}});
        let i = 1;
        while (i < 16) {
            // Mock the validator doing work for 15 blocks, and stats being updated.
            Stats::process_set_votes(&sender, &voters);
            i = i + 1;
        };

        assert(Cases::get_case(&sender, {{alice}}, 0 , 15) == 2, 7357300103011000);
    }
}
//check: EXECUTED

