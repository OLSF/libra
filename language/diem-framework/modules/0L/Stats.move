/////////////////////////////////////////////////////////////////////////
// 0L Module
// Stats Module
// Error code: 1900
/////////////////////////////////////////////////////////////////////////

address 0x1{
module Stats{
    use 0x1::CoreAddresses;
    use 0x1::Errors;
    use 0x1::FixedPoint32;
    use 0x1::Signer;
    use 0x1::Testnet;
    use 0x1::Vector;    

    struct SetData has copy, drop, store {
      addr: vector<address>,
      prop_count: vector<u64>,
      vote_count: vector<u64>,
      total_votes: u64,
      total_props: u64,
    }

    struct ValStats has copy, drop, key {
      history: vector<SetData>,
      current: SetData
    }

    //Permissions: Public, VM only.
    //Function: 01
    public fun initialize(vm: &signer) {
      let sender = Signer::address_of(vm);
      assert(sender == CoreAddresses::DIEM_ROOT_ADDRESS(), Errors::requires_role(190001));
       move_to<ValStats>(
        vm, 
        ValStats {
            history: Vector::empty(),
            current: blank()
          }
        );
    }
    
  fun blank():SetData {
    SetData {
        addr: Vector::empty(),
        prop_count: Vector::empty(),
        vote_count: Vector::empty(),
        total_votes: 0,
        total_props: 0,
      }
  }

    //Permissions: Public, VM only.
    //Function: 02
    public fun init_address(vm: &signer, node_addr: address) acquires ValStats {
      let sender = Signer::address_of(vm);

      assert(sender == CoreAddresses::DIEM_ROOT_ADDRESS(), Errors::requires_role(190002));

      let stats = borrow_global_mut<ValStats>(sender);
      let (is_init, _) = Vector::index_of<address>(&mut stats.current.addr, &node_addr);
      if (!is_init) {
        Vector::push_back(&mut stats.current.addr, node_addr);
        Vector::push_back(&mut stats.current.prop_count, 0);
        Vector::push_back(&mut stats.current.vote_count, 0);
      }
    }

    //Function: 03
    public fun init_set(vm: &signer, set: &vector<address>) acquires ValStats{
      let sender = Signer::address_of(vm);
      assert(sender == CoreAddresses::DIEM_ROOT_ADDRESS(), Errors::requires_role(190003));
      let length = Vector::length<address>(set);
      let k = 0;
      while (k < length) {
        let node_address = *(Vector::borrow<address>(set, k));
        init_address(vm, node_address);
        k = k + 1;
      }
    }

    //Function: 04
    public fun process_set_votes(vm: &signer, set: &vector<address>) acquires ValStats{
      let sender = Signer::address_of(vm);
      assert(sender == CoreAddresses::DIEM_ROOT_ADDRESS(), Errors::requires_role(190004));

      let length = Vector::length<address>(set);
      let k = 0;
      while (k < length) {
        let node_address = *(Vector::borrow<address>(set, k));
        inc_vote(vm, node_address);
        k = k + 1;
      }
    }

    //Permissions: Public, VM only.
    //Function: 05
    public fun node_current_votes(vm: &signer, node_addr: address): u64 acquires ValStats {
      let sender = Signer::address_of(vm);
      assert(sender == CoreAddresses::DIEM_ROOT_ADDRESS(), Errors::requires_role(190005));
      let stats = borrow_global_mut<ValStats>(sender);
      let (_, i) = Vector::index_of<address>(&mut stats.current.addr, &node_addr);
      *Vector::borrow<u64>(&mut stats.current.vote_count, i)
    }

    //Function: 06
    public fun node_above_thresh(vm: &signer, node_addr: address, height_start: u64, height_end: u64): bool acquires ValStats{
      let sender = Signer::address_of(vm);
      assert(sender == CoreAddresses::DIEM_ROOT_ADDRESS(), Errors::requires_role(190006));
      let range = height_end-height_start;
      let threshold_signing = FixedPoint32::multiply_u64(range, FixedPoint32::create_from_rational(1, 100));
      if (node_current_votes(vm, node_addr) >  threshold_signing) { return true };
      return false
    }

    //Function: 07
    public fun network_density(vm: &signer, height_start: u64, height_end: u64): u64 acquires ValStats {
      let sender = Signer::address_of(vm);
      assert(sender == CoreAddresses::DIEM_ROOT_ADDRESS(), Errors::requires_role(190007));
      let density = 0u64;
      let nodes = *&(borrow_global_mut<ValStats>(sender).current.addr);
      let len = Vector::length(&nodes);
      let k = 0;
      while (k < len) {
        let addr = *(Vector::borrow<address>(&nodes, k));
        if (node_above_thresh(vm, addr, height_start, height_end)) {
          density = density + 1;
        };
        k = k + 1;
      };
      return density
    }

    //Permissions: Public, VM only.
    //Function: 08    
    public fun node_current_props(vm: &signer, node_addr: address): u64 acquires ValStats {
      let sender = Signer::address_of(vm);
      assert(sender == CoreAddresses::DIEM_ROOT_ADDRESS(), Errors::requires_role(190008));
      let stats = borrow_global_mut<ValStats>(sender);
      let (_, i) = Vector::index_of<address>(&mut stats.current.addr, &node_addr);
      *Vector::borrow<u64>(&mut stats.current.prop_count, i)
    }

    //Permissions: Public, VM only.
    //Function: 09
    public fun inc_prop(vm: &signer, node_addr: address) acquires ValStats {
      let sender = Signer::address_of(vm);
      assert(sender == CoreAddresses::DIEM_ROOT_ADDRESS(), Errors::requires_role(190009));

      let stats = borrow_global_mut<ValStats>(sender);
      let (_, i) = Vector::index_of<address>(&mut stats.current.addr, &node_addr);
      let current_count = *Vector::borrow<u64>(&mut stats.current.prop_count, i);
      Vector::push_back(&mut stats.current.prop_count, current_count + 1);
      Vector::swap_remove(&mut stats.current.prop_count, i);
      stats.current.total_props = stats.current.total_props + 1;
    }
    
    //TODO: Duplicate code.
    //Permissions: Public, VM only.
    //Function: 10
    fun inc_vote(vm: &signer, node_addr: address) acquires ValStats {
      let sender = Signer::address_of(vm);
      assert(sender == CoreAddresses::DIEM_ROOT_ADDRESS(), Errors::requires_role(190010));
      let stats = borrow_global_mut<ValStats>(sender);
      let (_, i) = Vector::index_of<address>(&mut stats.current.addr, &node_addr);
      let test = *Vector::borrow<u64>(&mut stats.current.vote_count, i);
      Vector::push_back(&mut stats.current.vote_count, test + 1);
      Vector::swap_remove(&mut stats.current.vote_count, i);
      stats.current.total_votes = stats.current.total_votes + 1;
    }

    //Permissions: Public, VM only.
    //Function: 11
    public fun reconfig(vm: &signer, set: &vector<address>) acquires ValStats {
      let sender = Signer::address_of(vm);
      assert(sender == CoreAddresses::DIEM_ROOT_ADDRESS(), Errors::requires_role(190011));
      let stats = borrow_global_mut<ValStats>(sender);
      // Archive outgoing epoch stats.
      //TODO: limit the size of the history and drop ancient records.
      Vector::push_back(&mut stats.history, *&stats.current);

      stats.current = blank();
      
      init_set(vm, set);
    }

    //Function: 12
    public fun get_total_votes(vm: &signer): u64 acquires ValStats {
      let sender = Signer::address_of(vm);
      assert(sender == CoreAddresses::DIEM_ROOT_ADDRESS(), Errors::requires_role(190012));
      *&borrow_global_mut<ValStats>(CoreAddresses::DIEM_ROOT_ADDRESS()).current.total_votes
    }

    //Function: 13
    public fun get_total_props(vm: &signer): u64 acquires ValStats {
      let sender = Signer::address_of(vm);
      assert(sender == CoreAddresses::DIEM_ROOT_ADDRESS(), Errors::requires_role(190013));
      *&borrow_global_mut<ValStats>(CoreAddresses::DIEM_ROOT_ADDRESS()).current.total_props
    }

    //Function: 14
    public fun get_history(): vector<SetData> acquires ValStats {
      *&borrow_global_mut<ValStats>(CoreAddresses::DIEM_ROOT_ADDRESS()).history
    }

    /// TEST HELPERS
    //Function: 15
    public fun test_helper_inc_vote_addr(vm: &signer, node_addr: address) acquires ValStats {
      let sender = Signer::address_of(vm);
      assert(sender == CoreAddresses::DIEM_ROOT_ADDRESS(), Errors::requires_role(190015));

      assert(Testnet::is_testnet(), Errors::invalid_state(190015));
      inc_vote(vm, node_addr);
    }

}
}