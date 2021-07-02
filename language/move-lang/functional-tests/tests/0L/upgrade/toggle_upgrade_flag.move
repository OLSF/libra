//! new-transaction
//! sender: diemroot
script {
    use 0x1::Upgrade;

    fun main(sender: signer) {
        assert(Upgrade::has_upgrade() == false, 1);

        Upgrade::set_update(&sender, x"1234");
        assert(Upgrade::has_upgrade() == true, 1);
        assert(Upgrade::get_payload() == x"1234", 1);
    }
}
// check: EXECUTED