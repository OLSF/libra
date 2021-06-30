//! account: alice, 1000000
//! account: bob, 1000000
//! account: carol, 1000000

// We test creation of autopay, retrieving it using same and different accounts
// Finally, we also test deleting of autopay

// Test to create instruction and retrieve it
//! new-transaction
//! sender: alice
script {
  use 0x1::AutoPay2;
  use 0x1::Signer;
  fun main(sender: signer) {
    let sender = &sender;
    AutoPay2::enable_autopay(sender);
    assert(AutoPay2::is_enabled(Signer::address_of(sender)), 73570001);
    AutoPay2::create_instruction(sender, 1, 0, {{bob}}, 2, 5);
    let (type, payee, end_epoch, percentage) = AutoPay2::query_instruction(
      Signer::address_of(sender), 1
    );
    assert(type == 0, 7357005);
    assert(payee == {{bob}}, 73570002);
    assert(end_epoch == 2, 73570003);
    assert(percentage == 5, 73570004);
  }
}
// check: EXECUTED

// Test to create another instruction also by alice. The account already has autopay enabled.
//! new-transaction
//! sender: alice
script {
  use 0x1::AutoPay2;
  use 0x1::Signer;
  fun main(sender: signer) {
    let sender = &sender;
    assert(AutoPay2::is_enabled(Signer::address_of(sender)), 73570005);    
    AutoPay2::create_instruction(sender, 2, 0, {{alice}}, 4, 5);
  }
}
// check: EXECUTED