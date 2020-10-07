///////////////////////////////////////////////////////////////////////////
// 0L Module
// Epoch Prologue
///////////////////////////////////////////////////////////////////////////
// The prologue for transitioning to next epoch after every n blocks.
// File Prefix for errors: 1801
///////////////////////////////////////////////////////////////////////////

address 0x0 {
    module Reconfigure {
        use 0x0::Signer;
        use 0x0::Transaction;
        use 0x0::Subsidy;
        use 0x0::NodeWeight;
        use 0x0::LibraSystem;
        use 0x0::GAS;
        use 0x0::TransactionFee;
        use 0x0::MinerState;
        use 0x0::Globals;
        use 0x0::Vector;
        use 0x0::Stats;
        use 0x0::Debug::print;


        // This function is called by block-prologue once after n blocks.
        // Function code: 01. Prefix: 180101
        public fun reconfigure(vm: &signer) {
            Transaction::assert(Signer::address_of(vm) == 0x0, 180101014010);
            print(&0x011111111111);
            // Process outgoing validators
            // Step 1: Update MinerState for all validators
            // Step 2: Distribute Subsidy payments
            // Step 3: Distribute transaction fees to all outgoing validators
            process_outgoing_validators(vm);
            print(&0x022222222222222);


            // Recommend upcoming validator set
            // Step 1: Sort Top N Elegible validators
            // Step 2: Bulk update validator set (reconfig)
            // Step 3: Update Stats counter
            prepare_upcoming_validator_set(vm);
            print(&0x03333333333333333);

        }

        // Function code: 02. Prefix: 180102
        fun process_outgoing_validators(vm_sig: &signer) {
            // Get outgoing validator and sum of all validator weights

            // Step 1: End redeem for all validators
            MinerState::reconfig(vm_sig);
            print(&0x044444444444444444444);

            //TODO: do we need skip first epoch?
            let subsidy_units = Subsidy::calculate_Subsidy();
            print(&0x0555555555555555555555);

            
            Subsidy::process_subsidy(vm_sig, subsidy_units);
            print(&0x0666666666666666666666);

            // Step 3: Distribute transaction fees here before updating validators
            TransactionFee::distribute_transaction_fees<GAS::T>();
            print(&0x0777777777777777777777);

            // Step 4: Getting current epoch value. Burning for all epochs except for the first one.
            // Stats::reconfig(&LibraSystem::get_val_set_addr());

        }

        // Function code: 03. Prefix: 180103
        fun prepare_upcoming_validator_set(vm: &signer) {
            // Step 1: Calls NodeWeights on validatorset to select top N accounts.
            let top_accounts = NodeWeight::top_n_accounts(
                vm, Globals::get_max_validator_per_epoch());
            print(&top_accounts);
            let jailed_set = LibraSystem::get_jailed_set();
            print(&jailed_set);
            let proposed_set = Vector::empty();
            let i = 0;
            while (i < Vector::length(&top_accounts)) {
                    // public fun contains<Element>(v: &vector<Element>, e: &Element): bool {
                let addr = *Vector::borrow(&top_accounts, i);

                if (!Vector::contains(&jailed_set, &addr)){
                    Vector::push_back(&mut proposed_set, addr);
                };
                i = i+ 1;
            };

            let length = Vector::length<address>(&proposed_set);
            print(&0x08888888888888888888);


            // use previous set if no one qualifies.
            if(length <= 4) proposed_set = LibraSystem::get_val_set_addr();

            print(&proposed_set);
            Stats::reconfig(&proposed_set);
            
            LibraSystem::bulk_update_validators(vm, proposed_set);
            print(&0x09999999999999999999);

            // If the cardinality of validator_set in the next epoch is less than 4, we skip the epoch tranisition. 
            // Refer Theorem: If we reach an epoch boundary with at least 6 rounds, we would have at least 2/3rd of the validator set with at least 66% liveliness. 
            // This is very rare and theoretically impossible for network with at least 6 nodes and 6 rounds. 


        }
  }
}