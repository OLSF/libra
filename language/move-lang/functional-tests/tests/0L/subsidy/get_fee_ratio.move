//! account: alice, 1, 0, validator
//! account: bob, 1, 0, validator

//! new-transaction
//! sender: alice
script {
    use 0x1::MinerState;
    fun main(sender: signer) {
        MinerState::test_helper_mock_mining(&sender, 5);
    }
}
//check: EXECUTED

//! new-transaction
//! sender: bob
script {
    use 0x1::MinerState;
    fun main(sender: signer) {
        MinerState::test_helper_mock_mining(&sender, 5);
    }
}
//check: EXECUTED

//! new-transaction
//! sender: diemroot
script {
  use 0x1::Vector;
  use 0x1::Stats;
  use 0x1::FixedPoint32;
  use 0x1::DiemSystem;

  fun main(vm: signer) {
    // check the case of a network density of 4 active validators.

    let vm = &vm;
    let voters = Vector::singleton<address>({{alice}});
    Vector::push_back(&mut voters, {{bob}});

    // create mock validator stats for full epoch
    let i = 0;
    while (i < 16) {
      Stats::process_set_votes(vm, &voters);
      Stats::inc_prop(vm, {{alice}});
      Stats::inc_prop(vm, {{bob}});
      i = i + 1;
    };

    let (validators, fee_ratios) = DiemSystem::get_fee_ratio(vm, 0, 15);
    assert(Vector::length(&validators) == 2, 735701);
    assert(Vector::length(&fee_ratios) == 2, 735702);
    assert(
      *(Vector::borrow<FixedPoint32::FixedPoint32>(&fee_ratios, 1)) 
        == FixedPoint32::create_from_raw_value(2147483648u64),
      735703
    );

  }
}
// check: EXECUTED