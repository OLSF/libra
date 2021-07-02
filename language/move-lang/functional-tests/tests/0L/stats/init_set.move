//! account: alice, 1000000, 0, validator
//! account: bob, 1000000, 0, validator

// The data will be initialized and operated all through alice's account

//! new-transaction
//! sender: diemroot
script {
    use 0x1::Stats;
    use 0x1::Vector;

    fun main(vm: signer){
      let vm = &vm;
      // Checks that stats was initialized in genesis for Alice.
      let set = Vector::singleton({{alice}});
      Vector::push_back(&mut set, {{bob}});

      Stats::init_set(vm, &set);

      assert(Stats::node_current_props(vm, {{alice}}) == 0, 0);
      assert(Stats::node_current_props(vm, {{bob}}) == 0, 0);
      assert(Stats::node_current_votes(vm, {{alice}}) == 0, 0);
      assert(Stats::node_current_votes(vm, {{bob}}) == 0, 0);
    }
}
// check: EXECUTED