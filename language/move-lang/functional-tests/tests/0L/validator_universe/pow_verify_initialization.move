// This test is to check if Proof of weight is intialized properly

//! account: alice, 1000000, 0, validator
//! account: bob, 1000000, 0, validator

//! new-transaction
//! sender: diemroot
script {
    
    use 0x1::DiemSystem;
    use 0x1::NodeWeight;
    use 0x1::ValidatorUniverse;
    fun main(_account: signer) {
        // Valida
        // Tests on initial size of validators 
        assert(DiemSystem::validator_set_size() == 2, 7357220101011000);
        assert(DiemSystem::is_validator({{alice}}) == true, 7357220101021000);
        assert(NodeWeight::proof_of_weight({{alice}}) == 0, 7357220101031000);
        assert(ValidatorUniverse::exists_jailedbit({{alice}}), 7357220101041000);
    }
}
// check: EXECUTED
