//! account: alice, 300GAS
//! account: bob, 100GAS
//! account: greg, 100GAS
//! account: carol, 10000GAS, 0, validator

// Check autopay is triggered in block prologue correctly i.e., middle of epoch boundary

//! new-transaction
//! sender: bob
script {
    use 0x1::Wallet;
    use 0x1::Vector;

    fun main(sender: &signer) {
      Wallet::set_comm(sender);
      let list = Wallet::get_comm_list();
      assert(Vector::length(&list) == 1, 7357001);
    }
}

// check: EXECUTED


//! new-transaction
//! sender: greg
script {
    use 0x1::Wallet;
    use 0x1::Vector;

    fun main(sender: &signer) {
      Wallet::set_comm(sender);
      let list = Wallet::get_comm_list();
      assert(Vector::length(&list) == 2, 7357001);
    }
}

// check: EXECUTED


//! new-transaction
//! sender: libraroot
script {
    use 0x1::AccountLimits;
    use 0x1::CoreAddresses;
    use 0x1::GAS::GAS;
    use 0x1::AutoPay2;
    fun main(account: &signer) {
        AccountLimits::update_limits_definition<GAS>(account, CoreAddresses::LIBRA_ROOT_ADDRESS(), 0, 30, 0, 1);
        AutoPay2::enable_account_limits(account);
    }
}
// check: "Keep(EXECUTED)"

//! new-transaction
//! sender: libraroot
//! execute-as: alice
script {
use 0x1::AccountLimits;
use 0x1::GAS::GAS;
fun main(lr: &signer, alice_account: &signer) {
    AccountLimits::publish_unrestricted_limits<GAS>(alice_account);
    AccountLimits::update_limits_definition<GAS>(lr, {{alice}}, 0, 30, 0, 1);
    AccountLimits::publish_window<GAS>(lr, alice_account, {{alice}});
}
}
// check: "Keep(EXECUTED)"

//! new-transaction
//! sender: libraroot
//! execute-as: bob
script {
use 0x1::AccountLimits;
use 0x1::GAS::GAS;
fun main(lr: &signer, bob_account: &signer) {
    AccountLimits::publish_unrestricted_limits<GAS>(bob_account);
    AccountLimits::update_limits_definition<GAS>(lr, {{bob}}, 0, 30, 0, 1);
    AccountLimits::publish_window<GAS>(lr, bob_account, {{bob}});
}
}
// check: "Keep(EXECUTED)"

//! new-transaction
//! sender: libraroot
//! execute-as: greg
script {
use 0x1::AccountLimits;
use 0x1::GAS::GAS;
fun main(lr: &signer, greg_account: &signer) {
    AccountLimits::publish_unrestricted_limits<GAS>(greg_account);
    AccountLimits::update_limits_definition<GAS>(lr, {{greg}}, 0, 30, 0, 1);
    AccountLimits::publish_window<GAS>(lr, greg_account, {{greg}});
}
}
// check: "Keep(EXECUTED)"


// creating the payment
//! new-transaction
//! sender: alice
script {
  use 0x1::AutoPay2;
  use 0x1::Signer;
  fun main(sender: &signer) {
    AutoPay2::enable_autopay(sender);
    assert(AutoPay2::is_enabled(Signer::address_of(sender)), 0);
    
    AutoPay2::create_instruction(sender, 1, 2, {{bob}}, 2, 50);
    AutoPay2::create_instruction(sender, 2, 2, {{greg}}, 2, 50);

    let (type, payee, end_epoch, amt) = AutoPay2::query_instruction(Signer::address_of(sender), 1);
    assert(type == 2, 1);
    assert(payee == {{bob}}, 1);
    assert(end_epoch == 2, 1);
    assert(amt == 50, 1);

    let (type, payee, end_epoch, amt) = AutoPay2::query_instruction(Signer::address_of(sender), 2);
    assert(type == 2, 1);
    assert(payee == {{greg}}, 1);
    assert(end_epoch == 2, 1);
    assert(amt == 50, 1);
  }
}
// check: EXECUTED

// Checking balance before autopay module
//! new-transaction
//! sender: libraroot
script {
  use 0x1::LibraAccount;
  use 0x1::GAS::GAS;
  fun main() {
    let alice_balance = LibraAccount::balance<GAS>({{alice}});
    let bob_balance = LibraAccount::balance<GAS>({{bob}});
    let greg_balance = LibraAccount::balance<GAS>({{greg}});
    assert(alice_balance==300, 1);
    assert(bob_balance == 100, 2);
    assert(greg_balance == 100, 2);
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
//! sender: libraroot
script {
  use 0x1::LibraAccount;
  use 0x1::GAS::GAS;
  fun main(_vm: &signer) {
    let alice_balance = LibraAccount::balance<GAS>({{alice}});
    let bob_balance = LibraAccount::balance<GAS>({{bob}});
    let greg_balance = LibraAccount::balance<GAS>({{greg}});
    assert(alice_balance==200, 1);
    assert(bob_balance == 130, 2);
    assert(greg_balance == 100, 2);
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
//! sender: libraroot
script {
  use 0x1::LibraAccount;
  use 0x1::GAS::GAS;
  fun main(_vm: &signer) {
    let alice_balance = LibraAccount::balance<GAS>({{alice}});
    let bob_balance = LibraAccount::balance<GAS>({{bob}});
    let greg_balance = LibraAccount::balance<GAS>({{greg}});
    assert(alice_balance==100, 1);
    assert(bob_balance == 150, 2);
    assert(greg_balance == 110, 2);
  }
}
// check: EXECUTED

///////////////////////////////////////////////////
///// Trigger Autopay Tick at 31 secs           ////
/// i.e. 1 second after 1/2 epoch  /////
//! block-prologue
//! proposer: carol
//! block-time: 122000000
//! round: 67
///////////////////////////////////////////////////

///////////////////////////////////////////////////
///// Trigger Autopay Tick at 31 secs           ////
/// i.e. 1 second after 1/2 epoch  /////
//! block-prologue
//! proposer: carol
//! block-time: 123000000
//! round: 68
///////////////////////////////////////////////////


//! new-transaction
//! sender: libraroot
script {
  use 0x1::LibraAccount;
  use 0x1::GAS::GAS;
  fun main(_vm: &signer) {
    let alice_balance = LibraAccount::balance<GAS>({{alice}});
    let bob_balance = LibraAccount::balance<GAS>({{bob}});
    let greg_balance = LibraAccount::balance<GAS>({{greg}});
    assert(alice_balance==100, 1);
    assert(bob_balance == 150, 2);
    assert(greg_balance == 140, 2);
  }
}
// check: EXECUTED

///////////////////////////////////////////////////
///// Trigger Autopay Tick at 31 secs           ////
/// i.e. 1 second after 1/2 epoch  /////
//! block-prologue
//! proposer: carol
//! block-time: 183000000
//! round: 69
///////////////////////////////////////////////////

///////////////////////////////////////////////////
///// Trigger Autopay Tick at 31 secs           ////
/// i.e. 1 second after 1/2 epoch  /////
//! block-prologue
//! proposer: carol
//! block-time: 184000000
//! round: 70
///////////////////////////////////////////////////


//! new-transaction
//! sender: libraroot
script {
  use 0x1::LibraAccount;
  use 0x1::GAS::GAS;
  fun main(_vm: &signer) {
    let alice_balance = LibraAccount::balance<GAS>({{alice}});
    let bob_balance = LibraAccount::balance<GAS>({{bob}});
    let greg_balance = LibraAccount::balance<GAS>({{greg}});
    assert(alice_balance==100, 1);
    assert(bob_balance == 170, 2);
    assert(greg_balance == 150, 2);
  }
}
// check: EXECUTED

///////////////////////////////////////////////////
///// Trigger Autopay Tick at 31 secs           ////
/// i.e. 1 second after 1/2 epoch  /////
//! block-prologue
//! proposer: carol
//! block-time: 244000000
//! round: 71
///////////////////////////////////////////////////

///////////////////////////////////////////////////
///// Trigger Autopay Tick at 31 secs           ////
/// i.e. 1 second after 1/2 epoch  /////
//! block-prologue
//! proposer: carol
//! block-time: 245000000
//! round: 72
///////////////////////////////////////////////////


//! new-transaction
//! sender: libraroot
script {
  use 0x1::LibraAccount;
  use 0x1::GAS::GAS;
  fun main(_vm: &signer) {
    let alice_balance = LibraAccount::balance<GAS>({{alice}});
    let bob_balance = LibraAccount::balance<GAS>({{bob}});
    let greg_balance = LibraAccount::balance<GAS>({{greg}});
    assert(alice_balance==100, 1);
    assert(bob_balance == 200, 2);
    assert(greg_balance == 150, 2);
  }
}
// check: EXECUTED

///////////////////////////////////////////////////
///// Trigger Autopay Tick at 31 secs           ////
/// i.e. 1 second after 1/2 epoch  /////
//! block-prologue
//! proposer: carol
//! block-time: 305000000
//! round: 73
///////////////////////////////////////////////////

///////////////////////////////////////////////////
///// Trigger Autopay Tick at 31 secs           ////
/// i.e. 1 second after 1/2 epoch  /////
//! block-prologue
//! proposer: carol
//! block-time: 306000000
//! round: 74
///////////////////////////////////////////////////


//! new-transaction
//! sender: libraroot
script {
  use 0x1::LibraAccount;
  use 0x1::GAS::GAS;
  fun main(_vm: &signer) {
    let alice_balance = LibraAccount::balance<GAS>({{alice}});
    let bob_balance = LibraAccount::balance<GAS>({{bob}});
    let greg_balance = LibraAccount::balance<GAS>({{greg}});
    assert(alice_balance==100, 1);
    assert(bob_balance == 200, 2);
    assert(greg_balance == 180, 2);
  }
}
// check: EXECUTED


///////////////////////////////////////////////////
///// Trigger Autopay Tick at 31 secs           ////
/// i.e. 1 second after 1/2 epoch  /////
//! block-prologue
//! proposer: carol
//! block-time: 366000000
//! round: 75
///////////////////////////////////////////////////

///////////////////////////////////////////////////
///// Trigger Autopay Tick at 31 secs           ////
/// i.e. 1 second after 1/2 epoch  /////
//! block-prologue
//! proposer: carol
//! block-time: 367000000
//! round: 76
///////////////////////////////////////////////////


//! new-transaction
//! sender: libraroot
script {
  use 0x1::LibraAccount;
  use 0x1::GAS::GAS;
  fun main(_vm: &signer) {
    let alice_balance = LibraAccount::balance<GAS>({{alice}});
    let bob_balance = LibraAccount::balance<GAS>({{bob}});
    let greg_balance = LibraAccount::balance<GAS>({{greg}});
    assert(alice_balance==100, 1);
    assert(bob_balance == 200, 2);
    assert(greg_balance == 200, 2);
  }
}
// check: EXECUTED


///////////////////////////////////////////////////
///// Trigger Autopay Tick at 31 secs           ////
/// i.e. 1 second after 1/2 epoch  /////
//! block-prologue
//! proposer: carol
//! block-time: 427000000
//! round: 77
///////////////////////////////////////////////////

///////////////////////////////////////////////////
///// Trigger Autopay Tick at 31 secs           ////
/// i.e. 1 second after 1/2 epoch  /////
//! block-prologue
//! proposer: carol
//! block-time: 428000000
//! round: 78
///////////////////////////////////////////////////


//! new-transaction
//! sender: libraroot
script {
  use 0x1::LibraAccount;
  use 0x1::GAS::GAS;
  fun main(_vm: &signer) {
    let alice_balance = LibraAccount::balance<GAS>({{alice}});
    let bob_balance = LibraAccount::balance<GAS>({{bob}});
    let greg_balance = LibraAccount::balance<GAS>({{greg}});
    assert(alice_balance==100, 1);
    assert(bob_balance == 200, 2);
    assert(greg_balance == 200, 2);
  }
}
// check: EXECUTED