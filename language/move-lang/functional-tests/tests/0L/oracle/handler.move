// run with language/move-lang/functional-tests> cargo test handler

//! account: alice, 1000000, 0, validator

//! new-transaction
//! sender: alice
script {
    use 0x1::Oracle;
    use 0x1::Vector;
    use 0x1::Upgrade;
    fun main(sender: signer){
        let id = 1;
        let data = b"hello";
        Oracle::handler(sender, id, data);
        let vec = Oracle::test_helper_query_oracle_votes();

        let e = *Vector::borrow<address>(&vec, 0);
        assert(e == {{alice}}, 7357123401011000);

        assert(Upgrade::has_upgrade() == false, 7357123401011000); 
    }
}
// check: EXECUTED
