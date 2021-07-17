//! account: alice, 1000000, 0, validator
//! account: bob, 1000000, 0, validator

// The data will be initialized and operated all through alice's account

//! new-transaction
//! sender: libraroot
script {
    use 0x1::Stats;
    use 0x1::Vector;
    use 0x1::Reconfigure;

    fun main(vm: &signer){
      // Check that after a reconfig the counter is reset, and archived in history.
      assert(Stats::node_current_props(vm, {{alice}}) == 0, 7357008014001);
      assert(Stats::node_current_props(vm, {{bob}}) == 0, 7357008014002);
      assert(Stats::node_current_votes(vm, {{alice}}) == 0, 7357008014003);
      assert(Stats::node_current_votes(vm, {{bob}}) == 0, 7357008014004);


      Stats::inc_prop(vm, {{alice}});
      Stats::inc_prop(vm, {{alice}});

      Stats::inc_prop(vm, {{bob}});
      
      Stats::test_helper_inc_vote_addr(vm, {{alice}});
      Stats::test_helper_inc_vote_addr(vm, {{alice}});

      assert(Stats::node_current_props(vm, {{alice}}) == 2, 7357008014005);
      assert(Stats::node_current_props(vm, {{bob}}) == 1, 7357008014006);
      assert(Stats::node_current_votes(vm, {{alice}}) == 2, 7357008014007);
      assert(Stats::node_current_votes(vm, {{bob}}) == 0, 7357008014008);


      let set = Vector::empty<address>();
      Vector::push_back<address>(&mut set, {{alice}});
      Vector::push_back<address>(&mut set, {{bob}});


      Reconfigure::reconfigure(vm, 15); // reconfigure at height 15

      assert(Stats::node_current_props(vm, {{alice}}) == 0, 7357008014009);
      assert(Stats::node_current_props(vm, {{bob}}) == 0, 7357008014010);
    }
}
// check: EXECUTED