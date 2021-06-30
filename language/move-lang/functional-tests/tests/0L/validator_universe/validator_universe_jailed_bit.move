// Adding new validator epoch info
//! account: alice, 100000, 0, validator
//! account: eve, 100000

//! new-transaction
//! sender: alice
script{
use 0x1::ValidatorUniverse;
use 0x1::Signer;
fun main(eve_sig: signer) {
    // Test from genesis if not jailed and in universe
    let addr = Signer::address_of(eve_sig);
    assert(!ValidatorUniverse::is_jailed(addr), 73570001);
    assert(ValidatorUniverse::is_in_universe(addr), 73570002);
}
}
// check: EXECUTED


//! new-transaction
//! sender: diemroot
script{
use 0x1::ValidatorUniverse;
// use 0x1::Signer;
fun main(vm: signer) {
    // Test from genesis if not jailed and in universe
    ValidatorUniverse::jail(vm, {{alice}});
    assert(ValidatorUniverse::is_jailed({{alice}}), 73570001);
    assert(ValidatorUniverse::is_in_universe({{alice}}), 73570002);
}
}
// check: EXECUTED