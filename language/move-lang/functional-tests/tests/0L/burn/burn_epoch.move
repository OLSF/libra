//! account: alice, 10000000, 0, validator
//! account: bob, 0

// make Alice a case 1 validator, so that she is in the next validator set.

//! new-transaction
//! sender: alice
script {
    
    use 0x1::MinerState;

    fun main(sender: &signer) {
        // Alice is the only one that can update her mining stats. Hence this first transaction.

        MinerState::test_helper_mock_mining(sender, 5);
        // assert(MinerState::get_count_in_epoch({{alice}}) == 5, 7357300101011000);
    }
}
//check: EXECUTED


//! new-transaction
//! sender: libraroot
script {
    use 0x1::Stats;
    use 0x1::Vector;
    use 0x1::Cases;
    

    fun main(sender: &signer) {
        let voters = Vector::singleton<address>({{alice}});
        let i = 1;
        while (i < 16) {
            // Mock the validator doing work for 15 blocks, and stats being updated.
            Stats::process_set_votes(sender, &voters);
            i = i + 1;
        };

        assert(Cases::get_case(sender, {{alice}}, 0 , 15) == 1, 7357300103011000);
    }
}
//check: EXECUTED

//! new-transaction
//! sender: bob
script {
    use 0x1::Wallet;
    use 0x1::Vector;
    use 0x1::LibraAccount;

    fun main(sender: &signer) {
      Wallet::set_comm(sender);
      LibraAccount::init_cumulative_deposits(sender);
      let list = Wallet::get_comm_list();
      assert(Vector::length(&list) == 1, 7357001);
    }
}

// check: EXECUTED

//! new-transaction
//! sender: libraroot
script {
  use 0x1::LibraAccount;
  use 0x1::GAS::GAS;
  // use 0x1::Burn;
  // use 0x1::Vector;
  // use 0x1::FixedPoint32;

  fun main(vm: &signer) {
    // send to community wallet Bob
    LibraAccount::vm_make_payment_no_limit<GAS>({{alice}}, {{bob}}, 100, x"", x"", vm);

    let bal = LibraAccount::balance<GAS>({{bob}});
    assert(bal == 100, 7357001);
  }
}
// check: EXECUTED



//////////////////////////////////////////////
/// Trigger reconfiguration at 61 seconds ////
//! block-prologue
//! proposer: alice
//! block-time: 61000000
//! round: 15

////// TEST RECONFIGURATION IS HAPPENING /////
// check: NewEpochEvent
//////////////////////////////////////////////


//! new-transaction
//! sender: libraroot
script {
  use 0x1::LibraAccount;
  use 0x1::GAS::GAS;

  fun main(_vm: &signer) {
    let bal = LibraAccount::balance<GAS>({{bob}});
    // TODO: the burn equals 296/4 * scaling factor = 74000000
    // and then the 100 previously existed in the account.

    assert(bal == 1000100, 7357001);
  }
}

// check: EXECUTED