// This module is used to select the validators who would participate in LibraBFT protocol. Due to the restrictions on throughput with increasing validators above a threshold,
// we rank nodes based on node weight (i.e., stake they own, previous participation trends) to select the validators for an epoch.
address 0x0 {
  module NodeWeight {

    use 0x0::Vector;
    use 0x0::ValidatorUniverse;
    use 0x0::Signer;
    use 0x0::Transaction;
    
    public fun top_n_accounts(account: &signer, n: u64): vector<address> {
      let sender = Signer::address_of(account);
      Transaction::assert(sender == 0xA550C18 || sender == 0x0, 8001);

      //Get eligible validators from Redeem
      let eligible_validators = ValidatorUniverse::get_eligible_validators(account);

      let length = Vector::length<address>(&eligible_validators);

      //BASE CASE
      // If n is greater than or equal to accounts vector length - return the vector.
      if(length<=n)
        return eligible_validators;

      // Vector to store node_weights
      let weights = Vector::empty<u64>();
      let k = 0;
      while (k < length) {
          let cur_address = Vector::borrow<address>(&eligible_validators, k);
          Vector::push_back<u64>(&mut weights, ValidatorUniverse::update_validator_weight({{*cur_address}}, k));
          k = k + 1;
      };

      // Sorting the accounts vector based on value (weights).
      // Bubble sort algorithm
      let i = 0;
      while (i < length){
        let j = 0;
        while(j < length-i-1){
          let value_j = *(Vector::borrow<u64>(&weights, j));
          let value_jp1 = *(Vector::borrow<u64>(&weights, j+1));
          if(value_j > value_jp1){
            Vector::swap<u64>(&mut weights, j, j+1);
            Vector::swap<address>(&mut eligible_validators, j, j+1);
          };
          j = j + 1;
        };
        i = i + 1;
      };

      // Reverse to have sorted order - high to low.
      Vector::reverse<address>(&mut eligible_validators);
      let index = n;
      while(index < length){
        Vector::pop_back<address>(&mut eligible_validators);
        index = index + 1;
      };
      return eligible_validators
    }
  }
}
