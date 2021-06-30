//! account: alice, 300GAS
//! account: bob, 100GAS
//! account: carol, 10000GAS, 0, validator

// Ensure that changing the account limit percentage given to autopay works. 

//! new-transaction
//! sender: diemroot
script {
    use 0x1::AccountLimits;
    use 0x1::CoreAddresses;
    use 0x1::GAS::GAS;
    use 0x1::AutoPay2;
    fun main(account: signer) {
        AccountLimits::update_limits_definition<GAS>(
            &account, CoreAddresses::DIEM_ROOT_ADDRESS(), 0, 30, 0, 1
        );
        AutoPay2::enable_account_limits(&account);
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
    AccountLimits::update_limits_definition<GAS>(&lr, {{alice}}, 0, 30, 0, 1);
    AccountLimits::publish_window<GAS>(&lr, &alice_account, {{alice}});
}
}
// check: "Keep(EXECUTED)"

//! new-transaction
//! sender: diemroot
//! execute-as: bob
script {
use 0x1::AccountLimits;
use 0x1::GAS::GAS;
fun main(lr: signer, bob_account: signer) {
    AccountLimits::publish_unrestricted_limits<GAS>(&bob_account);
    AccountLimits::update_limits_definition<GAS>(&lr, {{bob}}, 0, 30, 0, 1);
    AccountLimits::publish_window<GAS>(&lr, &bob_account, {{bob}});
}
}
// check: "Keep(EXECUTED)"




// creating the payment
//! new-transaction
//! sender: alice
script {
  use 0x1::AutoPay2;
  use 0x1::Signer;
  use 0x1::GAS::GAS;
  use 0x1::DiemAccount;
  fun main(sender: signer) {
    let sender = &sender;
    AutoPay2::enable_autopay(sender);
    assert(AutoPay2::is_enabled(Signer::address_of(sender)), 0);
    
    //one shot payment to bob
    AutoPay2::create_instruction(sender, 1, 3, {{bob}}, 1000, 100);

    //update account limit dedicated to paying escrow to 50%
    DiemAccount::update_escrow_percentage<GAS>(sender, 50);

    let (type, payee, end_epoch, amt) = 
      AutoPay2::query_instruction(Signer::address_of(sender), 1);
    assert(type == 3, 1);
    assert(payee == {{bob}}, 1);
    assert(end_epoch == 1000, 1);
    assert(amt == 100, 1);
  }
}
// check: EXECUTED

// Checking balance before autopay module
//! new-transaction
//! sender: diemroot
script {
  use 0x1::DiemAccount;
  use 0x1::GAS::GAS;
  fun main() {
    let alice_balance = DiemAccount::balance<GAS>({{alice}});
    let bob_balance = DiemAccount::balance<GAS>({{bob}});
    assert(alice_balance == 300, 1);
    assert(bob_balance == 100, 2);
    }
}
// check: EXECUTED

///////////////////////////////////////////////////
///// Trigger Autopay Tick at 31 secs           ////
/// i.e. 1 second after 1/2 epoch  /////
//! block-prologue
//! proposer: carol
//! block-time: 31000000
//! round: 23
///////////////////////////////////////////////////


// Weird. This next block needs to be added here otherwise the prologue above does not run.
///////////////////////////////////////////////////
///// Trigger Autopay Tick at 31 secs           ////
/// i.e. 1 second after 1/2 epoch  /////
//! block-prologue
//! proposer: carol
//! block-time: 32000000
//! round: 24
///////////////////////////////////////////////////

//! new-transaction
//! sender: diemroot
script {
  use 0x1::DiemAccount;
  use 0x1::GAS::GAS;
  fun main(_vm: signer) {
    let alice_balance = DiemAccount::balance<GAS>({{alice}});
    let bob_balance = DiemAccount::balance<GAS>({{bob}});
    assert(alice_balance==200, 1);
    assert(bob_balance == 130, 2);
  }
}
// check: EXECUTED

///////////////////////////////////////////////////
///// Trigger Autopay Tick at 31 secs           ////
/// i.e. 1 second after 1/2 epoch  /////
//! block-prologue
//! proposer: carol
//! block-time: 61000000
//! round: 65
///////////////////////////////////////////////////

///////////////////////////////////////////////////
///// Trigger Autopay Tick at 31 secs           ////
/// i.e. 1 second after 1/2 epoch  /////
//! block-prologue
//! proposer: carol
//! block-time: 62000000
//! round: 66
///////////////////////////////////////////////////


//! new-transaction
//! sender: diemroot
script {
  use 0x1::DiemAccount;
  use 0x1::GAS::GAS;
  fun main(_vm: signer) {
    let alice_balance = DiemAccount::balance<GAS>({{alice}});
    let bob_balance = DiemAccount::balance<GAS>({{bob}});
    assert(alice_balance == 200, 1);
    assert(bob_balance == 145, 2);
  }
}
// check: EXECUTED
