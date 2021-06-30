//! account: alice, 100
//! account: bob, 100

// We test creation of autopay, retiriving it using same and different accounts
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
    assert(AutoPay2::is_enabled(Signer::address_of(sender)), 0);
    AutoPay2::create_instruction(sender, 1, 0, {{bob}}, 2, 5);
    let (type, payee, end_epoch, percentage) = AutoPay2::query_instruction(
      Signer::address_of(sender), 1
    );
    assert(type == 0, 1);
    assert(payee == {{bob}}, 1);
    assert(end_epoch == 2, 1);
    assert(percentage == 5, 1);
  }
}
// check: EXECUTED

// Query using different account
//! new-transaction
//! sender: bob
script {
  use 0x1::AutoPay2;
  fun main() {
    let (type, payee, end_epoch, percentage) = AutoPay2::query_instruction({{alice}}, 1);
    assert(type == 0, 1);
    assert(payee == {{bob}}, 1);
    assert(end_epoch == 2, 1);
    assert(percentage == 5, 1);
  }
}
// check: EXECUTED


// Test to create instruction and retrieve it
//! new-transaction
//! sender: alice
script {
  use 0x1::AutoPay2;
  use 0x1::Signer;
  fun main(sender: signer) {
    let sender = &sender;
    AutoPay2::delete_instruction(sender, 1);
    let (type, payee, end_epoch, percentage) = AutoPay2::query_instruction(
      Signer::address_of(sender), 1
    );
    // If autopay instruction doesn't exists, it returns (0x0, 0, 0)
    assert(type == 0u8, 1);
    assert(payee == {{0x0}}, 1);
    assert(end_epoch == 0, 1);
    assert(percentage == 0, 1);
  }
}
// check: EXECUTED
