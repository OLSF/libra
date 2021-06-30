//// frank is a fullnode
//! account: frank, 100, 0

//! new-transaction
// Check if genesis subsidies have been distributed
//! sender: diemroot
script {
use 0x1::Subsidy;
use 0x1::DiemAccount;
use 0x1::GAS::GAS;
use 0x1::Debug::print;
fun main(vm: signer) {
    let old_account_bal = DiemAccount::balance<GAS>({{frank}});
    let value = Subsidy::distribute_fullnode_subsidy(vm, {{frank}}, 10);
    let new_account_bal = DiemAccount::balance<GAS>({{frank}});
    print(&value);
    assert(value == 24975360, 735701);
    assert(new_account_bal == 24975460, 735702);

    print(&new_account_bal);
    assert(new_account_bal>old_account_bal, 735703);
}
}
