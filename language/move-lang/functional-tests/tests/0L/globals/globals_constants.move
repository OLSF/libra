//! account: alice, 100000,0, validator
//! new-transaction
//! sender: diemroot
script {
use 0x1::Globals;
use 0x1::Testnet;
use 0x1::DiemSystem;

    fun main(_sender: signer) {
        assert(DiemSystem::is_validator({{alice}}) == true, 98);

        let len = Globals::get_epoch_length();
        let set = DiemSystem::validator_set_size();
        
        assert(set == 1u64, 73570001);

        if (Testnet::is_testnet()){
            assert(len == 60u64, 73570001);
        } else {
            assert(len == 196992u64, 73570001);
        }
    }
}
// check: EXECUTED