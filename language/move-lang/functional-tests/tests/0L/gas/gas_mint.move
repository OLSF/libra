//! account: alice, 1000000, 0, validator

//! new-transaction
//! sender: diemroot
script {
use 0x1::Diem;
use 0x1::GAS::GAS;
use 0x1::DiemAccount;

fun main(vm: signer) {
    let old_market_cap = Diem::market_cap<GAS>();
    let coin = Diem::mint<GAS>(vm, 1000);
    assert(Diem::value<GAS>(&coin) == 1000, 1);
    assert(Diem::market_cap<GAS>() == old_market_cap + 1000, 2);
    DiemAccount::vm_deposit_with_metadata<GAS>(
        vm,
        {{alice}},
        coin,
        x"", x""
    );
}
}
