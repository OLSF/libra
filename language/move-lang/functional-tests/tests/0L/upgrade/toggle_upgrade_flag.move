//! new-transaction
//! sender: diemroot
script {
    use 0x1::Upgrade;

    fun main(s: signer) {
        assert(Upgrade::has_upgrade() == false, 1);

        Upgrade::set_update(s, x"123");
        assert(Upgrade::has_upgrade() == true, 1);
        assert(Upgrade::get_payload() == x"123", 1);
    }
}
// check: EXECUTED