//! account: alice, 4000000, 0, validator
//! account: bob, 1000000, 0, validator
//! account: carol, 1000000, 0, validator
//! account: dave, 1000000, 0, validator

//! new-transaction
//! sender: alice
script {
  use 0x1::LibraAccount;
  use 0x1::ValidatorConfig;
  use 0x1::TestFixtures;
  use 0x1::VDF;
  use 0x1::Signer;
  use 0x1::MinerState;

  fun main(sender: &signer) {
    // Scenario: Alice, an existing validator, is sending a transaction for Eve, with a challenge and proof from eve's block_0
    let challenge = TestFixtures::eve_0_easy_chal();
    let solution = TestFixtures::eve_0_easy_sol();
    // // Parse key and check
    let (eve_addr, _auth_key) = VDF::extract_address_from_challenge(&challenge);
    assert(eve_addr == 0x3DC18D1CF61FAAC6AC70E3A63F062E4B, 401);

    let sender_addr = Signer::address_of(sender);
    let epochs_since_creation = 10;
    MinerState::test_helper_set_rate_limit(sender_addr, epochs_since_creation);

    LibraAccount::create_validator_account_with_proof(
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
//! sender: alice
script {
    use 0x1::AutoPay2;
    fun main(sender: &signer) {
        AutoPay2::enable_autopay(sender);
    }
}

//! new-transaction
//! sender: bob
script {
    use 0x1::AutoPay2;
    fun main(sender: &signer) {
        AutoPay2::enable_autopay(sender);
    }
}

//! new-transaction
//! sender: carol
script {
    use 0x1::AutoPay2;
    fun main(sender: &signer) {
        AutoPay2::enable_autopay(sender);
    }
}

//! new-transaction
//! sender: dave
script {
    use 0x1::AutoPay2;
    fun main(sender: &signer) {
        AutoPay2::enable_autopay(sender);
    }
}

//! new-transaction
//! sender: libraroot
script {
    use 0x1::LibraSystem;
    use 0x1::Reconfigure;
    use 0x1::Vector;
    use 0x1::MinerState;
    use 0x1::Stats;
    use 0x1::LibraAccount;
    use 0x1::GAS::GAS;
    use 0x1::ValidatorConfig;

    fun main(vm: &signer) {
        // Tests on initial size of validators
        assert(LibraSystem::validator_set_size() == 4, 7357000180101);
        assert(LibraSystem::is_validator({{alice}}) == true, 7357000180102);
        assert(LibraSystem::is_validator({{bob}}) == true, 7357000180103);
        assert(LibraSystem::is_validator(0x3DC18D1CF61FAAC6AC70E3A63F062E4B) == false, 7357000180104);
        assert(MinerState::is_init(0x3DC18D1CF61FAAC6AC70E3A63F062E4B), 7357000180105);

        MinerState::test_helper_mock_mining_vm(vm, {{alice}}, 20);
        MinerState::test_helper_mock_mining_vm(vm, {{bob}}, 20);
        MinerState::test_helper_mock_mining_vm(vm, {{carol}}, 20);
        MinerState::test_helper_mock_mining_vm(vm, {{dave}}, 20);

        // Transfer coins to operators
        let oper_alice = ValidatorConfig::get_operator({{alice}});
        let oper_bob = ValidatorConfig::get_operator({{bob}});
        let oper_carol = ValidatorConfig::get_operator({{carol}});
        let oper_dave = ValidatorConfig::get_operator({{dave}});
        LibraAccount::vm_make_payment_no_limit<GAS>( {{alice}}, oper_alice, 60009, x"", x"", vm);  
        LibraAccount::vm_make_payment_no_limit<GAS>( {{bob}}, oper_bob, 60009, x"", x"", vm);  
        LibraAccount::vm_make_payment_no_limit<GAS>( {{carol}}, oper_carol, 60009, x"", x"", vm);  
        LibraAccount::vm_make_payment_no_limit<GAS>( {{dave}}, oper_dave, 60009, x"", x"", vm);

        // Mock everyone being a CASE 1
        let voters = Vector::empty<address>();
        Vector::push_back<address>(&mut voters, {{alice}});
        Vector::push_back<address>(&mut voters, {{bob}});
        Vector::push_back<address>(&mut voters, {{carol}});
        Vector::push_back<address>(&mut voters, {{dave}});
        let i = 1;
        while (i < 16) {
            // Mock the validator doing work for 15 blocks, and stats being updated.
            Stats::process_set_votes(vm, &voters);
            i = i + 1;
        };

        Reconfigure::reconfigure(vm, 15); // reconfigure at height 15
        assert(LibraSystem::validator_set_size() == 4, 7357000180106);
    }
}
// check: EXECUTED

// Epoch 2 began
// The new node is in validatorUniverse but not in validator set

//! new-transaction
//! sender: libraroot
script {
    use 0x1::LibraSystem;
    use 0x1::ValidatorUniverse;
    use 0x1::Vector;
    fun main(vm: &signer) {
        // Tests on initial size of validators
        // New validator is not in this set.
        assert(LibraSystem::validator_set_size() == 4, 7357000180101);
        assert(LibraSystem::is_validator({{alice}}) == true, 7357000180102);
        assert(!LibraSystem::is_validator(0x3DC18D1CF61FAAC6AC70E3A63F062E4B), 7357000180103);
        let len = Vector::length<address>(&ValidatorUniverse::get_eligible_validators(vm));
        // Is in validator universe
        assert(len == 5, 7357000180104);
      }
}
// check: EXECUTED

// The new node starts mining and submiting proofs in the epoch 2
//
//! new-transaction
//! sender: libraroot
script {
    use 0x1::LibraSystem;
    use 0x1::Reconfigure;
    use 0x1::Vector;
    use 0x1::MinerState;
    use 0x1::Stats;
    use 0x1::ValidatorUniverse;
    use 0x1::AutoPay2;
    use 0x1::LibraAccount;
    use 0x1::GAS::GAS;
    use 0x1::ValidatorConfig;

    fun main(vm: &signer) {
        // Tests on initial size of validators
        assert(LibraSystem::validator_set_size() == 4, 7357000180201);
        assert(LibraSystem::is_validator({{alice}}) == true, 7357000180202);
        assert(LibraSystem::is_validator({{bob}}) == true, 7357000180203);
        assert(LibraSystem::is_validator(0x3DC18D1CF61FAAC6AC70E3A63F062E4B) == false, 7357000180204);

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

        // enable autopay and transfer coins to the new operator
        let new_val = LibraAccount::test_helper_create_signer(vm, 0x3DC18D1CF61FAAC6AC70E3A63F062E4B);
        AutoPay2::enable_autopay(&new_val);
        let new_oper = ValidatorConfig::get_operator(0x3DC18D1CF61FAAC6AC70E3A63F062E4B);
        LibraAccount::vm_make_payment_no_limit<GAS>(0x3DC18D1CF61FAAC6AC70E3A63F062E4B, new_oper, 60009, x"", x"", vm);

        // check the new account is in the list of eligible
        let len = Vector::length<address>(&ValidatorUniverse::get_eligible_validators(vm));
        assert(len == 5 , 7357000180205);

        let len = Vector::length<address>(&ValidatorUniverse::get_eligible_validators(vm));
        assert(len == 5 , 7357000180206);

        let i = 1;
        while (i < 16) {
            // Mock the validator doing work for 15 blocks, and stats being updated.
            Stats::process_set_votes(vm, &voters);
            i = i + 1;
        };

        Reconfigure::reconfigure(vm, 15); // reconfigure at height 15
        LibraAccount::test_helper_destroy_signer(vm, new_val);
    }
}
// check: EXECUTED

// Epoch 3 began
// The new node is in validatorUniverse and also in validator set

//! new-transaction
//! sender: libraroot
script {
    use 0x1::LibraSystem;
    use 0x1::ValidatorUniverse;
    use 0x1::Vector;
    fun main(vm: &signer) {
        // Tests on initial size of validators
        assert(LibraSystem::validator_set_size() == 5, 7357000200301);
        assert(LibraSystem::is_validator({{alice}}) == true, 7357000200302);
        assert(LibraSystem::is_validator(0x3DC18D1CF61FAAC6AC70E3A63F062E4B), 7357000200303);
        let len = Vector::length<address>(&ValidatorUniverse::get_eligible_validators(vm));
        assert(len == 5, 7357000200304);
      }
}
// check: EXECUTED
