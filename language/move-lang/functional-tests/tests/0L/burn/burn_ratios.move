//! account: alice, 1000000, 0, validator
//! account: bob, 0
//! account: carol, 0

//! new-transaction
//! sender: bob
script {
    use 0x1::Wallet;
    use 0x1::Vector;
    use 0x1::LibraAccount;

    fun main(sender: &signer) {
      Wallet::set_comm(sender);
      LibraAccount::init_cumulative_deposits(sender);
      let list = Wallet::get_comm_list();
      assert(Vector::length(&list) == 1, 7357001);
    }
}

// check: EXECUTED

//! new-transaction
//! sender: carol
script {
    use 0x1::Wallet;
    use 0x1::Vector;
    use 0x1::LibraAccount;

    fun main(sender: &signer) {
      Wallet::set_comm(sender);
      LibraAccount::init_cumulative_deposits(sender);
      let list = Wallet::get_comm_list();
      assert(Vector::length(&list) == 2, 7357002);
    }
}

// check: EXECUTED

//! new-transaction
//! sender: libraroot
script {
  use 0x1::LibraAccount;
  use 0x1::GAS::GAS;
  use 0x1::Burn;
  use 0x1::Vector;
  use 0x1::FixedPoint32;

  fun main(vm: &signer) {
    // send to community wallet Bob
    LibraAccount::vm_make_payment_no_limit<GAS>( {{alice}}, {{bob}}, 100000, x"", x"", vm);

    // send to community wallet Carol
    LibraAccount::vm_make_payment_no_limit<GAS>( {{alice}}, {{carol}}, 300000, x"", x"", vm);

    Burn::reset_ratios(vm);
    let (addr, deps , ratios) = Burn::get_ratios();
    assert(Vector::length(&addr) == 2, 7357003);
    assert(Vector::length(&deps) == 2, 7357004);
    assert(Vector::length(&ratios) == 2, 7357005);

    let bob_deposits_indexed = *Vector::borrow<u64>(&deps, 0);
    assert(bob_deposits_indexed == 100500, 7357006);
    let carol_deposits_indexed = *Vector::borrow<u64>(&deps, 1);
    assert(carol_deposits_indexed == 301500, 7357007);

    let bob_mult = *Vector::borrow<FixedPoint32::FixedPoint32>(&ratios, 0);
    let pct_bob = FixedPoint32::multiply_u64(100, bob_mult);
    // ratio for bob's community wallet.
    assert(pct_bob == 25, 7357008);

    let carol_mult = *Vector::borrow<FixedPoint32::FixedPoint32>(&ratios, 1);
    let pct_carol = FixedPoint32::multiply_u64(100, carol_mult);

    // ratio for carol's community wallet.
    assert(pct_carol == 75, 7357009);
  }
}
// check: EXECUTED

