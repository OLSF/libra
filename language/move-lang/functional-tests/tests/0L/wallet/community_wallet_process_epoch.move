//! account: alice, 1000000, 0, validator
//! account: bob, 0, 0


//! new-transaction
//! sender: alice
script {
    use 0x1::Wallet;
    use 0x1::Vector;

    fun main(sender: &signer) {
      Wallet::set_comm(sender);
      let list = Wallet::get_comm_list();

      assert(Vector::length(&list) == 1, 7357001);
      assert(Wallet::is_comm({{alice}}), 7357002);

      let uid = Wallet::new_timed_transfer(sender, {{bob}}, 100, b"thanks bob");
      assert(Wallet::transfer_is_proposed(uid), 7357003);
    }
}

// check: EXECUTED


//////////////////////////////////////////////
//// Trigger reconfiguration at 61 seconds ///
//! block-prologue
//! proposer: alice
//! block-time: 61000000
//! round: 15

////// TEST RECONFIGURATION IS HAPPENING /////
// check: NewEpochEvent
//////////////////////////////////////////////

//////////////////////////////////////////////
//// Trigger reconfiguration again         ///
//! block-prologue
//! proposer: alice
//! block-time: 125000000
//! round: 20

////// TEST RECONFIGURATION IS HAPPENING /////
// check: NewEpochEvent
//////////////////////////////////////////////


//////////////////////////////////////////////
//// Trigger reconfiguration again         ///
//! block-prologue
//! proposer: alice
//! block-time: 190000000
//! round: 20

////// TEST RECONFIGURATION IS HAPPENING /////
// check: NewEpochEvent
//////////////////////////////////////////////


//! new-transaction
//! sender: libraroot
script {
    use 0x1::LibraAccount;
    use 0x1::GAS::GAS;
    fun main(_vm: &signer) {
      let bob_balance = LibraAccount::balance<GAS>({{bob}});
      assert(bob_balance == 100, 7357005);
    }
}

// check: EXECUTED