//! account: bob, 2000002, 0, validator

//! new-transaction
//! sender: bob

script {
  use 0x1::VDF;
  use 0x1::DiemAccount;
  use 0x1::MinerState;
  use 0x1::TestFixtures;
  use 0x1::Signer;

  // Scenario: Bob, an existing validator, is sending an onboarding 
  // transaction for Eve.
  fun main(sender: signer) {
    let challenge = TestFixtures::eve_0_easy_chal();
    let solution = TestFixtures::eve_0_easy_sol();
    // Parse key and check
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
  }
}
// check: EXECUTED

//! new-transaction

//! sender: diemroot
script {
    use 0x1::Subsidy;
    use 0x1::DiemAccount;
    use 0x1::GAS::GAS;
    use 0x1::Debug::print;
    use 0x1::Reconfigure;
    use 0x1::FullnodeState;

    fun main(vm: signer) {
        let eve_addr = 0x3DC18D1CF61FAAC6AC70E3A63F062E4B;
        let old_account_bal = DiemAccount::balance<GAS>(eve_addr);
        print(&old_account_bal);

        print(&FullnodeState::is_onboarding(eve_addr));
        // Set the auction proof price to 0, so we check if we trigger guaranteed price.
        Subsidy::test_set_fullnode_fixtures(&vm, 0, 0, 0, 0, 0);

        // Fullnode rewards are paid at epoch boundary.
        Reconfigure::reconfigure(&vm, 100);

        let new_account_bal = DiemAccount::balance<GAS>(eve_addr);
        print(&new_account_bal);

        assert(new_account_bal == 1576000, 735702);
        assert(new_account_bal>old_account_bal, 735703);
    }
}
// check: EXECUTED

