//! account: alice, 1, 0, validator

//! new-transaction
//! sender: diemroot
script {
    use 0x1::TransactionFee;
    use 0x1::Diem;
    use 0x1::GAS::GAS;

    fun main(vm: signer) {
        assert(TransactionFee::get_amount_to_distribute(&vm)==0, 735701);
        let coin = Diem::mint<GAS>(&vm, 1);
        TransactionFee::pay_fee(coin);
        assert(TransactionFee::get_amount_to_distribute(&vm)==1, 735701);

    }
}