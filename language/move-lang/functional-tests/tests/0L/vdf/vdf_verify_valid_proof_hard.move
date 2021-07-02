//! account: dummy-prevents-genesis-reload, 100000 ,0, validator

//! new-transaction
script{
  use 0x1::VDF;
  use 0x1::TestFixtures;
  fun main() {
    // this tests the happy case, that a proof is submitted with all three 
    // correct parameters.
    
    let difficulty = 5000000;
    let challenge = TestFixtures::hard_chal();
    // Generate solutions with cd ./verfiable_delay/vdf-cli && cargo run -- -l=2048 aa 100
    // the -l=2048 is important because this is the security paramater of 0L miner.
    let solution = TestFixtures::hard_sol();

    assert(VDF::verify(&challenge, &difficulty, &solution) == true, 1);
  }
}
