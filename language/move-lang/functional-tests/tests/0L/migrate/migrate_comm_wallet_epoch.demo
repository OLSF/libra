//! account: alice, 1000000, 0, validator
//! account: bob, 1000000
//! account: carol, 1000000

// create autopay instructions to wallets which have not yet been marked as community wallets.

//! new-transaction
//! sender: alice
script {
  use 0x1::AutoPay2;
  use 0x1::Signer;
  use 0x1::Wallet;

  fun main(sender: &signer) {
    AutoPay2::enable_autopay(sender);
    assert(AutoPay2::is_enabled(Signer::address_of(sender)), 73570001);
    AutoPay2::create_instruction(sender, 1, 0, {{bob}}, 2, 5);
    AutoPay2::create_instruction(sender, 2, 0, {{carol}}, 2, 5);

    // is not a community wallet
    assert(!Wallet::is_comm({{bob}}), 7357006);
    assert(!Wallet::is_comm({{carol}}), 7357007);

  }
}
// check: EXECUTED

//////////////////////////////////////////////
///// Trigger reconfiguration at 61 seconds ////
//! block-prologue
//! proposer: alice
//! block-time: 61000000
//! round: 15

///// TEST RECONFIGURATION IS HAPPENING ////
// check: NewEpochEvent
//////////////////////////////////////////////

//! new-transaction
//! sender: libraroot
script {
    use 0x1::Wallet;
    fun main(_vm: &signer) {
      assert(Wallet::is_comm({{bob}}), 7357008);
      assert(Wallet::is_comm({{carol}}), 7357009);

    }
}
// check: EXECUTED
