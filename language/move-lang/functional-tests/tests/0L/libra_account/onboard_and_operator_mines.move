// Module to test bulk validator updates function in DiemSystem.move
//! account: alice, 3000000, 0, validator
// NOTE: enough balance for onboarding gas transfer.

//! new-transaction
//! sender: alice

script {
use 0x1::Signer;
use 0x1::VDF;
use 0x1::TestFixtures;
use 0x1::MinerState;
use 0x1::DiemAccount;

// Test Prefix: 1301

fun main(alice_sig: signer) {
  let challenge = TestFixtures::eve_0_easy_chal();
  let solution = TestFixtures::eve_0_easy_sol();
  // // Parse key and check
  let (eve_addr, _auth_key) = VDF::extract_address_from_challenge(&challenge);
  assert(eve_addr == 0x3DC18D1CF61FAAC6AC70E3A63F062E4B, 401);
  
  let sender_addr = Signer::address_of(&alice_sig);
  let epochs_since_creation = 10;
  MinerState::test_helper_set_rate_limit(sender_addr, epochs_since_creation);

  DiemAccount::create_validator_account_with_proof(
      &alice_sig,
      &challenge,
      &solution,
      b"leet",
      0xfa72817f1b5aab94658238ddcdc08010,
      x"fa72817f1b5aab94658238ddcdc08010",
      x"8108aedfacf5cf1d73c67b6936397ba5fa72817f1b5aab94658238ddcdc08010", // random consensus_pubkey: vector<u8>,
      b"192.168.0.1", // validator_network_addresses: vector<u8>,
      b"192.168.0.1", // fullnode_network_addresses: vector<u8>,
      x"1ee7", // human_name: vector<u8>,
  );
}
}
// check: EXECUTED

//! new-transaction
//! sender: diemroot
script {
use 0x1::MinerState;
use 0x1::TestFixtures;

// SIMULATES A MINER ONBOARDING PROOF (block_0.json)
fun main(_: signer) { 
  let eve = 0x3DC18D1CF61FAAC6AC70E3A63F062E4B;
  let oper = 0xfa72817f1b5aab94658238ddcdc08010;

    let proof = MinerState::create_proof_blob(
      TestFixtures::eve_1_easy_chal(),
      100, // difficulty
      TestFixtures::eve_1_easy_sol(),
    );
    MinerState::test_helper_operator_submits(
      oper,
      eve,
      proof
    );

    // // check for initialized MinerState
    let verified_tower_height_after = MinerState::test_helper_get_height(eve);

    assert(verified_tower_height_after > 0, 10008001);

}
}
// check: EXECUTED