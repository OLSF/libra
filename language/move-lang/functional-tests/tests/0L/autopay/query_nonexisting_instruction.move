//! account: shashank, 100
//! account: bob, 100

// We are trying to query a instruction which doesnot exist in the network

// Create instruction and retrieve it
//! new-transaction
//! sender: shashank
script {
  use 0x1::AutoPay2;
  use 0x1::Signer;
  fun main(sender: signer) {
    let sender = &sender;
    AutoPay2::enable_autopay(sender);
    assert(AutoPay2::is_enabled(Signer::address_of(sender)), 0);
    
    let (type, payee, end_epoch, percentage) = AutoPay2::query_instruction(
      Signer::address_of(sender), 1
    );
    // If autopay instruction doesn't exists, it returns (0x0, 0, 0)
    assert(type == 0, 1);
    assert(payee == {{0x0}}, 1);
    assert(end_epoch == 0, 1);
    assert(percentage == 0, 1);
  }
}
// check: EXECUTED
