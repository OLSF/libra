//! account: alice, 1000000, 0, validator
//! account: bob, 1000000, 0, validator

// The data will be initialized and operated all through alice's account

//! new-transaction
//! sender: diemroot
script {
    use 0x1::Stats;
    use 0x1::Vector;

    fun main(vm: signer){
      // Check that after a reconfig the counter is reset, and archived in history.
      
      let vm = &vm;
      assert(Stats::node_current_props(vm, {{alice}}) == 0, 7357190201011000);
      assert(Stats::node_current_props(vm, {{bob}}) == 0, 7357190201021000);
      assert(Stats::node_current_votes(vm, {{alice}}) == 0, 7357190201031000);
      assert(Stats::node_current_votes(vm, {{bob}}) == 0, 7357190201041000);


      Stats::inc_prop(vm, {{alice}});
      Stats::inc_prop(vm, {{alice}});

      Stats::inc_prop(vm, {{bob}});
      
      Stats::test_helper_inc_vote_addr(vm, {{alice}});
      Stats::test_helper_inc_vote_addr(vm, {{alice}});

      assert(Stats::node_current_props(vm, {{alice}}) == 2, 7357190201051000);
      assert(Stats::node_current_props(vm, {{bob}}) == 1, 7357190201061000);

      assert(Stats::node_current_votes(vm, {{alice}}) == 2, 7357190201071000);
      assert(Stats::node_current_votes(vm, {{bob}}) == 0, 7357190201081000);


      let set = Vector::empty<address>();
      Vector::push_back<address>(&mut set, {{alice}});
      Vector::push_back<address>(&mut set, {{bob}});


      Stats::reconfig(vm, &set);

      assert(Stats::node_current_props(vm, {{alice}}) == 0, 0);
      assert(Stats::node_current_props(vm, {{bob}}) == 0, 0);
    }
}
// check: EXECUTED