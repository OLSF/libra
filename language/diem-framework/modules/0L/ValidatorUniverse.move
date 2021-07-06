///////////////////////////////////////////////////////////////////////////
// 0L Module
// ValidatorUniverse
///////////////////////////////////////////////////////////////////////////
// Stores all the validators who submitted a vdf proof.
// File Prefix for errors: 2201
///////////////////////////////////////////////////////////////////////////

address 0x1 {
  module ValidatorUniverse {
    use 0x1::CoreAddresses;
    use 0x1::Errors;
    use 0x1::MinerState;
    use 0x1::Signer;
    use 0x1::Testnet;
    use 0x1::Vector;
    
    // resource for tracking the universe of accounts that have submitted a mined proof correctly, with the epoch number.
    struct ValidatorUniverse has key {
        validators: vector<address>
    }

    struct JailedBit has key {
        is_jailed: bool
    }

    // Genesis function to initialize ValidatorUniverse struct in 0x0.
    // This is triggered in new epoch by Configuration in Genesis.move
    // Function code: 01 Prefix: 220101
    public fun initialize(account: &signer){
      // Check for transactions sender is association
      let sender = Signer::address_of(account);
      assert(sender == CoreAddresses::DIEM_ROOT_ADDRESS(), Errors::requires_role(220101));
      move_to<ValidatorUniverse>(account, ValidatorUniverse {
          validators: Vector::empty<address>()
      });
    }

    // This function is called to add validator to the validator universe.
    // Function code: 02 Prefix: 220102
    // TODO: This is public, anyone can add themselves to the validator universe.
    public fun add_self(sender: &signer) acquires ValidatorUniverse, JailedBit {
      let addr = Signer::address_of(sender);
      // Miner can only add self to set if the mining is above a threshold.
      assert(MinerState::node_above_thresh(sender, addr), 220102014010);
      add(sender);
    }

    fun add(sender: &signer) acquires ValidatorUniverse, JailedBit {
      let addr = Signer::address_of(sender);
      let state = borrow_global_mut<ValidatorUniverse>(CoreAddresses::DIEM_ROOT_ADDRESS());
      let (in_set, _) = Vector::index_of<address>(&state.validators, &addr);
      if (!in_set) {
        Vector::push_back<address>(&mut state.validators, addr);
        unjail(sender);
      }
    }

    // Permissions: Public, VM Only
    public fun remove_validator_vm(vm: &signer, validator: address) acquires ValidatorUniverse {
      assert(Signer::address_of(vm) == CoreAddresses::DIEM_ROOT_ADDRESS(), 220101014010);

      let state = borrow_global_mut<ValidatorUniverse>(CoreAddresses::DIEM_ROOT_ADDRESS());
      let (in_set, index) = Vector::index_of<address>(&state.validators, &validator);
      if (in_set) {
        Vector::remove<address>(&mut state.validators, index);
      }
    }

    // Permissions: Public, Anyone.
    // Can only remove self from validator list.
    public fun remove_self(validator: &signer) acquires ValidatorUniverse {
      let val = Signer::address_of(validator);
      let state = borrow_global_mut<ValidatorUniverse>(CoreAddresses::DIEM_ROOT_ADDRESS());
      let (in_set, index) = Vector::index_of<address>(&state.validators, &val);
      if (in_set) {
        Vector::remove<address>(&mut state.validators, index);
      }
    }

    // A simple public function to query the EligibleValidators.
    // Function code: 03 Prefix: 220103
    public fun get_eligible_validators(vm: &signer): vector<address> acquires ValidatorUniverse {
      assert(Signer::address_of(vm) == CoreAddresses::DIEM_ROOT_ADDRESS(), Errors::requires_role(220103));
      let state = borrow_global<ValidatorUniverse>(CoreAddresses::DIEM_ROOT_ADDRESS());
      *&state.validators
    }

    // Is a candidate for validation
    public fun is_in_universe(miner: address): bool acquires ValidatorUniverse {
      let state = borrow_global<ValidatorUniverse>(CoreAddresses::DIEM_ROOT_ADDRESS());
      Vector::contains<address>(&state.validators, &miner)
    }

    public fun jail(vm: &signer, validator: address) acquires JailedBit{
      assert(Signer::address_of(vm) == CoreAddresses::DIEM_ROOT_ADDRESS(), 220101014010);

      borrow_global_mut<JailedBit>(validator).is_jailed = true;
    }

    public fun unjail_self(sender: &signer) acquires JailedBit {
      // only a validator can un-jail themselves.
      let validator = Signer::address_of(sender);
      // check the node has been mining before unjailing.
      assert(MinerState::node_above_thresh(sender, validator), 220102014010);
      unjail(sender);
    }

    fun unjail(sender: &signer) acquires JailedBit {
      let addr = Signer::address_of(sender);
      if (!exists<JailedBit>(addr)) {
        move_to<JailedBit>(sender, JailedBit{
          is_jailed: false
        });
      };

      borrow_global_mut<JailedBit>(addr).is_jailed = false;
    }

    public fun is_jailed(validator: address): bool acquires JailedBit {
      if (!exists<JailedBit>(validator)) {
        return false
      };
      borrow_global<JailedBit>(validator).is_jailed
    }

    public fun genesis_helper(vm: &signer, validator: &signer) acquires ValidatorUniverse, JailedBit {
      assert(Signer::address_of(vm) == CoreAddresses::DIEM_ROOT_ADDRESS(), 220101014010);
      // let addr = Signer::address_of(sender);
      // MinerState::node_above_thresh(sender, addr);
      add(validator);
    }

    //////// TEST ////////
    public fun exists_jailedbit(addr: address): bool {
      assert(Testnet::is_testnet()== true, 130115014011);
      exists<JailedBit>(addr)
    }

    public fun test_helper_add_self_onboard(vm: &signer, addr:address) acquires ValidatorUniverse {
      assert(Testnet::is_testnet()== true, 130115014011);
      assert(Signer::address_of(vm) == CoreAddresses::DIEM_ROOT_ADDRESS(), 220101015010);
      let state = borrow_global_mut<ValidatorUniverse>(CoreAddresses::DIEM_ROOT_ADDRESS());
      Vector::push_back<address>(&mut state.validators, addr);
    }
  }
}