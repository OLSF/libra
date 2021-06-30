//! account: alice, 1000000GAS
//! account: bob, 10000GAS

// We test processing of autopay at differnt epochs and balance transfers
// Finally, we also check the end_epoch functionality of autopay


//! new-transaction
//! sender: diemroot
script {
    use 0x1::AccountLimits;
    use 0x1::CoreAddresses;
    use 0x1::GAS::GAS;
    fun main(account: signer) {
      AccountLimits::update_limits_definition<GAS>(
        &account, CoreAddresses::DIEM_ROOT_ADDRESS(), 0, 10, 0, 1
      );
    }
}
// check: "Keep(EXECUTED)"

//! new-transaction
//! sender: diemroot
//! execute-as: alice
script {
use 0x1::AccountLimits;
use 0x1::GAS::GAS;
fun main(lr: signer, alice_account: signer) {
    AccountLimits::publish_unrestricted_limits<GAS>(&alice_account);
    AccountLimits::update_limits_definition<GAS>(&lr, {{alice}}, 0, 10, 0, 0);
    AccountLimits::publish_window<GAS>(&lr, &alice_account, {{alice}});
}
}
// check: "Keep(EXECUTED)"


// creating the instruction
//! new-transaction
//! sender: alice
script {
  use 0x1::AutoPay2;
  use 0x1::Signer;
  fun main(sender: signer) {
    let sender = &sender;    
    AutoPay2::enable_autopay(sender);
    assert(AutoPay2::is_enabled(Signer::address_of(sender)), 0);
    AutoPay2::create_instruction(sender, 1, 0, {{bob}}, 2, 500);
    let (type, payee, end_epoch, percentage) = AutoPay2::query_instruction(
      Signer::address_of(sender), 1
    );
    assert(type == 0u8, 1);
    assert(payee == {{bob}}, 1);
    assert(end_epoch == 2, 1);
    assert(percentage == 500, 1);
  }
}
// check: EXECUTED

// Processing AutoPay2 to see if payments are done
//! new-transaction
//! sender: diemroot
script {
  use 0x1::AutoPay2;
  use 0x1::DiemAccount;
  use 0x1::GAS::GAS;
  use 0x1::Debug::print;
  fun main(sender: signer) {
    let alice_balance = DiemAccount::balance<GAS>({{alice}});
    let bob_balance = DiemAccount::balance<GAS>({{bob}});
    assert(alice_balance == 1000000, 1);
    AutoPay2::process_autopay(&sender);
    
    let alice_balance_after = DiemAccount::balance<GAS>({{alice}});
    assert(alice_balance_after < alice_balance, 2);
    
    let transferred = alice_balance - alice_balance_after;
    print(&transferred);
    let bob_received = DiemAccount::balance<GAS>({{bob}}) - bob_balance;
    print(&bob_received);
    //assert(bob_received == transferred, 2);
  }
}
// check: EXECUTED