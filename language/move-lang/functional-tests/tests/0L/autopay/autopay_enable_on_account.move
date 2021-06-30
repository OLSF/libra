//! account: alice, 1000000, 0, validator

// This test is to test enable and disable functionalites of autopay
//! new-transaction
//! sender: alice
script {
  use 0x1::AutoPay2;
  use 0x1::Signer;
  fun main(sender: signer) {
    AutoPay2::enable_autopay(&sender);
    assert(AutoPay2::is_enabled(Signer::address_of(&sender)), 0);
    AutoPay2::disable_autopay(&sender);
    assert(!AutoPay2::is_enabled(Signer::address_of(&sender)), 1);
  }
}
// check: EXECUTED