//! account: alice, 4000000, 0, validator
//! account: bob, 1000000, 0, validator
//! account: carol, 1000000, 0, validator
//! account: dave, 1000000, 0, validator

//! new-transaction
//! sender: alice
script {
  use 0x1::DiemAccount;
  use 0x1::ValidatorConfig;
  use 0x1::TestFixtures;
  use 0x1::VDF;
  use 0x1::Signer;
  use 0x1::MinerState;

  fun main(sender: signer) {
    // Scenario: Alice, an existing validator, is sending a transaction for Eve, with a challenge and proof from eve's block_0
    let challenge = TestFixtures::eve_0_easy_chal();
    let solution = TestFixtures::eve_0_easy_sol();
    // // Parse key and check
    let (eve_addr, _auth_key) = VDF::extract_address_from_challenge(&challenge);
    assert(eve_addr == 0x3DC18D1CF61FAAC6AC70E3A63F062E4B, 401);

    let sender_addr = Signer::address_of(sender);
    let epochs_since_creation = 10;
    MinerState::test_helper_set_rate_limit(sender_addr, epochs_since_creation);

    DiemAccount::create_validator_account_with_proof(
        sender,
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
    assert(ValidatorConfig::is_valid(eve_addr), 7357130101031000);

  }
}
//check: EXECUTED


//! new-transaction
//! sender: diemroot
script {
    use 0x1::DiemSystem;
    use 0x1::Reconfigure;
    use 0x1::Vector;
    use 0x1::MinerState;
    use 0x1::Stats;

    fun main(vm: signer) {
        // Tests on initial size of validators
        assert(DiemSystem::validator_set_size() == 4, 7357000180101);
        assert(DiemSystem::is_validator({{alice}}) == true, 7357000180102);
        assert(DiemSystem::is_validator({{bob}}) == true, 7357000180103);
        assert(DiemSystem::is_validator(0x3DC18D1CF61FAAC6AC70E3A63F062E4B) == false, 7357000180104);

        // Mock everyone being a CASE 1
        let voters = Vector::empty<address>();
        Vector::push_back<address>(&mut voters, {{alice}});
        Vector::push_back<address>(&mut voters, {{bob}});
        Vector::push_back<address>(&mut voters, {{carol}});
        Vector::push_back<address>(&mut voters, {{dave}});


        MinerState::test_helper_mock_mining_vm(vm, {{alice}}, 20);
        MinerState::test_helper_mock_mining_vm(vm, {{bob}}, 20);
        MinerState::test_helper_mock_mining_vm(vm, {{carol}}, 20);
        MinerState::test_helper_mock_mining_vm(vm, {{dave}}, 20);

        let i = 1;
        while (i < 16) {
            // Mock the validator doing work for 15 blocks, and stats being updated.
            Stats::process_set_votes(vm, &voters);
            i = i + 1;
        };

        Reconfigure::reconfigure(vm, 15); // reconfigure at height 15
    }
}
// check: EXECUTED

// Epoch 2 began
// The new node is in validatorUniverse but not in validator set

//! new-transaction
//! sender: diemroot
script {
    use 0x1::DiemSystem;
    use 0x1::ValidatorUniverse;
    use 0x1::Vector;
    fun main(vm: signer) {
        // Tests on initial size of validators
        assert(DiemSystem::validator_set_size() == 4, 7357000180101);
        assert(DiemSystem::is_validator({{alice}}) == true, 7357000180102);
        assert(!DiemSystem::is_validator(0x3DC18D1CF61FAAC6AC70E3A63F062E4B), 7357000180103);
        let len = Vector::length<address>(&ValidatorUniverse::get_eligible_validators(vm));
        assert(len == 4 , 7357000180104);
      }
}
// check: EXECUTED

// The new node starts mining and submiting proofs in the epoch 2
//
//! new-transaction
//! sender: diemroot
script {
    use 0x1::DiemSystem;
    use 0x1::Reconfigure;
    use 0x1::Vector;
    use 0x1::MinerState;
    use 0x1::Stats;
    use 0x1::ValidatorUniverse;

    fun main(vm: signer) {
        // Tests on initial size of validators
        assert(DiemSystem::validator_set_size() == 4, 7357000180101);
        assert(DiemSystem::is_validator({{alice}}) == true, 7357000180102);
        assert(DiemSystem::is_validator({{bob}}) == true, 7357000180103);
        assert(DiemSystem::is_validator(0x3DC18D1CF61FAAC6AC70E3A63F062E4B) == false, 7357000180103);

        // Mock everyone being a CASE 1
        let voters = Vector::empty<address>();
        Vector::push_back<address>(&mut voters, {{alice}});
        Vector::push_back<address>(&mut voters, {{bob}});
        Vector::push_back<address>(&mut voters, {{carol}});
        Vector::push_back<address>(&mut voters, {{dave}});

        MinerState::test_helper_mock_mining_vm(vm, {{alice}}, 20);
        MinerState::test_helper_mock_mining_vm(vm, {{bob}}, 20);
        MinerState::test_helper_mock_mining_vm(vm, {{carol}}, 20);
        MinerState::test_helper_mock_mining_vm(vm, {{dave}}, 20);
        MinerState::test_helper_mock_mining_vm(vm, 0x3DC18D1CF61FAAC6AC70E3A63F062E4B, 20);

        let len = Vector::length<address>(&ValidatorUniverse::get_eligible_validators(vm));
        assert(len == 4 , 7357000180104);

        // Adding eve to validator universe - would be done by self
        ValidatorUniverse::test_helper_add_self_onboard(vm, 0x3DC18D1CF61FAAC6AC70E3A63F062E4B);

        let len = Vector::length<address>(&ValidatorUniverse::get_eligible_validators(vm));
        assert(len == 5 , 7357000180104);

        let i = 1;
        while (i < 16) {
            // Mock the validator doing work for 15 blocks, and stats being updated.
            Stats::process_set_votes(vm, &voters);
            i = i + 1;
        };

        Reconfigure::reconfigure(vm, 15); // reconfigure at height 15
    }
}
// check: EXECUTED

// Epoch 3 began
// The new node is in validatorUniverse and also in validator set

//! new-transaction
//! sender: diemroot
script {
    use 0x1::DiemSystem;
    use 0x1::ValidatorUniverse;
    use 0x1::Vector;
    fun main(vm: signer) {
        // Tests on initial size of validators
        assert(DiemSystem::validator_set_size() == 5, 7357000200101);
        assert(DiemSystem::is_validator({{alice}}) == true, 7357000200102);
        assert(DiemSystem::is_validator(0x3DC18D1CF61FAAC6AC70E3A63F062E4B), 7357000200103);
        let len = Vector::length<address>(&ValidatorUniverse::get_eligible_validators(vm));
        assert(DiemSystem::validator_set_size() == len, 7357000200104);
      }
}
// check: EXECUTED
