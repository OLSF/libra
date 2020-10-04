//! account: alice, 1000000, 0, validator
//! account: bob, 1000000

// The data will be initialized and operated all through alice's account

//! new-transaction
//! sender: association
script {
    use 0x0::AltStats;
    // use 0x0::Transaction;
    // use 0x0::Debug::print;

    fun main(){
      // Checks that altstats was initialized in genesis for Alice.

      // AltStats::initialize();

      AltStats::insert_prop({{alice}});
      AltStats::inc_prop({{alice}});
      AltStats::inc_prop({{alice}});

      AltStats::insert_prop({{bob}});
      AltStats::inc_prop({{bob}});
      AltStats::inc_prop({{bob}});
      
      AltStats::inc_vote({{alice}});
      AltStats::inc_vote({{alice}});

      AltStats::reconfig();


      // AltStats::inc_vote({{bob}});


      // Transaction::assert(AltStats::length() == 1, 0);      Transaction::assert(AltStats::get(0) == {{alice}}, 0);
      // Transaction::assert(AltStats::contains({{alice}}), 1);
    }
}
// check: EXECUTED