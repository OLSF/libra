
<a name="0x1_Reconfigure"></a>

# Module `0x1::Reconfigure`



-  [Function `reconfigure`](#0x1_Reconfigure_reconfigure)
-  [Function `update_validator_withdrawal_limit`](#0x1_Reconfigure_update_validator_withdrawal_limit)


<pre><code><b>use</b> <a href="AccountLimits.md#0x1_AccountLimits">0x1::AccountLimits</a>;
<b>use</b> <a href="AutoPay.md#0x1_AutoPay2">0x1::AutoPay2</a>;
<b>use</b> <a href="Burn.md#0x1_Burn">0x1::Burn</a>;
<b>use</b> <a href="CoreAddresses.md#0x1_CoreAddresses">0x1::CoreAddresses</a>;
<b>use</b> <a href="Epoch.md#0x1_Epoch">0x1::Epoch</a>;
<b>use</b> <a href="Errors.md#0x1_Errors">0x1::Errors</a>;
<b>use</b> <a href="FixedPoint32.md#0x1_FixedPoint32">0x1::FixedPoint32</a>;
<b>use</b> <a href="FullnodeState.md#0x1_FullnodeState">0x1::FullnodeState</a>;
<b>use</b> <a href="GAS.md#0x1_GAS">0x1::GAS</a>;
<b>use</b> <a href="Globals.md#0x1_Globals">0x1::Globals</a>;
<b>use</b> <a href="LibraAccount.md#0x1_LibraAccount">0x1::LibraAccount</a>;
<b>use</b> <a href="LibraConfig.md#0x1_LibraConfig">0x1::LibraConfig</a>;
<b>use</b> <a href="LibraSystem.md#0x1_LibraSystem">0x1::LibraSystem</a>;
<b>use</b> <a href="MinerState.md#0x1_MinerState">0x1::MinerState</a>;
<b>use</b> <a href="NodeWeight.md#0x1_NodeWeight">0x1::NodeWeight</a>;
<b>use</b> <a href="Signer.md#0x1_Signer">0x1::Signer</a>;
<b>use</b> <a href="Stats.md#0x1_Stats">0x1::Stats</a>;
<b>use</b> <a href="Subsidy.md#0x1_Subsidy">0x1::Subsidy</a>;
<b>use</b> <a href="ValidatorUniverse.md#0x1_ValidatorUniverse">0x1::ValidatorUniverse</a>;
<b>use</b> <a href="Vector.md#0x1_Vector">0x1::Vector</a>;
</code></pre>



<a name="0x1_Reconfigure_reconfigure"></a>

## Function `reconfigure`



<pre><code><b>public</b> <b>fun</b> <a href="Reconfigure.md#0x1_Reconfigure_reconfigure">reconfigure</a>(vm: &signer, height_now: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="Reconfigure.md#0x1_Reconfigure_reconfigure">reconfigure</a>(vm: &signer, height_now: u64) {
    <b>assert</b>(<a href="Signer.md#0x1_Signer_address_of">Signer::address_of</a>(vm) == <a href="CoreAddresses.md#0x1_CoreAddresses_LIBRA_ROOT_ADDRESS">CoreAddresses::LIBRA_ROOT_ADDRESS</a>(), <a href="Errors.md#0x1_Errors_requires_role">Errors::requires_role</a>(180001));

    // Fullnode subsidy
    // <b>loop</b> through validators and pay full node subsidies.
    // Should happen before transactionfees get distributed.
    // There may be new validators which have not mined yet.
// print(&03100);

    <b>let</b> miners = <a href="MinerState.md#0x1_MinerState_get_miner_list">MinerState::get_miner_list</a>();

    // Migration for miner list.
    <b>if</b> (<a href="Vector.md#0x1_Vector_length">Vector::length</a>(&miners) == 0) { miners = <a href="ValidatorUniverse.md#0x1_ValidatorUniverse_get_eligible_validators">ValidatorUniverse::get_eligible_validators</a>(vm) };

    <b>let</b> global_proofs_count = 0;
    <b>let</b> k = 0;
// print(&03200);

    // Distribute mining subsidy <b>to</b> fullnodes
    <b>while</b> (k &lt; <a href="Vector.md#0x1_Vector_length">Vector::length</a>(&miners)) {
        <b>let</b> addr = *<a href="Vector.md#0x1_Vector_borrow">Vector::borrow</a>(&miners, k);
// print(&03210);

        <b>if</b> (!<a href="FullnodeState.md#0x1_FullnodeState_is_init">FullnodeState::is_init</a>(addr)) <b>continue</b>; // fail-safe

        <b>let</b> count = <a href="MinerState.md#0x1_MinerState_get_count_in_epoch">MinerState::get_count_in_epoch</a>(addr);

        global_proofs_count = global_proofs_count + count;

        <b>let</b> value: u64;
        // check <b>if</b> is in onboarding state (or stuck)
// print(&03220);

        <b>if</b> (<a href="FullnodeState.md#0x1_FullnodeState_is_onboarding">FullnodeState::is_onboarding</a>(addr)) {
// print(&03221);

          // TODO: onboarding subsidy is not necessary <b>with</b> onboarding transfer.
            value = <a href="Subsidy.md#0x1_Subsidy_distribute_onboarding_subsidy">Subsidy::distribute_onboarding_subsidy</a>(vm, addr);
        } <b>else</b> {
            // steady state
            value = <a href="Subsidy.md#0x1_Subsidy_distribute_fullnode_subsidy">Subsidy::distribute_fullnode_subsidy</a>(vm, addr, count);
        };

// print(&03230);
        <a href="FullnodeState.md#0x1_FullnodeState_inc_payment_count">FullnodeState::inc_payment_count</a>(vm, addr, count);
        <a href="FullnodeState.md#0x1_FullnodeState_inc_payment_value">FullnodeState::inc_payment_value</a>(vm, addr, value);
        <a href="FullnodeState.md#0x1_FullnodeState_reconfig">FullnodeState::reconfig</a>(vm, addr, count);

        k = k + 1;
    };
    // Process outgoing validators:
    // Distribute Transaction fees and subsidy payments <b>to</b> all outgoing validators
    <b>let</b> height_start = <a href="Epoch.md#0x1_Epoch_get_timer_height_start">Epoch::get_timer_height_start</a>(vm);

// print(&03240);

    <b>let</b> (outgoing_set, fee_ratio) = <a href="LibraSystem.md#0x1_LibraSystem_get_fee_ratio">LibraSystem::get_fee_ratio</a>(vm, height_start, height_now);
    <b>if</b> (<a href="Vector.md#0x1_Vector_length">Vector::length</a>&lt;address&gt;(&outgoing_set) &gt; 0) {
        <b>let</b> subsidy_units = <a href="Subsidy.md#0x1_Subsidy_calculate_subsidy">Subsidy::calculate_subsidy</a>(vm, height_start, height_now);
// print(&03241);

        <b>if</b> (subsidy_units &gt; 0) {
            <a href="Subsidy.md#0x1_Subsidy_process_subsidy">Subsidy::process_subsidy</a>(vm, subsidy_units, &outgoing_set, &fee_ratio);
        };
// print(&03241);

        <a href="Subsidy.md#0x1_Subsidy_process_fees">Subsidy::process_fees</a>(vm, &outgoing_set, &fee_ratio);
    };

    // Propose upcoming validator set:
    // Step 1: Sort Top N eligible validators
    // Step 2: Jail non-performing validators
    // Step 3: Reset counters
    // Step 4: Bulk <b>update</b> validator set (reconfig)

    // TODO: Temporary until JailedBit is fully migrated.
    // 1. remove jailed set from validator universe

    // save all the eligible list, before the jailing removes them.
    <b>let</b> proposed_set = <a href="Vector.md#0x1_Vector_empty">Vector::empty</a>();

    <b>let</b> top_accounts = <a href="NodeWeight.md#0x1_NodeWeight_top_n_accounts">NodeWeight::top_n_accounts</a>(vm, <a href="Globals.md#0x1_Globals_get_max_validator_per_epoch">Globals::get_max_validator_per_epoch</a>());

    <b>let</b> jailed_set = <a href="LibraSystem.md#0x1_LibraSystem_get_jailed_set">LibraSystem::get_jailed_set</a>(vm, height_start, height_now);
// print(&03250);
    <a href="Burn.md#0x1_Burn_reset_ratios">Burn::reset_ratios</a>(vm);
    // <b>let</b> incoming_count = <a href="Vector.md#0x1_Vector_length">Vector::length</a>&lt;address&gt;(&top_accounts) - <a href="Vector.md#0x1_Vector_length">Vector::length</a>&lt;address&gt;(&jailed_set);
    // <b>let</b> burn_value = <a href="Subsidy.md#0x1_Subsidy_subsidy_curve">Subsidy::subsidy_curve</a>(
    //   <a href="Globals.md#0x1_Globals_get_subsidy_ceiling_gas">Globals::get_subsidy_ceiling_gas</a>(),
    //   incoming_count,
    //   <a href="Globals.md#0x1_Globals_get_max_node_density">Globals::get_max_node_density</a>()
    // )/4;
    <b>let</b> burn_value = 1000000; // TODO: switch <b>to</b> a variable cost, <b>as</b> above.

// print(&burn_value);


    <b>let</b> i = 0;
    <b>while</b> (i &lt; <a href="Vector.md#0x1_Vector_length">Vector::length</a>&lt;address&gt;(&top_accounts)) {
// print(&03251);

        <b>let</b> addr = *<a href="Vector.md#0x1_Vector_borrow">Vector::borrow</a>(&top_accounts, i);
// print(&addr);
        <b>let</b> mined_last_epoch = <a href="MinerState.md#0x1_MinerState_node_above_thresh">MinerState::node_above_thresh</a>(vm, addr);

// print(&mined_last_epoch);

        // TODO: temporary until jail-refactor merge.
        <b>if</b> ((!<a href="Vector.md#0x1_Vector_contains">Vector::contains</a>(&jailed_set, &addr)) && mined_last_epoch) {
// print(&03252);
            // execute the burn according <b>to</b> preferences
            <a href="Burn.md#0x1_Burn_epoch_start_burn">Burn::epoch_start_burn</a>(vm, addr, burn_value);
            <a href="Vector.md#0x1_Vector_push_back">Vector::push_back</a>(&<b>mut</b> proposed_set, addr);
        };
        i = i+ 1;
    };

    // 2. get top accounts.
    // TODO: This is temporary. Top N is after jailed have been removed
    // <b>let</b> proposed_set = <a href="NodeWeight.md#0x1_NodeWeight_top_n_accounts">NodeWeight::top_n_accounts</a>(vm, <a href="Globals.md#0x1_Globals_get_max_validator_per_epoch">Globals::get_max_validator_per_epoch</a>());
    // <b>let</b> proposed_set = top_accounts;

// print(&03260);

    // If the cardinality of validator_set in the next epoch is less than 4, we keep the same validator set.
    <b>if</b> (<a href="Vector.md#0x1_Vector_length">Vector::length</a>&lt;address&gt;(&proposed_set)&lt;= 3) proposed_set = *&top_accounts;
    // Usually an issue in staging network for QA only.
    // This is very rare and theoretically impossible for network <b>with</b> at least 6 nodes and 6 rounds. If we reach an epoch boundary <b>with</b> at least 6 rounds, we would have at least 2/3rd of the validator set <b>with</b> at least 66% liveliness.
// print(&03270);

    // Update all validators <b>with</b> account limits
    // After <a href="Epoch.md#0x1_Epoch">Epoch</a> 1000.
    <b>if</b> (<a href="LibraConfig.md#0x1_LibraConfig_check_transfer_enabled">LibraConfig::check_transfer_enabled</a>()) {
        <a href="Reconfigure.md#0x1_Reconfigure_update_validator_withdrawal_limit">update_validator_withdrawal_limit</a>(vm);
    };
    // needs <b>to</b> be set before the auctioneer runs in <a href="Subsidy.md#0x1_Subsidy_fullnode_reconfig">Subsidy::fullnode_reconfig</a>
    <a href="Subsidy.md#0x1_Subsidy_set_global_count">Subsidy::set_global_count</a>(vm, global_proofs_count);
// print(&03280);

    //Reset Counters
    <a href="Stats.md#0x1_Stats_reconfig">Stats::reconfig</a>(vm, &proposed_set);
// print(&03290);

    // Migrate <a href="MinerState.md#0x1_MinerState">MinerState</a> list from elegible: in case there is no minerlist <b>struct</b>, <b>use</b> eligible for migrate_eligible_validators
    <b>let</b> eligible = <a href="ValidatorUniverse.md#0x1_ValidatorUniverse_get_eligible_validators">ValidatorUniverse::get_eligible_validators</a>(vm);
    <a href="MinerState.md#0x1_MinerState_reconfig">MinerState::reconfig</a>(vm, &eligible);
// print(&032100);

    // <a href="Reconfigure.md#0x1_Reconfigure">Reconfigure</a> the network
    <a href="LibraSystem.md#0x1_LibraSystem_bulk_update_validators">LibraSystem::bulk_update_validators</a>(vm, proposed_set);
// print(&032110);

    // reset clocks
    <a href="Subsidy.md#0x1_Subsidy_fullnode_reconfig">Subsidy::fullnode_reconfig</a>(vm);
//  print(&032120);

    // process community wallets
    <a href="LibraAccount.md#0x1_LibraAccount_process_community_wallets">LibraAccount::process_community_wallets</a>(vm, <a href="LibraConfig.md#0x1_LibraConfig_get_current_epoch">LibraConfig::get_current_epoch</a>());

    <a href="AutoPay.md#0x1_AutoPay2_reconfig_reset_tick">AutoPay2::reconfig_reset_tick</a>(vm);
//  print(&032130);
    <a href="Epoch.md#0x1_Epoch_reset_timer">Epoch::reset_timer</a>(vm, height_now);
}
</code></pre>



</details>

<a name="0x1_Reconfigure_update_validator_withdrawal_limit"></a>

## Function `update_validator_withdrawal_limit`

OL function to update withdrawal limits in all validator accounts


<pre><code><b>fun</b> <a href="Reconfigure.md#0x1_Reconfigure_update_validator_withdrawal_limit">update_validator_withdrawal_limit</a>(vm: &signer)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="Reconfigure.md#0x1_Reconfigure_update_validator_withdrawal_limit">update_validator_withdrawal_limit</a>(vm: &signer) {
    <b>let</b> validator_set = <a href="LibraSystem.md#0x1_LibraSystem_get_val_set_addr">LibraSystem::get_val_set_addr</a>();
    <b>let</b> k = 0;
    <b>while</b>(k &lt; <a href="Vector.md#0x1_Vector_length">Vector::length</a>(&validator_set)){
        <b>let</b> addr = *<a href="Vector.md#0x1_Vector_borrow">Vector::borrow</a>&lt;address&gt;(&validator_set, k);

        // Check <b>if</b> limits definition is published
        <b>if</b>(<a href="AccountLimits.md#0x1_AccountLimits_has_limits_published">AccountLimits::has_limits_published</a>&lt;<a href="GAS.md#0x1_GAS">GAS</a>&gt;(addr)) {
            <a href="AccountLimits.md#0x1_AccountLimits_update_limits_definition">AccountLimits::update_limits_definition</a>&lt;<a href="GAS.md#0x1_GAS">GAS</a>&gt;(vm, addr, 0, <a href="LibraConfig.md#0x1_LibraConfig_get_epoch_transfer_limit">LibraConfig::get_epoch_transfer_limit</a>(), 0, 0);
        };

        k = k + 1;
    };
}
</code></pre>



</details>


[//]: # ("File containing references which can be used from documentation")
[ACCESS_CONTROL]: https://github.com/libra/lip/blob/master/lips/lip-2.md
[ROLE]: https://github.com/libra/lip/blob/master/lips/lip-2.md#roles
[PERMISSION]: https://github.com/libra/lip/blob/master/lips/lip-2.md#permissions
