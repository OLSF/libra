
<a name="0x1_LibraBlock"></a>

# Module `0x1::LibraBlock`

This module defines a struct storing the metadata of the block and new block events.


-  [Resource `BlockMetadata`](#0x1_LibraBlock_BlockMetadata)
-  [Struct `NewBlockEvent`](#0x1_LibraBlock_NewBlockEvent)
-  [Constants](#@Constants_0)
-  [Function `initialize_block_metadata`](#0x1_LibraBlock_initialize_block_metadata)
-  [Function `is_initialized`](#0x1_LibraBlock_is_initialized)
-  [Function `block_prologue`](#0x1_LibraBlock_block_prologue)
-  [Function `get_current_block_height`](#0x1_LibraBlock_get_current_block_height)
-  [Module Specification](#@Module_Specification_1)
    -  [Initialization](#@Initialization_2)


<pre><code><b>use</b> <a href="AutoPay.md#0x1_AutoPay2">0x1::AutoPay2</a>;
<b>use</b> <a href="CoreAddresses.md#0x1_CoreAddresses">0x1::CoreAddresses</a>;
<b>use</b> <a href="Epoch.md#0x1_Epoch">0x1::Epoch</a>;
<b>use</b> <a href="Errors.md#0x1_Errors">0x1::Errors</a>;
<b>use</b> <a href="Event.md#0x1_Event">0x1::Event</a>;
<b>use</b> <a href="GAS.md#0x1_GAS">0x1::GAS</a>;
<b>use</b> <a href="LibraAccount.md#0x1_LibraAccount">0x1::LibraAccount</a>;
<b>use</b> <a href="LibraSystem.md#0x1_LibraSystem">0x1::LibraSystem</a>;
<b>use</b> <a href="LibraTimestamp.md#0x1_LibraTimestamp">0x1::LibraTimestamp</a>;
<b>use</b> <a href="Migrations.md#0x1_Migrations">0x1::Migrations</a>;
<b>use</b> <a href="Reconfigure.md#0x1_Reconfigure">0x1::Reconfigure</a>;
<b>use</b> <a href="Stats.md#0x1_Stats">0x1::Stats</a>;
</code></pre>



<a name="0x1_LibraBlock_BlockMetadata"></a>

## Resource `BlockMetadata`



<pre><code><b>resource</b> <b>struct</b> <a href="LibraBlock.md#0x1_LibraBlock_BlockMetadata">BlockMetadata</a>
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>height: u64</code>
</dt>
<dd>
 Height of the current block
</dd>
<dt>
<code>new_block_events: <a href="Event.md#0x1_Event_EventHandle">Event::EventHandle</a>&lt;<a href="LibraBlock.md#0x1_LibraBlock_NewBlockEvent">LibraBlock::NewBlockEvent</a>&gt;</code>
</dt>
<dd>
 Handle where events with the time of new blocks are emitted
</dd>
</dl>


</details>

<a name="0x1_LibraBlock_NewBlockEvent"></a>

## Struct `NewBlockEvent`



<pre><code><b>struct</b> <a href="LibraBlock.md#0x1_LibraBlock_NewBlockEvent">NewBlockEvent</a>
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>round: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>proposer: address</code>
</dt>
<dd>

</dd>
<dt>
<code>previous_block_votes: vector&lt;address&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>time_microseconds: u64</code>
</dt>
<dd>
 On-chain time during  he block at the given height
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0x1_LibraBlock_EBLOCK_METADATA"></a>

The <code><a href="LibraBlock.md#0x1_LibraBlock_BlockMetadata">BlockMetadata</a></code> resource is in an invalid state


<pre><code><b>const</b> <a href="LibraBlock.md#0x1_LibraBlock_EBLOCK_METADATA">EBLOCK_METADATA</a>: u64 = 0;
</code></pre>



<a name="0x1_LibraBlock_EVM_OR_VALIDATOR"></a>

An invalid signer was provided. Expected the signer to be the VM or a Validator.


<pre><code><b>const</b> <a href="LibraBlock.md#0x1_LibraBlock_EVM_OR_VALIDATOR">EVM_OR_VALIDATOR</a>: u64 = 1;
</code></pre>



<a name="0x1_LibraBlock_initialize_block_metadata"></a>

## Function `initialize_block_metadata`

This can only be invoked by the Association address, and only a single time.
Currently, it is invoked in the genesis transaction


<pre><code><b>public</b> <b>fun</b> <a href="LibraBlock.md#0x1_LibraBlock_initialize_block_metadata">initialize_block_metadata</a>(account: &signer)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="LibraBlock.md#0x1_LibraBlock_initialize_block_metadata">initialize_block_metadata</a>(account: &signer) {
    <a href="LibraTimestamp.md#0x1_LibraTimestamp_assert_genesis">LibraTimestamp::assert_genesis</a>();
    // Operational constraint, only callable by the Association address
    <a href="CoreAddresses.md#0x1_CoreAddresses_assert_libra_root">CoreAddresses::assert_libra_root</a>(account);

    <b>assert</b>(!<a href="LibraBlock.md#0x1_LibraBlock_is_initialized">is_initialized</a>(), <a href="Errors.md#0x1_Errors_already_published">Errors::already_published</a>(<a href="LibraBlock.md#0x1_LibraBlock_EBLOCK_METADATA">EBLOCK_METADATA</a>));
    move_to&lt;<a href="LibraBlock.md#0x1_LibraBlock_BlockMetadata">BlockMetadata</a>&gt;(
        account,
        <a href="LibraBlock.md#0x1_LibraBlock_BlockMetadata">BlockMetadata</a> {
            height: 0,
            new_block_events: <a href="Event.md#0x1_Event_new_event_handle">Event::new_event_handle</a>&lt;<a href="LibraBlock.md#0x1_LibraBlock_NewBlockEvent">Self::NewBlockEvent</a>&gt;(account),
        }
    );
}
</code></pre>



</details>

<details>
<summary>Specification</summary>



<pre><code><b>include</b> <a href="LibraTimestamp.md#0x1_LibraTimestamp_AbortsIfNotGenesis">LibraTimestamp::AbortsIfNotGenesis</a>;
<b>include</b> <a href="CoreAddresses.md#0x1_CoreAddresses_AbortsIfNotLibraRoot">CoreAddresses::AbortsIfNotLibraRoot</a>;
<b>aborts_if</b> <a href="LibraBlock.md#0x1_LibraBlock_is_initialized">is_initialized</a>() <b>with</b> <a href="Errors.md#0x1_Errors_ALREADY_PUBLISHED">Errors::ALREADY_PUBLISHED</a>;
<b>ensures</b> <a href="LibraBlock.md#0x1_LibraBlock_is_initialized">is_initialized</a>();
<b>ensures</b> <a href="LibraBlock.md#0x1_LibraBlock_get_current_block_height">get_current_block_height</a>() == 0;
</code></pre>



</details>

<a name="0x1_LibraBlock_is_initialized"></a>

## Function `is_initialized`

Helper function to determine whether this module has been initialized.


<pre><code><b>fun</b> <a href="LibraBlock.md#0x1_LibraBlock_is_initialized">is_initialized</a>(): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="LibraBlock.md#0x1_LibraBlock_is_initialized">is_initialized</a>(): bool {
    <b>exists</b>&lt;<a href="LibraBlock.md#0x1_LibraBlock_BlockMetadata">BlockMetadata</a>&gt;(<a href="CoreAddresses.md#0x1_CoreAddresses_LIBRA_ROOT_ADDRESS">CoreAddresses::LIBRA_ROOT_ADDRESS</a>())
}
</code></pre>



</details>

<a name="0x1_LibraBlock_block_prologue"></a>

## Function `block_prologue`

Set the metadata for the current block.
The runtime always runs this before executing the transactions in a block.


<pre><code><b>fun</b> <a href="LibraBlock.md#0x1_LibraBlock_block_prologue">block_prologue</a>(vm: &signer, round: u64, timestamp: u64, previous_block_votes: vector&lt;address&gt;, proposer: address)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="LibraBlock.md#0x1_LibraBlock_block_prologue">block_prologue</a>(
    vm: &signer,
    round: u64,
    timestamp: u64,
    previous_block_votes: vector&lt;address&gt;,
    proposer: address
) <b>acquires</b> <a href="LibraBlock.md#0x1_LibraBlock_BlockMetadata">BlockMetadata</a> {
    <a href="LibraTimestamp.md#0x1_LibraTimestamp_assert_operating">LibraTimestamp::assert_operating</a>();
    // Operational constraint: can only be invoked by the VM.
    <a href="CoreAddresses.md#0x1_CoreAddresses_assert_vm">CoreAddresses::assert_vm</a>(vm);
    // Authorization
    <b>assert</b>(
        proposer == <a href="CoreAddresses.md#0x1_CoreAddresses_VM_RESERVED_ADDRESS">CoreAddresses::VM_RESERVED_ADDRESS</a>() || <a href="LibraSystem.md#0x1_LibraSystem_is_validator">LibraSystem::is_validator</a>(proposer),
        <a href="Errors.md#0x1_Errors_requires_address">Errors::requires_address</a>(<a href="LibraBlock.md#0x1_LibraBlock_EVM_OR_VALIDATOR">EVM_OR_VALIDATOR</a>)
    );
    //////// 0L ////////
    // increment stats
// print(&01000);
    <a href="Stats.md#0x1_Stats_process_set_votes">Stats::process_set_votes</a>(vm, &previous_block_votes);
    <a href="Stats.md#0x1_Stats_inc_prop">Stats::inc_prop</a>(vm, *&proposer);

    <b>if</b> (<a href="AutoPay.md#0x1_AutoPay2_tick">AutoPay2::tick</a>(vm)){
        //triggers autopay at beginning of each epoch
        //tick is reset at end of previous epoch
        <a href="LibraAccount.md#0x1_LibraAccount_process_escrow">LibraAccount::process_escrow</a>&lt;<a href="GAS.md#0x1_GAS">GAS</a>&gt;(vm);
        <a href="AutoPay.md#0x1_AutoPay2_process_autopay">AutoPay2::process_autopay</a>(vm);
    };
    ///////////////////
    <b>let</b> block_metadata_ref = borrow_global_mut&lt;<a href="LibraBlock.md#0x1_LibraBlock_BlockMetadata">BlockMetadata</a>&gt;(<a href="CoreAddresses.md#0x1_CoreAddresses_LIBRA_ROOT_ADDRESS">CoreAddresses::LIBRA_ROOT_ADDRESS</a>());
    <a href="LibraTimestamp.md#0x1_LibraTimestamp_update_global_time">LibraTimestamp::update_global_time</a>(vm, proposer, timestamp);
    block_metadata_ref.height = block_metadata_ref.height + 1;
    <a href="Event.md#0x1_Event_emit_event">Event::emit_event</a>&lt;<a href="LibraBlock.md#0x1_LibraBlock_NewBlockEvent">NewBlockEvent</a>&gt;(
        &<b>mut</b> block_metadata_ref.new_block_events,
        <a href="LibraBlock.md#0x1_LibraBlock_NewBlockEvent">NewBlockEvent</a> {
            round,
            proposer,
            previous_block_votes,
            time_microseconds: timestamp,
        }
    );

     //////// 0L ////////
    // EPOCH BOUNDARY
    <b>if</b> (<a href="Epoch.md#0x1_Epoch_epoch_finished">Epoch::epoch_finished</a>()) {
// print(&03000);
      // Run migrations
      <a href="Migrations.md#0x1_Migrations_init">Migrations::init</a>(vm);

      // TODO: We don't need <b>to</b> pass block height <b>to</b> ReconfigureOL. It should <b>use</b> the <a href="LibraBlock.md#0x1_LibraBlock_BlockMetadata">BlockMetadata</a>. But there's a circular reference there when we try.
      <a href="Reconfigure.md#0x1_Reconfigure_reconfigure">Reconfigure::reconfigure</a>(vm, <a href="LibraBlock.md#0x1_LibraBlock_get_current_block_height">get_current_block_height</a>());
    };

}
</code></pre>



</details>

<details>
<summary>Specification</summary>



<pre><code><b>include</b> <a href="LibraTimestamp.md#0x1_LibraTimestamp_AbortsIfNotOperating">LibraTimestamp::AbortsIfNotOperating</a>;
<b>include</b> <a href="CoreAddresses.md#0x1_CoreAddresses_AbortsIfNotVM">CoreAddresses::AbortsIfNotVM</a>{account: vm};
<b>aborts_if</b> proposer != <a href="CoreAddresses.md#0x1_CoreAddresses_VM_RESERVED_ADDRESS">CoreAddresses::VM_RESERVED_ADDRESS</a>() && !<a href="LibraSystem.md#0x1_LibraSystem_spec_is_validator">LibraSystem::spec_is_validator</a>(proposer)
    <b>with</b> <a href="Errors.md#0x1_Errors_REQUIRES_ADDRESS">Errors::REQUIRES_ADDRESS</a>;
<b>ensures</b> <a href="LibraTimestamp.md#0x1_LibraTimestamp_spec_now_microseconds">LibraTimestamp::spec_now_microseconds</a>() == timestamp;
<b>ensures</b> <a href="LibraBlock.md#0x1_LibraBlock_get_current_block_height">get_current_block_height</a>() == <b>old</b>(<a href="LibraBlock.md#0x1_LibraBlock_get_current_block_height">get_current_block_height</a>()) + 1;
</code></pre>


The below counter overflow is assumed to be excluded from verification of callers.


<pre><code><b>aborts_if</b> [<b>assume</b>] <a href="LibraBlock.md#0x1_LibraBlock_get_current_block_height">get_current_block_height</a>() + 1 &gt; MAX_U64 <b>with</b> EXECUTION_FAILURE;
</code></pre>



</details>

<a name="0x1_LibraBlock_get_current_block_height"></a>

## Function `get_current_block_height`

Get the current block height


<pre><code><b>public</b> <b>fun</b> <a href="LibraBlock.md#0x1_LibraBlock_get_current_block_height">get_current_block_height</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="LibraBlock.md#0x1_LibraBlock_get_current_block_height">get_current_block_height</a>(): u64 <b>acquires</b> <a href="LibraBlock.md#0x1_LibraBlock_BlockMetadata">BlockMetadata</a> {
    <b>assert</b>(<a href="LibraBlock.md#0x1_LibraBlock_is_initialized">is_initialized</a>(), <a href="Errors.md#0x1_Errors_not_published">Errors::not_published</a>(<a href="LibraBlock.md#0x1_LibraBlock_EBLOCK_METADATA">EBLOCK_METADATA</a>));
    borrow_global&lt;<a href="LibraBlock.md#0x1_LibraBlock_BlockMetadata">BlockMetadata</a>&gt;(<a href="CoreAddresses.md#0x1_CoreAddresses_LIBRA_ROOT_ADDRESS">CoreAddresses::LIBRA_ROOT_ADDRESS</a>()).height
}
</code></pre>



</details>

<a name="@Module_Specification_1"></a>

## Module Specification



<a name="@Initialization_2"></a>

### Initialization



<pre><code><b>invariant</b> [<b>global</b>] <a href="LibraTimestamp.md#0x1_LibraTimestamp_is_operating">LibraTimestamp::is_operating</a>() ==&gt; <a href="LibraBlock.md#0x1_LibraBlock_is_initialized">is_initialized</a>();
</code></pre>


[//]: # ("File containing references which can be used from documentation")
[ACCESS_CONTROL]: https://github.com/libra/lip/blob/master/lips/lip-2.md
[ROLE]: https://github.com/libra/lip/blob/master/lips/lip-2.md#roles
[PERMISSION]: https://github.com/libra/lip/blob/master/lips/lip-2.md#permissions
