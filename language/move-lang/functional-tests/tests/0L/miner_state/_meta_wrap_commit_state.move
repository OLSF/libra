//! account: dummy-prevents-genesis-reload, 100000 ,0, validator

// Prepare the state for the next test.
// Bob Submits a CORRECT VDF Proof, and that updates the state.
//! account: alice, 10000000GAS
//! account: bob, 10000000GAS
//! new-transaction
//! sender: bob
script {
use 0x1::MinerState;
// ;
use 0x1::TestFixtures;

fun main(sender: signer) {

    // Testing that state can be initialized, and a proof submitted as if it were genesis.
    // buildign block for other tests.
    MinerState::test_helper(
        sender,
        100u64, // difficulty
        TestFixtures::easy_chal(),
        TestFixtures::easy_sol()
    );

    let height = MinerState::test_helper_get_height({{bob}});
    assert(height==0, 01);

}
}
// check: EXECUTED
