//! account: alice, 1, 0, validator

//! new-transaction
//! sender: diemroot
script {
    use 0x1::TransactionFee;

    fun main(vm: signer) {
        assert(TransactionFee::get_amount_to_distribute(&vm) == 0, 735701);
    }
}
