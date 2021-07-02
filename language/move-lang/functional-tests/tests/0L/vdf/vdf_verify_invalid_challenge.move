// NOTE: TEST SETUP: If you add a "validator", functional tests will add 
// only that validator to genesis.
// By default 0L tests load 3 random validators on genesis. This is slow. 
// So adding a dummy validator will only run the initialize_miners once 
// instead of three times, and speeds up testing.

//! new-transaction
script{
  use 0x1::VDF;
  use 0x1::TestFixtures;
  fun main() {
    // This checks that the VDF verifier catches an invalide "challenge" 
    // parameter, and fails gracefully with error.

    let difficulty = 100;
    // incorrect challenge.
    let challenge = x"bb";
    let solution = TestFixtures::easy_sol();

    assert(VDF::verify(&challenge, &difficulty, &solution) ==false, 1);
  }
}