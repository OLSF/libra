//! account: alice, 1, 0, validator

//! new-transaction
//! sender: alice
script {
    use 0x1::MinerState;
    // use 0x1::Signer;

    fun main(sender: signer) {
        // MinerState::init_miner_state(sender);
        MinerState::test_helper_mock_mining(&sender, 5);
    }
}
//check: EXECUTED

//! new-transaction
//! sender: diemroot
script {
    use 0x1::MinerState;
    
    fun main(sender: signer) {
        assert(MinerState::get_count_in_epoch({{alice}}) == 5, 73570001);
        MinerState::test_helper_mock_reconfig(&sender, {{alice}});
        assert(MinerState::get_epochs_mining({{alice}}) == 1, 73570002);
    }
}
//check: EXECUTED