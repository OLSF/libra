// Case 1: Validators are compliant. 
// This test is to check if validators are present after the first epoch.
// Here EPOCH-LENGTH = 15 Blocks.
// NOTE: This test will fail with Staging and Production Constants, only for Debug - due to epoch length.

//! account: alice, 1000000, 0, validator
//! account: bob, 1000000, 0, validator
//! account: carol, 1000000, 0, validator
//! account: dave, 1000000, 0, validator

//! new-transaction
//! sender: diemroot
script {
    use 0x1::Vector;
    use 0x1::Stats;


    // Assumes an epoch changed at round 15
    
    fun main(vm: signer) {
      assert(Stats::node_current_props(vm, {{bob}}) == 0, 0);
      assert(Stats::node_current_votes(vm, {{alice}}) == 0, 0);
      assert(Stats::node_current_votes(vm, {{bob}}) == 0, 0);

      let voters = Vector::empty<address>();
      Vector::push_back<address>(&mut voters, {{alice}});
      Vector::push_back<address>(&mut voters, {{bob}});
      Vector::push_back<address>(&mut voters, {{carol}});
      Vector::push_back<address>(&mut voters, {{dave}});

      // Testing Below Threshold
      let i = 1;
      while (i < 5) {
          // Below threshold: Mock 4 blocks of 500 blocks
          Stats::process_set_votes(vm, &voters);
          i = i + 1;
      };

      assert(!Stats::node_above_thresh(vm, {{alice}}, 0, 500), 735701);
      assert(Stats::network_density(vm, 0, 500) == 0, 735702);

      // Testing Above Threshold
      let i = 1;
      while (i < 2) {
          // Mock additional 2 blocks validated out of 500.
          Stats::process_set_votes(vm, &voters);
          i = i + 1;
      };

      assert(Stats::node_above_thresh(vm, {{alice}}, 0, 500), 735703);
      assert(Stats::network_density(vm, 0, 500) == 4, 735704);
    }
}
// check: EXECUTED
