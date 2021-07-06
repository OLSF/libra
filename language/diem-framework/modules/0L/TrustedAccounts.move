address 0x1 {
///////////////////////////////////////////////////////////////////////////
// 
// File Prefix for errors: 2001
///////////////////////////////////////////////////////////////////////////
module TrustedAccounts {
    use 0x1::Errors;
    use 0x1::Signer;
    use 0x1::Vector;

    struct Trusted has key {
      my_trusted_accounts: vector<address>,
      follow_operators_trusting_accounts: vector<address>
    }

    public fun initialize(account: &signer) {
      move_to<Trusted>(account, Trusted{
        my_trusted_accounts: Vector::empty(),
        follow_operators_trusting_accounts: Vector::empty()
      });
    }

    public fun update(account: &signer, update_my: vector<address>, update_follow: vector<address>) acquires Trusted{
      // TODO: Check exists
      // exists_at(payee)
      let state = borrow_global_mut<Trusted>(Signer::address_of(account));
      state.my_trusted_accounts = update_my;
      state.follow_operators_trusting_accounts = update_follow;
    }


    //////// PUBLIC GETTERS ////////
    // Function code: 01
    public fun get_trusted(account: address): (vector<address>, vector<address>) acquires Trusted{
      assert(exists<Trusted>(account), Errors::not_published(200101));
      let state = borrow_global<Trusted>(account);
      (*&state.my_trusted_accounts, *&state.follow_operators_trusting_accounts)
    }
  }
}
