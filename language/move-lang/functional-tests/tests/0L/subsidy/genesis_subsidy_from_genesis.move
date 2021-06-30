// Do not add validators here, the settings added here will overwrite the genesis defaults which is what we are checking for.

//! new-transaction
//! sender: diemroot
script {
    use 0x1::DiemSystem;
    use 0x1::DiemAccount;
    use 0x1::GAS::GAS;
    use 0x1::Debug::print;

    fun main(_account: signer) {
        let num_validators = DiemSystem::validator_set_size();
        let index = 0;
        while (index < num_validators) {
            let addr = DiemSystem::get_ith_validator_address(index);
            print(&DiemAccount::balance<GAS>(addr));
            assert(DiemAccount::balance<GAS>(addr) == 2497536, 7357001);
            index = index + 1;
        };
    }
}
// check: "Keep(EXECUTED)"