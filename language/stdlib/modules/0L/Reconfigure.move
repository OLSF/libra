///////////////////////////////////////////////////////////////////////////
// 0L Module
// Epoch Prologue
///////////////////////////////////////////////////////////////////////////
// The prologue for transitioning to next epoch after every n blocks.
// File Prefix for errors: 1800
///////////////////////////////////////////////////////////////////////////

address 0x1 {
module Reconfigure {
    use 0x1::Signer;
    use 0x1::CoreAddresses;
    use 0x1::Errors;
    use 0x1::Subsidy;
    use 0x1::NodeWeight;
    use 0x1::LibraSystem;
    use 0x1::MinerState;
    use 0x1::Globals;
    use 0x1::Vector;
    use 0x1::Stats;
    use 0x1::ValidatorUniverse;
    use 0x1::AutoPay2;
    use 0x1::Epoch;
    use 0x1::FullnodeState;
    use 0x1::AccountLimits;
    use 0x1::GAS::GAS;
    use 0x1::LibraConfig;
    use 0x1::Burn;
    use 0x1::LibraAccount;
    // This function is called by block-prologue once after n blocks.
    // Function code: 01. Prefix: 180001
    public fun reconfigure(vm: &signer, height_now: u64) {
        assert(Signer::address_of(vm) == CoreAddresses::LIBRA_ROOT_ADDRESS(), Errors::requires_role(180001));

        // Fullnode subsidy
        // loop through validators and pay full node subsidies.
        // Should happen before transactionfees get distributed.
        // There may be new validators which have not mined yet.
// print(&03100);

        let miners = MinerState::get_miner_list();
        
        // Migration for miner list.
        if (Vector::length(&miners) == 0) { miners = ValidatorUniverse::get_eligible_validators(vm) };

        let global_proofs_count = 0;
        let k = 0;
// print(&03200);

        // Distribute mining subsidy to fullnodes
        while (k < Vector::length(&miners)) {
            let addr = *Vector::borrow(&miners, k);
// print(&03210);
          
            if (!FullnodeState::is_init(addr)) continue; // fail-safe

            let count = MinerState::get_count_in_epoch(addr);
            
            global_proofs_count = global_proofs_count + count;
            
            let value: u64;
            // check if is in onboarding state (or stuck)
// print(&03220);

            if (FullnodeState::is_onboarding(addr)) {
// print(&03221);

              // TODO: onboarding subsidy is not necessary with onboarding transfer.
                value = Subsidy::distribute_onboarding_subsidy(vm, addr);
            } else {
                // steady state
                value = Subsidy::distribute_fullnode_subsidy(vm, addr, count);
            };

// print(&03230);
            FullnodeState::inc_payment_count(vm, addr, count);
            FullnodeState::inc_payment_value(vm, addr, value);
            FullnodeState::reconfig(vm, addr, count);

            k = k + 1;
        };
        // Process outgoing validators:
        // Distribute Transaction fees and subsidy payments to all outgoing validators
        let height_start = Epoch::get_timer_height_start(vm);

// print(&03240);

        let (outgoing_set, fee_ratio) = LibraSystem::get_fee_ratio(vm, height_start, height_now);
        if (Vector::length<address>(&outgoing_set) > 0) {
            let subsidy_units = Subsidy::calculate_subsidy(vm, height_start, height_now);
// print(&03241);

            if (subsidy_units > 0) {
                Subsidy::process_subsidy(vm, subsidy_units, &outgoing_set, &fee_ratio);
            };
// print(&03241);

            Subsidy::process_fees(vm, &outgoing_set, &fee_ratio);
        };

        // Propose upcoming validator set:
        // Step 1: Sort Top N eligible validators
        // Step 2: Jail non-performing validators
        // Step 3: Reset counters
        // Step 4: Bulk update validator set (reconfig)

        // TODO: Temporary until JailedBit is fully migrated.
        // 1. remove jailed set from validator universe
        
        // save all the eligible list, before the jailing removes them.
        let proposed_set = Vector::empty();

        let top_accounts = NodeWeight::top_n_accounts(vm, Globals::get_max_validator_per_epoch());

        let jailed_set = LibraSystem::get_jailed_set(vm, height_start, height_now);
// print(&03250);
        Burn::reset_ratios(vm);
        // let incoming_count = Vector::length<address>(&top_accounts) - Vector::length<address>(&jailed_set);
        // let burn_value = Subsidy::subsidy_curve(
        //   Globals::get_subsidy_ceiling_gas(),
        //   incoming_count,
        //   Globals::get_max_node_density()
        // )/4;
        let burn_value = 1000000; // TODO: switch to a variable cost, as above.

// print(&burn_value);


        let i = 0;
        while (i < Vector::length<address>(&top_accounts)) {
// print(&03251);

            let addr = *Vector::borrow(&top_accounts, i);
// print(&addr);
            let mined_last_epoch = MinerState::node_above_thresh(vm, addr);

// print(&mined_last_epoch);
       
            // TODO: temporary until jail-refactor merge.
            if ((!Vector::contains(&jailed_set, &addr)) && mined_last_epoch) {
// print(&03252);   
                // execute the burn according to preferences
                Burn::epoch_start_burn(vm, addr, burn_value);
                Vector::push_back(&mut proposed_set, addr);
            };
            i = i+ 1;
        };

        // 2. get top accounts.
        // TODO: This is temporary. Top N is after jailed have been removed
        // let proposed_set = NodeWeight::top_n_accounts(vm, Globals::get_max_validator_per_epoch());
        // let proposed_set = top_accounts;

// print(&03260);

        // If the cardinality of validator_set in the next epoch is less than 4, we keep the same validator set. 
        if (Vector::length<address>(&proposed_set)<= 3) proposed_set = *&top_accounts;
        // Usually an issue in staging network for QA only.
        // This is very rare and theoretically impossible for network with at least 6 nodes and 6 rounds. If we reach an epoch boundary with at least 6 rounds, we would have at least 2/3rd of the validator set with at least 66% liveliness. 
// print(&03270);
        
        // Update all validators with account limits
        // After Epoch 1000. 
        if (LibraConfig::check_transfer_enabled()) {
            update_validator_withdrawal_limit(vm);
        };
        // needs to be set before the auctioneer runs in Subsidy::fullnode_reconfig
        Subsidy::set_global_count(vm, global_proofs_count);
// print(&03280);

        //Reset Counters
        Stats::reconfig(vm, &proposed_set);
// print(&03290);

        // Migrate MinerState list from elegible: in case there is no minerlist struct, use eligible for migrate_eligible_validators
        let eligible = ValidatorUniverse::get_eligible_validators(vm);
        MinerState::reconfig(vm, &eligible);
// print(&032100);

        // Reconfigure the network
        LibraSystem::bulk_update_validators(vm, proposed_set);
// print(&032110);

        // reset clocks
        Subsidy::fullnode_reconfig(vm);
//  print(&032120);

        // process community wallets
        LibraAccount::process_community_wallets(vm, LibraConfig::get_current_epoch());

        AutoPay2::reconfig_reset_tick(vm);
//  print(&032130);
        Epoch::reset_timer(vm, height_now);
    }

    /// OL function to update withdrawal limits in all validator accounts
    fun update_validator_withdrawal_limit(vm: &signer) {
        let validator_set = LibraSystem::get_val_set_addr();
        let k = 0;
        while(k < Vector::length(&validator_set)){
            let addr = *Vector::borrow<address>(&validator_set, k);

            // Check if limits definition is published
            if(AccountLimits::has_limits_published<GAS>(addr)) {
                AccountLimits::update_limits_definition<GAS>(vm, addr, 0, LibraConfig::get_epoch_transfer_limit(), 0, 0);
            };  
            
            k = k + 1;
        };
    }

}
}