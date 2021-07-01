//! account: bob, 10000000, 0, validator

//! new-transaction
//! sender: bob
script {
use 0x1::VDF;
use 0x1::DiemAccount;
use 0x1::GAS::GAS;
use 0x1::MinerState;
use 0x1::NodeWeight;
use 0x1::TestFixtures;
use 0x1::ValidatorConfig;
use 0x1::Roles;
use 0x1::Signer;
// Test Prefix: 1301

  fun main(sender: signer) {
    // Scenario: Bob, an existing validator, is sending a transaction for Eve, 
    // with a challenge and proof not yet submitted to the chain.
    let challenge = TestFixtures::eve_0_easy_chal();
    let solution = TestFixtures::eve_0_easy_sol();
    // // Parse key and check
    let (eve_addr, _auth_key) = VDF::extract_address_from_challenge(&challenge);
    assert(eve_addr == 0x3DC18D1CF61FAAC6AC70E3A63F062E4B, 401);
    
    let sender_addr = Signer::address_of(&sender);
    let epochs_since_creation = 10;
    MinerState::test_helper_set_rate_limit(sender_addr, epochs_since_creation);

    DiemAccount::create_validator_account_with_proof(
        &sender,
        &challenge,
        &solution,
        b"leet",
        0xfa72817f1b5aab94658238ddcdc08010,
        x"fa72817f1b5aab94658238ddcdc08010",
        // random consensus_pubkey: vector<u8>,
        x"8108aedfacf5cf1d73c67b6936397ba5fa72817f1b5aab94658238ddcdc08010",
        b"192.168.0.1", // validator_network_addresses: vector<u8>,
        b"192.168.0.1", // fullnode_network_addresses: vector<u8>,
        x"1ee7", // human_name: vector<u8>,
    );

    assert(Roles::assert_validator_addr(eve_addr), 7357130101011000);
    assert(
      Roles::assert_validator_operator_addr(0xfa72817f1b5aab94658238ddcdc08010), 
      7357130101021000
    );

    assert(ValidatorConfig::is_valid(eve_addr), 7357130101031000);
    assert(
      ValidatorConfig::get_operator(eve_addr) == 0xfa72817f1b5aab94658238ddcdc08010, 
      7357130101041000
    );
    
    let config = ValidatorConfig::get_config(eve_addr);
    let consensus_pubkey = ValidatorConfig::get_consensus_pubkey(&config);
    assert(
      consensus_pubkey == &x"8108aedfacf5cf1d73c67b6936397ba5fa72817f1b5aab94658238ddcdc08010", 
      7357130101051000
    );

    assert(MinerState::test_helper_get_height(eve_addr) == 0, 7357130101061000);

    //Check the validator is in the validator universe.
    assert(NodeWeight::proof_of_weight(eve_addr) == 0, 7357130101071000);

    // Check the account exists and the balance is 10, from Bob's onboarding transfer
    assert(DiemAccount::balance<GAS>(eve_addr) == 1000000, 7357130101081000);

    // // assert the operator has balance
    assert(
      DiemAccount::balance<GAS>(0xfa72817f1b5aab94658238ddcdc08010) == 1000000, 
      7357130101091000
    );

    // // Bob's balance should have gone down by 2M gas (operator and owner)
    assert(DiemAccount::balance<GAS>({{bob}}) == 8000000, 73571301011000);

  }
}
// check: EXECUTED


//! new-transaction
//! sender: diemroot
script {
  use 0x1::DiemAccount;
  use 0x1::GAS::GAS;
  use 0x1::Reconfigure;
  use 0x1::MinerState;
  use 0x1::Testnet;
  
  fun main(vm: signer) {
      // need to remove testnet for this test, since testnet does not ratelimit account creation.
      Testnet::remove_testnet(&vm); 

      let eve = 0x3DC18D1CF61FAAC6AC70E3A63F062E4B;
      let old_account_bal = DiemAccount::balance<GAS>(eve);
      let old_account_bal_oper = DiemAccount::balance<GAS>(0xfa72817f1b5aab94658238ddcdc08010);

      Reconfigure::reconfigure(&vm, 100);
      let new_account_bal = DiemAccount::balance<GAS>(eve);

      assert(old_account_bal == 1000000, 7357001);
      assert(new_account_bal == 3497536, 7357002);

      // Operator account should not increase after epoch change
      assert(
        DiemAccount::balance<GAS>(0xfa72817f1b5aab94658238ddcdc08010) == old_account_bal_oper, 
        7357003
      );

      assert(MinerState::can_create_val_account({{bob}}) == false, 7357004);
      
  }
}