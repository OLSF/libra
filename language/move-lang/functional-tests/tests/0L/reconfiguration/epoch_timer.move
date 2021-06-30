//! account: alice, 1000000, 0, validator

// Tests the prologue reconfigures based on wall clock

//! block-prologue
//! proposer: alice
//! block-time: 1
//! round: 1


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
    use 0x1::Epoch;
    use 0x1::DiemTimestamp;
    fun main(){
      // the new epoch has reset the timer.
      assert(DiemTimestamp::now_seconds() == 61, 735701);
      assert(!Epoch::epoch_finished(), 735702);
    }
}
// check: EXECUTED