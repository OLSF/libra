address 0x1 {
///////////////////////////////////////////////////////////////////////////
// File Prefix for errors: 0600
///////////////////////////////////////////////////////////////////////////
module FullnodeState {
  use 0x1::CoreAddresses;
  use 0x1::Errors;
  use 0x1::Signer;
  use 0x1::Testnet::is_testnet;
  
  struct FullnodeCounter has key {
    proofs_submitted_in_epoch: u64,
    proofs_paid_in_epoch: u64,
    subsidy_in_epoch: u64,
    cumulative_proofs_submitted: u64,
    cumulative_proofs_paid: u64,
    cumulative_subsidy: u64,
  }

  //Function code: 01
  public fun init(sender: &signer) {
      assert(!exists<FullnodeCounter>(Signer::address_of(sender)), Errors::not_published(060001));
      move_to<FullnodeCounter>(
      sender, 
      FullnodeCounter {
          proofs_submitted_in_epoch: 0,
          proofs_paid_in_epoch: 0, // count
          subsidy_in_epoch: 0, // value
          cumulative_proofs_submitted: 0,
          cumulative_proofs_paid: 0,
          cumulative_subsidy: 0,
        }
      );
  }

  /// On recongfiguration events, reset.
  // Function code: 2
  public fun reconfig(vm: &signer, addr: address, proofs_in_epoch: u64) acquires FullnodeCounter {
      let sender = Signer::address_of(vm);
      assert(sender == CoreAddresses::DIEM_ROOT_ADDRESS(), Errors::requires_role(060001));
      let state = borrow_global_mut<FullnodeCounter>(addr);
      state.cumulative_proofs_submitted = state.cumulative_proofs_submitted + proofs_in_epoch;
      state.cumulative_proofs_paid = state.cumulative_proofs_paid + state.proofs_paid_in_epoch;
      state.cumulative_subsidy = state.cumulative_subsidy + state.subsidy_in_epoch;
      // reset 
      state.proofs_submitted_in_epoch = proofs_in_epoch;
      state.proofs_paid_in_epoch = 0;
      state.subsidy_in_epoch = 0;
  }

  // /// Miner increments proofs by 1
  // /// TO
  // public fun inc_proof(sender: &signer) acquires FullnodeCounter {
  //     let addr = Signer::address_of(sender);
  //     let state = borrow_global_mut<FullnodeCounter>(addr);
  //     state.proofs_submitted_in_epoch = state.proofs_submitted_in_epoch + 1;
  // }

  // /// Miner increments proofs by 1
  // //Function Code:03
  // public fun inc_proof_by_operator(operator_sig: &signer, miner_addr: address) acquires FullnodeCounter {
  //   assert(ValidatorConfig::get_operator(miner_addr) == Signer::address_of(operator_sig), Errors::requires_role(0600103));
  //     let state = borrow_global_mut<FullnodeCounter>(miner_addr);
  //     state.proofs_submitted_in_epoch = state.proofs_submitted_in_epoch + 1;
  // }

  /// VM Increments payments in epoch. Increases by `count`
  // Function code:04
  public fun inc_payment_count(vm: &signer, addr: address, count: u64) acquires FullnodeCounter {
    assert(Signer::address_of(vm) == CoreAddresses::DIEM_ROOT_ADDRESS(), Errors::requires_role(060004));
    let state = borrow_global_mut<FullnodeCounter>(addr);
    state.proofs_paid_in_epoch = state.proofs_paid_in_epoch + count;
  }

  /// VM Increments payments in epoch. Increases by `count`
  //Function code:05
  public fun inc_payment_value(vm: &signer, addr: address, value: u64) acquires FullnodeCounter {
    assert(Signer::address_of(vm) == CoreAddresses::DIEM_ROOT_ADDRESS(), Errors::requires_role(060005));
    let state = borrow_global_mut<FullnodeCounter>(addr);
    state.subsidy_in_epoch = state.subsidy_in_epoch + value;
  }

  public fun is_init(addr: address): bool {
    exists<FullnodeCounter>(addr)
  }


  public fun is_onboarding(addr: address): bool acquires FullnodeCounter{
    let state = borrow_global<FullnodeCounter>(addr);

    state.cumulative_proofs_submitted < 2 &&
    state.cumulative_proofs_paid < 2 &&
    state.cumulative_subsidy < 1000000
  }

  //////// GETTERS /////////

  public fun get_address_proof_count(addr:address): u64 acquires FullnodeCounter {
    borrow_global<FullnodeCounter>(addr).proofs_submitted_in_epoch
  }

  public fun get_cumulative_subsidy(addr: address): u64 acquires FullnodeCounter{
    let state = borrow_global<FullnodeCounter>(addr);
    state.cumulative_subsidy
  }

  //////// TEST HELPERS /////////
  
  // Function code:06
  public fun test_set_fullnode_fixtures(
    vm: &signer,
    addr: address,
    proofs_submitted_in_epoch: u64,
    proofs_paid_in_epoch: u64,
    subsidy_in_epoch: u64,
    cumulative_proofs_submitted: u64,
    cumulative_proofs_paid: u64,
    cumulative_subsidy: u64,
  ) acquires FullnodeCounter {
    CoreAddresses::assert_diem_root(vm);
    assert(is_testnet(), Errors::invalid_state(060006));

    let state = borrow_global_mut<FullnodeCounter>(addr);
    state.proofs_submitted_in_epoch = proofs_submitted_in_epoch;
    state.proofs_paid_in_epoch = proofs_paid_in_epoch;
    state.subsidy_in_epoch = subsidy_in_epoch;
    state.cumulative_proofs_submitted = cumulative_proofs_submitted;
    state.cumulative_proofs_paid = cumulative_proofs_paid;
    state.cumulative_subsidy = cumulative_subsidy;
  }

  /// Testhelper
  public fun mock_proof(sender: &signer, count: u64) acquires FullnodeCounter {
    let addr = Signer::address_of(sender);
    let state = borrow_global_mut<FullnodeCounter>(addr);
    state.proofs_submitted_in_epoch = state.proofs_submitted_in_epoch + count;
  }
}
}