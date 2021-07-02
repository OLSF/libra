// Adding new validator epoch info
//! account: bob, 100000 ,0, validator


//! new-transaction
//! sender: bob
script{
use 0x1::ValidatorUniverse;

fun main(bob: signer) {
    ValidatorUniverse::remove_self(&bob);
}
}
// check: EXECUTED


//! new-transaction
//! sender: diemroot
script{
use 0x1::ValidatorUniverse;
use 0x1::Vector;

fun main(vm: signer) {
    let len = Vector::length<address>(
        &ValidatorUniverse::get_eligible_validators(&vm)
    );
    assert(len == 0, 73570);
}
}
// check: EXECUTED