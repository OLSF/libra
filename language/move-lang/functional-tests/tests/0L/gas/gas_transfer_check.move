//! account: alice, 100, 0, validator
//! account: bob, 100, 0

//! new-transaction
// Transfers between accounts is disabled
//! sender: alice
//! gas-currency: GAS
script {
use 0x1::DiemAccount;
use 0x1::GAS::GAS;
use 0x1::Testnet;
fun main(account: signer) {
    //transfers are enabled in testnet, need to disable testnet to check that they are disabled otherwise
    Testnet::remove_testnet(account);
    let with_cap = DiemAccount::extract_withdraw_capability(account);
    DiemAccount::pay_from<GAS>(&with_cap, {{bob}}, 10, x"", x"");
    assert(DiemAccount::balance<GAS>({{alice}}) == 0, 0);
    assert(DiemAccount::balance<GAS>({{bob}}) == 10, 1);
    DiemAccount::restore_withdraw_capability(with_cap);
}
}
////////// Transfers should fail ////////
// check: VMExecutionFailure
/////////////////////////////////////////

// //! new-transaction
// // Transfers from diemroot to other accounts is enabled
// //! sender: diemroot
// //! gas-currency: GAS
// script {
// use 0x1::Diem;
// use 0x1::DiemAccount;
// use 0x1::GAS;
// ;
// fun main(account: signer) {
//     let coin = Diem::mint<GAS::T>(account, 10);
//     DiemAccount::deposit(account, {{bob}}, coin);
//     assert(DiemAccount::balance<GAS>({{bob}}) == 10, 4);
// }
// }
// // check: EXECUTED
