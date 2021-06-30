// Do not add validators here, the settings added here will overwrite the genesis defaults which is what we are checking for.

//! new-transaction
//! sender: diemroot
script {
    use 0x1::DiemSystem;
    // use 0x1::DiemAccount;
    // use 0x1::GAS::GAS;
    use 0x1::TrustedAccounts;
    use 0x1::Vector;

    fun main(_account: signer) {
        let num_validators = DiemSystem::validator_set_size();
        let index = 0;
        while (index < num_validators) {
            let addr = DiemSystem::get_ith_validator_address(index);
                let (test, _ ) = TrustedAccounts::get_trusted(addr);
                let len = Vector::length<address>(&test);
                assert(len == 0, 7357130101051000);
            index = index + 1;
        };
    }
}
// check: "Keep(EXECUTED)"