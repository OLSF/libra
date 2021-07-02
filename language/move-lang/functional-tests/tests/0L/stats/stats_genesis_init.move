// TODO: Not sure how this is different from increment.move test

//! account: alice, 1000000, 0, validator
//! account: bob, 1000000, 0, validator

// The data will be initialized and operated all through alice's account

//! new-transaction
//! sender: diemroot
script {
    use 0x1::Stats;

    fun main(vm: signer){
      // Checks that stats was initialized in genesis for Alice.

      let vm = &vm;
      assert(Stats::node_current_props(vm, {{alice}}) == 0, 7357190201011000);
      assert(Stats::node_current_props(vm, {{bob}}) == 0, 7357190201021000);
      assert(Stats::node_current_votes(vm, {{alice}}) == 0, 7357190201031000);
      assert(Stats::node_current_votes(vm, {{bob}}) == 0, 7357190201014000);


      Stats::inc_prop(vm, {{alice}});
      Stats::inc_prop(vm, {{alice}});
      Stats::inc_prop(vm, {{bob}});
      
      Stats::test_helper_inc_vote_addr(vm, {{alice}});
      Stats::test_helper_inc_vote_addr(vm, {{alice}});

      assert(Stats::node_current_props(vm, {{alice}}) == 2, 7357190202011000);
      assert(Stats::node_current_props(vm, {{bob}}) == 1, 7357190202021000);
      assert(Stats::node_current_votes(vm, {{alice}}) == 2, 7357190202031000);
      assert(Stats::node_current_votes(vm, {{bob}}) == 0, 7357190202041000);
    }
}
// check: EXECUTED