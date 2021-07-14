//! account: bob, 100000, 0

//! new-transaction
//! sender: bob
script {
use 0x1::LibraAccount;
use 0x1::TestFixtures;
use 0x1::GAS::GAS;
use 0x1::Wallet;

fun main(_sender: &signer) {
  // Scenario: Bob, an existing user, is sending a transaction for Eve, with a challenge and proof not yet submitted to the chain.
  // This proof will create a new account, with the preimage data.
  let challenge = TestFixtures::eve_0_easy_chal();
  let solution = TestFixtures::eve_0_easy_sol();
  
  let eve_addr = LibraAccount::create_user_account_with_proof(
    &challenge,
    &solution,
  );

  assert(LibraAccount::balance<GAS>(eve_addr) == 0, 7357130101081000);
  assert(Wallet::is_slow(eve_addr), 7357130101081001);
}
}
// check: EXECUTED
