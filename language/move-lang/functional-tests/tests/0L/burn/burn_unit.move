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
    LibraAccount::vm_make_payment_no_limit<GAS>( {{alice}}, {{bob}}, 100, x"", x"", vm);
    // send to community wallet Carol
    LibraAccount::vm_make_payment_no_limit<GAS>( {{alice}}, {{carol}}, 300, x"", x"", vm);

    let bal = LibraAccount::balance<GAS>({{bob}});
    assert(bal == 100, 7357003);
    let bal = LibraAccount::balance<GAS>({{carol}});
    assert(bal == 300, 7357004);

    Burn::reset_ratios(vm);
    let (addr, _ , ratios) = Burn::get_ratios();
    assert(Vector::length(&addr) == 2, 7357005);

    let carol_mult = *Vector::borrow<FixedPoint32::FixedPoint32>(&ratios, 1);
    let pct_carol = FixedPoint32::multiply_u64(100, carol_mult);

    // ratio for carol's community wallet.
    assert(pct_carol == 75, 7357006);

    Burn::epoch_start_burn(vm, {{alice}}, 100);

    let bal_alice = LibraAccount::balance<GAS>({{alice}});
    assert(bal_alice == 999501, 7357007);
    // rounding issues
    let bal_bob = LibraAccount::balance<GAS>({{bob}});
    assert(bal_bob == 124, 7357008);

    let bal_carol = LibraAccount::balance<GAS>({{carol}});
    assert(bal_carol == 375, 7357009);
  }
}
// check: EXECUTED

