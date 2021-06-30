//! account: carol, 1000000
//! account: bob, 1000000
//! account: alice, 1000000

// We test creation of autopay, retiriving it using same and different accounts
// Finally, we also test deleting of autopay

// Test to create instruction and retrieve it
//! new-transaction
//! sender: carol
script {
  use 0x1::AutoPay2;
  use 0x1::Signer;
  fun main(sender: signer) {
    let sender = &sender;
    AutoPay2::enable_autopay(sender);
    assert(AutoPay2::is_enabled(Signer::address_of(sender)), 0);
    AutoPay2::create_instruction(sender, 1, 0, {{bob}}, 2, 5);
    let (type, payee, end_epoch, percentage) = AutoPay2::query_instruction(
      Signer::address_of(sender), 1
    );
    assert(type == 0u8, 1);
    assert(payee == {{bob}}, 1);
    assert(end_epoch == 2, 1);
    assert(percentage == 5, 1);
  }
}
// check: EXECUTED

// Test to create another instruction
//! new-transaction
//! sender: carol
script {
  use 0x1::AutoPay2;
  use 0x1::Signer;
  fun main(sender: signer) {    
    assert(AutoPay2::is_enabled(Signer::address_of(&sender)), 0);
    AutoPay2::create_instruction(&sender, 2, 0, {{alice}}, 4, 5);
  }
}
// check: EXECUTED

// // // Test to create instruction with wrong UUID
// //! new-transaction
// //! sender: carol
// script {
//   use 0x0::AutoPay2;
//   use 0x0::Transaction;
//   use 0x0::Signer;
//   fun main(sender: signer) {
//     AutoPay2::enable_autopay();
//     Transaction::assert(AutoPay2::is_enabled(Signer::address_of(sender)), 0);
//     AutoPay2::create_instruction(2, {{bob}}, 5, 5);
//     }
// }
// // check: EXECUTED
