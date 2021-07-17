//! account: bob, 10000GAS, 0, validator
//! account: alice, 10000GAS, 0 
//! account: carol, 10000GAS, 0 

// test runs various autopay instruction types to ensure they are being executed as expected

//! new-transaction
//! sender: carol
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
//! sender: libraroot
script {
    use 0x1::AccountLimits;
    use 0x1::CoreAddresses;
    use 0x1::GAS::GAS;
    fun main(account: &signer) {
        AccountLimits::update_limits_definition<GAS>(account, CoreAddresses::LIBRA_ROOT_ADDRESS(), 0, 10000, 0, 1);
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
      AccountLimits::update_limits_definition<GAS>(lr, {{alice}}, 0, 10000, 0, 1);
      AccountLimits::publish_window<GAS>(lr, alice_account, {{alice}});
  }
}
// check: "Keep(EXECUTED)"

// alice commits to paying carol 500 GAS at the next tick
//! new-transaction
//! sender: alice
script {
  use 0x1::AutoPay2;
  use 0x1::Signer;
  fun main(sender: &signer) {
    AutoPay2::enable_autopay(sender);
    assert(AutoPay2::is_enabled(Signer::address_of(sender)), 0);
    
    // note: end epoch does not matter here as long as it is after the next epoch
    AutoPay2::create_instruction(sender, 1, 3, {{carol}}, 200, 500);

    let (type, payee, end_epoch, percentage) = AutoPay2::query_instruction(Signer::address_of(sender), 1);
    assert(type == 3, 1);
    assert(payee == {{carol}}, 1);
    assert(end_epoch == 200, 1);
    assert(percentage == 500, 1);
  }
}
// check: EXECUTED


///////////////////////////////////////////////////
///// Trigger Autopay Tick at 31 secs           ////
/// i.e. 1 second after 1/2 epoch  /////
//! block-prologue
//! proposer: bob
//! block-time: 31000000
//! round: 23
///////////////////////////////////////////////////


// Weird. This next block needs to be added here otherwise the prologue above does not run.
///////////////////////////////////////////////////
///// Trigger Autopay Tick at 31 secs           ////
/// i.e. 1 second after 1/2 epoch  /////
//! block-prologue
//! proposer: bob
//! block-time: 32000000
//! round: 24
///////////////////////////////////////////////////

//! new-transaction
//! sender: libraroot
script {
  use 0x1::LibraAccount;
  use 0x1::GAS::GAS;
  use 0x1::AutoPay2;
  fun main(_vm: &signer) {

    let ending_balance = LibraAccount::balance<GAS>({{alice}});
    assert(ending_balance == 9500, 7357006);
    
    //Confirm the one-shot instruction was deleted
    let (type, payee, end_epoch, percentage) = AutoPay2::query_instruction({{alice}}, 1);
    assert(type == 0, 1);
    assert(payee == 0x0, 1);
    assert(end_epoch == 0, 1);
    assert(percentage == 0, 1);
  }
}
// check: EXECUTED

///////////////////////////////////////////////////
///// Trigger Autopay Tick at 31 secs           ////
/// i.e. 1 second after 1/2 epoch  /////
//! block-prologue
//! proposer: bob
//! block-time: 61000000
//! round: 65
///////////////////////////////////////////////////

///////////////////////////////////////////////////
///// Trigger Autopay Tick at 31 secs           ////
/// i.e. 1 second after 1/2 epoch  /////
//! block-prologue
//! proposer: bob
//! block-time: 92000000
//! round: 66
///////////////////////////////////////////////////

///////////////////////////////////////////////////
///// Trigger Autopay Tick at 31 secs           ////
/// i.e. 1 second after 1/2 epoch  /////
//! block-prologue
//! proposer: bob
//! block-time: 93000000
//! round: 67
///////////////////////////////////////////////////

//! new-transaction
//! sender: libraroot
script {
  use 0x1::LibraAccount;
  use 0x1::GAS::GAS;
  fun main(_vm: &signer) {
    // no change, one-shot instruction is finished
    let ending_balance = LibraAccount::balance<GAS>({{alice}});
    assert(ending_balance == 9500, 7357006);

    // check balance of recipients
    let ending_balance = LibraAccount::balance<GAS>({{carol}});
    assert(ending_balance == 10500, 7357006);
  }
}
// check: EXECUTED
