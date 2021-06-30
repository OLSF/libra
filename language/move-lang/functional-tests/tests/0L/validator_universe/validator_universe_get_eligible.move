// Adding new validator epoch info
//! account: alice, 100000, 0, validator

//! new-transaction
//! sender: diemroot
script{
use 0x1::ValidatorUniverse;
use 0x1::Vector;


fun main(vm: signer) {
    // NOTE: in functional and e2e tests the genesis block includes 3 validators.
    // this is set here anguage/tools/vm-genesis/src/lib.rs
    // ValidatorUniverse::add_validator(0xDEADBEEF);
    // let validators_in_genesis = 4;
    let len = Vector::length<address>(&ValidatorUniverse::get_eligible_validators(vm));

    assert(len == 1, 100001);
}
}
// check: EXECUTED
