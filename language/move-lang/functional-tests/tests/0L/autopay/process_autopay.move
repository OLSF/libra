//! account: alice, 1000000GAS
//! account: bob, 10000GAS

// We test processing of autopay at differnt epochs and balance transfers
// Finally, we also check the end_epoch functionality of autopay


//! new-transaction
//! sender: libraroot
script {
    use 0x1::AccountLimits;
    use 0x1::CoreAddresses;
    use 0x1::GAS::GAS;
    fun main(account: &signer) {
        AccountLimits::update_limits_definition<GAS>(account, CoreAddresses::LIBRA_ROOT_ADDRESS(), 0, 10, 0, 1);
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
    AccountLimits::update_limits_definition<GAS>(lr, {{alice}}, 0, 10, 0, 0);
    AccountLimits::publish_window<GAS>(lr, alice_account, {{alice}});
}
}
// check: "Keep(EXECUTED)"


// creating the instruction
//! new-transaction
//! sender: alice
script {
  use 0x1::AutoPay2;
  use 0x1::Signer;
  fun main(sender: &signer) {
    AutoPay2::enable_autopay(sender);
    assert(AutoPay2::is_enabled(Signer::address_of(sender)), 7357001);
    AutoPay2::create_instruction(sender, 1, 0, {{bob}}, 2, 500);
    let (type, payee, end_epoch, percentage) = AutoPay2::query_instruction(Signer::address_of(sender), 1);
    assert(type == 0u8, 7357002);
    assert(payee == {{bob}}, 7357003);
    assert(end_epoch == 2, 7357004);
    assert(percentage == 500, 7357005);
    }
}
// check: EXECUTED


// //! new-transaction
// //! sender: libraroot
// script {
//     use 0x1::Wallet;

//     fun main(vm: &signer) {
//       Wallet::init_comm_list(vm);
//     }
// }

// // check: EXECUTED

//! new-transaction
//! sender: bob
script {
    use 0x1::Wallet;
    use 0x1::Vector;

    fun main(sender: &signer) {
      Wallet::set_comm(sender);
      let list = Wallet::get_comm_list();
      assert(Vector::length(&list) == 1, 7357006);
    }
}

// check: EXECUTED

// Processing AutoPay2 to see if payments are done
//! new-transaction
//! sender: libraroot
script {
  use 0x1::AutoPay2;
  use 0x1::LibraAccount;
  use 0x1::GAS::GAS;
  fun main(sender: &signer) {
    let alice_balance = LibraAccount::balance<GAS>({{alice}});
    let bob_balance = LibraAccount::balance<GAS>({{bob}});
    assert(alice_balance==1000000, 7357007);
    AutoPay2::process_autopay(sender);
    
    let alice_balance_after = LibraAccount::balance<GAS>({{alice}});
    assert(alice_balance_after < alice_balance, 7357008);
    
    let transferred = alice_balance - alice_balance_after;
    let bob_received = LibraAccount::balance<GAS>({{bob}}) - bob_balance;
    assert(transferred==bob_received, 7357009)
  }
}
// check: EXECUTED