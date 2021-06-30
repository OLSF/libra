// Do not add validators here, the settings added here will overwrite the genesis defaults which is what we are checking for.
//! account: alice, 1, 0, validator
//! account: bob, 1, 0
//! account: carol, 1, 0


//! new-transaction
//! sender: alice
script {
    use 0x1::Signer;
    use 0x1::TrustedAccounts;
    use 0x1::Vector;

    fun main(account: signer) {
        let addr = Signer::address_of(account);
        let (my_before, _ ) = TrustedAccounts::get_trusted(addr);
        let len = Vector::length<address>(&my_before);
        assert(len == 0, 7357130101011000);

        let vec_my = Vector::singleton<address>({{alice}});
        let vec_follow = Vector::singleton<address>({{bob}});

        TrustedAccounts::update(
            account,
            vec_my, //update_my
            vec_follow, //update_follow
        );

        let (my_after, _ ) = TrustedAccounts::get_trusted(addr);
        let len = Vector::length<address>(&my_after);
        assert(len == 1, 7357130101021000);
    }
}
// check: "Keep(EXECUTED)"