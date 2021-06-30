//! account: alice, 1000000, 0, validator
//! account: bob, 1000000, 0, validator

// The data will be initialized and operated all through alice's account

//! new-transaction
//! sender: diemroot
script {
    use 0x1::Stats;
    use 0x1::Vector;
    

    fun main(vm: signer){
      // Checks that altstats was initialized in genesis for Alice.
      let set = Vector::singleton({{alice}});
      Vector::push_back(&mut set, {{bob}});

      Stats::process_set_votes(vm, &set);

      assert(Stats::node_current_props(vm, {{alice}}) == 0, 0);
      assert(Stats::node_current_props(vm, {{bob}}) == 0, 0);
      assert(Stats::node_current_votes(vm, {{alice}}) == 1, 0);
      assert(Stats::node_current_votes(vm, {{bob}}) == 1, 0);
    }
}
// check: EXECUTED