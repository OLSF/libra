//! account: alice, 1000000, 0 , validator

//! new-transaction
//! sender: diemroot
script {
  
  use 0x1::Subsidy;

  fun main(_vm: signer) {
    let subsidy_ceiling_gas = 296;
    let network_density = 4;
    let max_node_count = 300;
    assert(Subsidy::subsidy_curve(subsidy_ceiling_gas, network_density, max_node_count) == 296, 7357190101021000);

    }
}
// check: EXECUTED
