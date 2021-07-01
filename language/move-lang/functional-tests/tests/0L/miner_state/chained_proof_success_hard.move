//! account: dummy-prevents-genesis-reload, 100000 ,0, validator
//! account: alice, 10000000GAS

// Alice Submit VDF Proof
//! new-transaction
//! sender: alice
script {
    use 0x1::MinerState;
    use 0x1::TestFixtures;

    // SIMULATES A MINER ONBOARDING PROOF (block_0.json)
    fun main(sender: signer) {
        let difficulty = 5000000;
        let height_after = 0;

        // return solution
        MinerState::test_helper(
            &sender,
            difficulty,
            TestFixtures::alice_0_hard_chal(),
            TestFixtures::alice_0_hard_sol()
        );

        // check for initialized MinerState
        let verified_tower_height_after = MinerState::test_helper_get_height({{alice}});

        assert(verified_tower_height_after == height_after, 10008001);
    }
}
// check: EXECUTED


//! new-transaction
//! sender: alice
script {
    use 0x1::MinerState;
    use 0x1::TestFixtures;

    // SIMULATES THE SECOND PROOF OF THE MINER (block_1.json)
    fun main(sender: signer) {
        let difficulty = 5000000;
        assert(MinerState::test_helper_get_height({{alice}}) == 0, 10008001);
        let height_after = 1;
        
        let proof = MinerState::create_proof_blob(
            TestFixtures::alice_1_hard_chal(),
            difficulty,
            TestFixtures::alice_1_hard_sol()
        );
        MinerState::commit_state(&sender, proof);

        let verified_height = MinerState::test_helper_get_height({{alice}});
        assert(verified_height == height_after, 10008002);
    }
}
// check: EXECUTED
