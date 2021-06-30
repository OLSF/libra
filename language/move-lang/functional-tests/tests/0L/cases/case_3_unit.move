//! account: alice, 1, 0, validator

//! new-transaction
//! sender: alice
script {    
    use 0x1::MinerState;

    fun main(sender: signer) {
        // Alice is the only one that can update her mining stats. 
        // Hence this first transaction.

        MinerState::test_helper_mock_mining(&sender, 5);
        assert(MinerState::get_count_in_epoch({{alice}}) == 5, 7357300101011000);
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
        let voters = Vector::singleton<address>({{alice}});
        // only voted on 1 block out of 200
        Stats::process_set_votes(&sender, &voters);

        assert(Cases::get_case(&sender, {{alice}}, 0, 200) == 3, 7357300103011000);
    }
}
//check: EXECUTED