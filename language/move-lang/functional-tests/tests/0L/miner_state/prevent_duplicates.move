//! account: dummy-prevents-genesis-reload, 100000 ,0, validator

// Prepare the state for the next test.
// Bob Submits a CORRECT VDF Proof, and that updates the state.
//! account: alice, 10000000GAS
//! account: bob, 10000000GAS
//! new-transaction
//! sender: bob
script {
    use 0x1::MinerState;
    use 0x1::TestFixtures;

    fun main(sender: signer) {
        MinerState::test_helper(
            &sender,
            100u64, //difficulty
            TestFixtures::easy_chal(),
            TestFixtures::easy_sol()
        );

        let height = MinerState::test_helper_get_height({{bob}});
        assert(height==0, 01);

    }
}
// check: EXECUTED

// Bob Submit the Duplicated CORRECT VDF Proof, which he just sent.
//! new-transaction
//! sender: bob
script {
    use 0x1::MinerState;
    use 0x1::TestFixtures;

    fun main(sender: signer) {
        let difficulty = 100;
        let proof = MinerState::create_proof_blob(
            TestFixtures::easy_chal(),
            difficulty,
            TestFixtures::easy_sol()
        );
        MinerState::commit_state(&sender, proof);
    }
}
// check: VMExecutionFailure(ABORTED { code: 130103