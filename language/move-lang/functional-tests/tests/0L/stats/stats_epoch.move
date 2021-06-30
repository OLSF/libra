// Case 1: Validators are compliant. 
// This test is to check if validators are present after the first epoch.
// Here EPOCH-LENGTH = 15 Blocks.
// NOTE: This test will fail with Staging and Production Constants, only for Debug - due to epoch length.

//! account: alice, 1000000, 0, validator
//! account: bob, 1000000, 0, validator
//! account: carol, 1000000, 0, validator
//! account: dave, 1000000, 0, validator

//! block-prologue
//! proposer: alice
//! block-time: 1

//! new-transaction
//! sender: diemroot
script {
    use 0x1::Stats;

    // Assumes an epoch changed at round 15
    fun main(vm: signer) {
      //proposals
      assert(Stats::node_current_props(vm, {{alice}}) == 1, 0);
      assert(Stats::node_current_props(vm, {{bob}}) == 0, 0);
      //votes
      assert(Stats::node_current_votes(vm, {{alice}}) == 0, 0);
      assert(Stats::node_current_votes(vm, {{bob}}) == 0, 0);

    }
}
// check: EXECUTED

///////// ADD A NEW PROPOSAL /////
//! block-prologue
//! proposer: alice
//! block-time: 2


//! new-transaction
//! sender: diemroot
script {
    use 0x1::Vector;
    use 0x1::Stats;
    // This is the the epoch boundary.
    fun main(vm: signer) {
      assert(Stats::node_current_props(vm, {{alice}}) == 2, 735700001);
      assert(Stats::node_current_props(vm, {{bob}}) == 0, 735700002);
      assert(Stats::node_current_votes(vm, {{alice}}) == 0, 735700003);
      assert(Stats::node_current_votes(vm, {{bob}}) == 0, 735700004);

      let voters = Vector::empty<address>();
      Vector::push_back<address>(&mut voters, {{alice}});
      Vector::push_back<address>(&mut voters, {{bob}});
      Vector::push_back<address>(&mut voters, {{carol}});
      Vector::push_back<address>(&mut voters, {{dave}});

        // Overwrite the statistics to mock that all have been validating.
        let i = 1;
        while (i < 16) {
            // Mock the validator doing work for 15 blocks, and stats being updated.
            Stats::process_set_votes(vm, &voters);
            i = i + 1;
        };

      assert(Stats::node_above_thresh(vm, {{alice}}, 0, 15), 735700005);
      assert(Stats::node_above_thresh(vm, {{bob}}, 0, 15), 735700006);
      assert(Stats::node_above_thresh(vm, {{carol}}, 0, 15), 735700007);
      assert(Stats::node_above_thresh(vm, {{dave}}, 0, 15), 735700008);

      assert(Stats::network_density(vm, 0, 15) == 4, 735700009);
    }
}
// check: EXECUTED

//////////////////////////////////////////////
///// Trigger reconfiguration at 61 seconds ////
//! block-prologue
//! proposer: alice
//! block-time: 61000000
//! round: 15

///// TEST RECONFIGURATION IS HAPPENING ////
// check: NewEpochEvent
//////////////////////////////////////////////


//! new-transaction
//! sender: diemroot
script {
    use 0x1::Stats;
    // use 0x1::Vector;
    fun main(vm: signer) {
      // Testing that reconfigure reset the counter for current epoch.
      assert(!Stats::node_above_thresh(vm, {{alice}}, 16, 17), 735700010);

      // should reset alice's count
      assert(Stats::node_current_props(vm, {{alice}}) == 0, 735700011);
      assert(Stats::node_current_props(vm, {{bob}}) == 0, 735700012);
      assert(Stats::node_current_votes(vm, {{alice}}) == 0, 735700013);
      assert(Stats::node_current_votes(vm, {{bob}}) == 0, 735700014);
    }
}
// check: EXECUTED
