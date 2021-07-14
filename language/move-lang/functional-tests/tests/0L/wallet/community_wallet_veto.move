///// Setting up the test fixtures for the transactions below. The tags below create validators alice and bob, giving them 1000000 GAS coins.

// alice is a community wallet
// bob is a recipient of the community wallet
// carol and dave are validators, that vote to reject the transaction

//! account: alice, 1000000, 0
//! account: bob, 1000000, 0
//! account: carol, 1000000, 0, validator
//! account: dave, 1000000, 0, validator

// Set voting power of the validtors

//! new-transaction
//! sender: alice
script {
    use 0x1::MinerState;
    fun main(_sender: &signer) {
      MinerState::test_helper_set_epochs_mining({{carol}}, 50);
      MinerState::test_helper_set_epochs_mining({{dave}}, 50);
    }
}
// check: EXECUTED


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

//! new-transaction
//! sender: carol
script {
    use 0x1::Wallet;
    
    fun main(sender: &signer) {
      let uid = 1;
      Wallet::veto(sender, uid);
      assert(Wallet::transfer_is_proposed(uid), 7357004);
      assert(!Wallet::transfer_is_rejected(uid), 7357005);
    }
}

// check: EXECUTED

//! new-transaction
//! sender: dave
script {
    use 0x1::Wallet;
    use 0x1::Debug::print;

    fun main(sender: &signer) {
      let uid = 1;
      print(&0x1);
      Wallet::veto(sender, uid);
      print(&0x2);
      assert(!Wallet::transfer_is_proposed(uid), 7357006);
      print(&0x2);
      assert(Wallet::transfer_is_rejected(uid), 7357007);
    }
}

// check: EXECUTED