
<a name="0x1_Wallet"></a>

# Module `0x1::Wallet`



-  [Resource `CommunityWallets`](#0x1_Wallet_CommunityWallets)
-  [Resource `CommunityTransfers`](#0x1_Wallet_CommunityTransfers)
-  [Struct `TimedTransfer`](#0x1_Wallet_TimedTransfer)
-  [Struct `Veto`](#0x1_Wallet_Veto)
-  [Resource `CommunityFreeze`](#0x1_Wallet_CommunityFreeze)
-  [Resource `SlowWallet`](#0x1_Wallet_SlowWallet)
-  [Constants](#@Constants_0)
-  [Function `init`](#0x1_Wallet_init)
-  [Function `set_comm`](#0x1_Wallet_set_comm)
-  [Function `vm_set_comm`](#0x1_Wallet_vm_set_comm)
-  [Function `vm_remove_comm`](#0x1_Wallet_vm_remove_comm)
-  [Function `new_timed_transfer`](#0x1_Wallet_new_timed_transfer)
-  [Function `find`](#0x1_Wallet_find)
-  [Function `veto`](#0x1_Wallet_veto)
-  [Function `reject`](#0x1_Wallet_reject)
-  [Function `tally_veto`](#0x1_Wallet_tally_veto)
-  [Function `calculate_proportional_voting_threshold`](#0x1_Wallet_calculate_proportional_voting_threshold)
-  [Function `list_tx_by_epoch`](#0x1_Wallet_list_tx_by_epoch)
-  [Function `maybe_reset_rejection_counter`](#0x1_Wallet_maybe_reset_rejection_counter)
-  [Function `maybe_freeze`](#0x1_Wallet_maybe_freeze)
-  [Function `vote_to_unfreeze`](#0x1_Wallet_vote_to_unfreeze)
-  [Function `tally_unfreeze`](#0x1_Wallet_tally_unfreeze)
-  [Function `get_tx_args`](#0x1_Wallet_get_tx_args)
-  [Function `transfer_is_proposed`](#0x1_Wallet_transfer_is_proposed)
-  [Function `transfer_is_rejected`](#0x1_Wallet_transfer_is_rejected)
-  [Function `get_comm_list`](#0x1_Wallet_get_comm_list)
-  [Function `is_comm`](#0x1_Wallet_is_comm)
-  [Function `is_frozen`](#0x1_Wallet_is_frozen)
-  [Function `set_slow`](#0x1_Wallet_set_slow)
-  [Function `is_slow`](#0x1_Wallet_is_slow)


<pre><code><b>use</b> <a href="CoreAddresses.md#0x1_CoreAddresses">0x1::CoreAddresses</a>;
<b>use</b> <a href="Debug.md#0x1_Debug">0x1::Debug</a>;
<b>use</b> <a href="Errors.md#0x1_Errors">0x1::Errors</a>;
<b>use</b> <a href="LibraConfig.md#0x1_LibraConfig">0x1::LibraConfig</a>;
<b>use</b> <a href="LibraSystem.md#0x1_LibraSystem">0x1::LibraSystem</a>;
<b>use</b> <a href="NodeWeight.md#0x1_NodeWeight">0x1::NodeWeight</a>;
<b>use</b> <a href="Option.md#0x1_Option">0x1::Option</a>;
<b>use</b> <a href="Signer.md#0x1_Signer">0x1::Signer</a>;
<b>use</b> <a href="Vector.md#0x1_Vector">0x1::Vector</a>;
</code></pre>



<a name="0x1_Wallet_CommunityWallets"></a>

## Resource `CommunityWallets`



<pre><code><b>resource</b> <b>struct</b> <a href="Wallet.md#0x1_Wallet_CommunityWallets">CommunityWallets</a>
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>list: vector&lt;address&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x1_Wallet_CommunityTransfers"></a>

## Resource `CommunityTransfers`



<pre><code><b>resource</b> <b>struct</b> <a href="Wallet.md#0x1_Wallet_CommunityTransfers">CommunityTransfers</a>
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>proposed: vector&lt;<a href="Wallet.md#0x1_Wallet_TimedTransfer">Wallet::TimedTransfer</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>approved: vector&lt;<a href="Wallet.md#0x1_Wallet_TimedTransfer">Wallet::TimedTransfer</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>rejected: vector&lt;<a href="Wallet.md#0x1_Wallet_TimedTransfer">Wallet::TimedTransfer</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>max_uid: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x1_Wallet_TimedTransfer"></a>

## Struct `TimedTransfer`



<pre><code><b>struct</b> <a href="Wallet.md#0x1_Wallet_TimedTransfer">TimedTransfer</a>
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>uid: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>expire_epoch: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>payer: address</code>
</dt>
<dd>

</dd>
<dt>
<code>payee: address</code>
</dt>
<dd>

</dd>
<dt>
<code>value: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>description: vector&lt;u8&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>veto: <a href="Wallet.md#0x1_Wallet_Veto">Wallet::Veto</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x1_Wallet_Veto"></a>

## Struct `Veto`



<pre><code><b>struct</b> <a href="Wallet.md#0x1_Wallet_Veto">Veto</a>
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>list: vector&lt;address&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>count: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>threshold: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x1_Wallet_CommunityFreeze"></a>

## Resource `CommunityFreeze`



<pre><code><b>resource</b> <b>struct</b> <a href="Wallet.md#0x1_Wallet_CommunityFreeze">CommunityFreeze</a>
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>is_frozen: bool</code>
</dt>
<dd>

</dd>
<dt>
<code>consecutive_rejections: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>unfreeze_votes: vector&lt;address&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x1_Wallet_SlowWallet"></a>

## Resource `SlowWallet`



<pre><code><b>resource</b> <b>struct</b> <a href="Wallet.md#0x1_Wallet_SlowWallet">SlowWallet</a>
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>is_slow: bool</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0x1_Wallet_ERR_PREFIX"></a>



<pre><code><b>const</b> <a href="Wallet.md#0x1_Wallet_ERR_PREFIX">ERR_PREFIX</a>: u64 = 23;
</code></pre>



<a name="0x1_Wallet_init"></a>

## Function `init`



<pre><code><b>public</b> <b>fun</b> <a href="Wallet.md#0x1_Wallet_init">init</a>(vm: &signer)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="Wallet.md#0x1_Wallet_init">init</a>(vm: &signer) {
    <a href="CoreAddresses.md#0x1_CoreAddresses_assert_libra_root">CoreAddresses::assert_libra_root</a>(vm);

    <b>if</b> ((!<b>exists</b>&lt;<a href="Wallet.md#0x1_Wallet_CommunityTransfers">CommunityTransfers</a>&gt;(0x0))) {
      move_to&lt;<a href="Wallet.md#0x1_Wallet_CommunityTransfers">CommunityTransfers</a>&gt;(vm, <a href="Wallet.md#0x1_Wallet_CommunityTransfers">CommunityTransfers</a>{
        proposed: <a href="Vector.md#0x1_Vector_empty">Vector::empty</a>&lt;<a href="Wallet.md#0x1_Wallet_TimedTransfer">TimedTransfer</a>&gt;(),
        approved: <a href="Vector.md#0x1_Vector_empty">Vector::empty</a>&lt;<a href="Wallet.md#0x1_Wallet_TimedTransfer">TimedTransfer</a>&gt;(),
        rejected: <a href="Vector.md#0x1_Vector_empty">Vector::empty</a>&lt;<a href="Wallet.md#0x1_Wallet_TimedTransfer">TimedTransfer</a>&gt;(),
        max_uid: 0,
      })
    };

  <b>if</b> (!<b>exists</b>&lt;<a href="Wallet.md#0x1_Wallet_CommunityWallets">CommunityWallets</a>&gt;(0x0)) {
    move_to&lt;<a href="Wallet.md#0x1_Wallet_CommunityWallets">CommunityWallets</a>&gt;(vm, <a href="Wallet.md#0x1_Wallet_CommunityWallets">CommunityWallets</a> {
      list: <a href="Vector.md#0x1_Vector_empty">Vector::empty</a>&lt;address&gt;()
    });
  }
}
</code></pre>



</details>

<a name="0x1_Wallet_set_comm"></a>

## Function `set_comm`



<pre><code><b>public</b> <b>fun</b> <a href="Wallet.md#0x1_Wallet_set_comm">set_comm</a>(sig: &signer)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="Wallet.md#0x1_Wallet_set_comm">set_comm</a>(sig: &signer) <b>acquires</b> <a href="Wallet.md#0x1_Wallet_CommunityWallets">CommunityWallets</a> {
  <b>if</b> (<b>exists</b>&lt;<a href="Wallet.md#0x1_Wallet_CommunityWallets">CommunityWallets</a>&gt;(0x0)) {
    <b>let</b> addr = <a href="Signer.md#0x1_Signer_address_of">Signer::address_of</a>(sig);
    <b>let</b> list = <a href="Wallet.md#0x1_Wallet_get_comm_list">get_comm_list</a>();
    <b>if</b> (!<a href="Vector.md#0x1_Vector_contains">Vector::contains</a>&lt;address&gt;(&list, &addr)) {
        <b>let</b> s = borrow_global_mut&lt;<a href="Wallet.md#0x1_Wallet_CommunityWallets">CommunityWallets</a>&gt;(0x0);
        <a href="Vector.md#0x1_Vector_push_back">Vector::push_back</a>(&<b>mut</b> s.list, addr);
    };

    move_to&lt;<a href="Wallet.md#0x1_Wallet_CommunityFreeze">CommunityFreeze</a>&gt;(sig, <a href="Wallet.md#0x1_Wallet_CommunityFreeze">CommunityFreeze</a> {
      is_frozen: <b>false</b>,
      consecutive_rejections: 0,
      unfreeze_votes: <a href="Vector.md#0x1_Vector_empty">Vector::empty</a>&lt;address&gt;()
    })
  }
}
</code></pre>



</details>

<a name="0x1_Wallet_vm_set_comm"></a>

## Function `vm_set_comm`



<pre><code><b>public</b> <b>fun</b> <a href="Wallet.md#0x1_Wallet_vm_set_comm">vm_set_comm</a>(vm: &signer, addr: address)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="Wallet.md#0x1_Wallet_vm_set_comm">vm_set_comm</a>(vm: &signer, addr: address) <b>acquires</b> <a href="Wallet.md#0x1_Wallet_CommunityWallets">CommunityWallets</a> {
  <a href="CoreAddresses.md#0x1_CoreAddresses_assert_libra_root">CoreAddresses::assert_libra_root</a>(vm);
  <b>if</b> (<b>exists</b>&lt;<a href="Wallet.md#0x1_Wallet_CommunityWallets">CommunityWallets</a>&gt;(0x0)) {
    <b>let</b> list = <a href="Wallet.md#0x1_Wallet_get_comm_list">get_comm_list</a>();
    <b>if</b> (!<a href="Vector.md#0x1_Vector_contains">Vector::contains</a>&lt;address&gt;(&list, &addr)) {

      <b>let</b> s = borrow_global_mut&lt;<a href="Wallet.md#0x1_Wallet_CommunityWallets">CommunityWallets</a>&gt;(0x0);
      <a href="Vector.md#0x1_Vector_push_back">Vector::push_back</a>(&<b>mut</b> s.list, addr);
    }
  }
}
</code></pre>



</details>

<a name="0x1_Wallet_vm_remove_comm"></a>

## Function `vm_remove_comm`



<pre><code><b>public</b> <b>fun</b> <a href="Wallet.md#0x1_Wallet_vm_remove_comm">vm_remove_comm</a>(vm: &signer, addr: address)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="Wallet.md#0x1_Wallet_vm_remove_comm">vm_remove_comm</a>(vm: &signer, addr: address) <b>acquires</b> <a href="Wallet.md#0x1_Wallet_CommunityWallets">CommunityWallets</a> {
  <a href="CoreAddresses.md#0x1_CoreAddresses_assert_libra_root">CoreAddresses::assert_libra_root</a>(vm);
  <b>if</b> (<b>exists</b>&lt;<a href="Wallet.md#0x1_Wallet_CommunityWallets">CommunityWallets</a>&gt;(0x0)) {
    <b>let</b> list = <a href="Wallet.md#0x1_Wallet_get_comm_list">get_comm_list</a>();
    <b>let</b> (yes, i) = <a href="Vector.md#0x1_Vector_index_of">Vector::index_of</a>&lt;address&gt;(&list, &addr);
    <b>if</b> (yes) {
      <b>let</b> s = borrow_global_mut&lt;<a href="Wallet.md#0x1_Wallet_CommunityWallets">CommunityWallets</a>&gt;(0x0);
      <a href="Vector.md#0x1_Vector_remove">Vector::remove</a>(&<b>mut</b> s.list, i);
    }
  }
}
</code></pre>



</details>

<a name="0x1_Wallet_new_timed_transfer"></a>

## Function `new_timed_transfer`



<pre><code><b>public</b> <b>fun</b> <a href="Wallet.md#0x1_Wallet_new_timed_transfer">new_timed_transfer</a>(sender: &signer, payee: address, value: u64, description: vector&lt;u8&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="Wallet.md#0x1_Wallet_new_timed_transfer">new_timed_transfer</a>(sender: &signer, payee: address, value: u64, description: vector&lt;u8&gt;): u64 <b>acquires</b> <a href="Wallet.md#0x1_Wallet_CommunityTransfers">CommunityTransfers</a>, <a href="Wallet.md#0x1_Wallet_CommunityWallets">CommunityWallets</a> {
    <b>let</b> sender_addr = <a href="Signer.md#0x1_Signer_address_of">Signer::address_of</a>(sender);
    <b>let</b> list = <a href="Wallet.md#0x1_Wallet_get_comm_list">get_comm_list</a>();

    <b>assert</b>(
      <a href="Vector.md#0x1_Vector_contains">Vector::contains</a>&lt;address&gt;(&list, &sender_addr), <a href="Errors.md#0x1_Errors_requires_role">Errors::requires_role</a>(<a href="Wallet.md#0x1_Wallet_ERR_PREFIX">ERR_PREFIX</a> + 001)
    );

    <b>let</b> d = borrow_global_mut&lt;<a href="Wallet.md#0x1_Wallet_CommunityTransfers">CommunityTransfers</a>&gt;(0x0);
    d.max_uid = d.max_uid + 1;

    // add current epoch + 1
    <b>let</b> current_epoch = <a href="LibraConfig.md#0x1_LibraConfig_get_current_epoch">LibraConfig::get_current_epoch</a>();

    <b>let</b> t = <a href="Wallet.md#0x1_Wallet_TimedTransfer">TimedTransfer</a> {
        uid: d.max_uid,
        expire_epoch: current_epoch + 3,
        payer: sender_addr,
        payee: payee,
        value: value,
        description: description,
        veto: <a href="Wallet.md#0x1_Wallet_Veto">Veto</a> {
          list: <a href="Vector.md#0x1_Vector_empty">Vector::empty</a>&lt;address&gt;(),
          count: 0,
          threshold: 0,
        }
    };

    <a href="Vector.md#0x1_Vector_push_back">Vector::push_back</a>&lt;<a href="Wallet.md#0x1_Wallet_TimedTransfer">TimedTransfer</a>&gt;(&<b>mut</b> d.proposed, t);
    <b>return</b> d.max_uid
  }
</code></pre>



</details>

<a name="0x1_Wallet_find"></a>

## Function `find`



<pre><code><b>public</b> <b>fun</b> <a href="Wallet.md#0x1_Wallet_find">find</a>(uid: u64, type_of: u8): (<a href="Option.md#0x1_Option_Option">Option::Option</a>&lt;<a href="Wallet.md#0x1_Wallet_TimedTransfer">Wallet::TimedTransfer</a>&gt;, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="Wallet.md#0x1_Wallet_find">find</a>(uid: u64, type_of: u8): (<a href="Option.md#0x1_Option">Option</a>&lt;<a href="Wallet.md#0x1_Wallet_TimedTransfer">TimedTransfer</a>&gt;, u64) <b>acquires</b> <a href="Wallet.md#0x1_Wallet_CommunityTransfers">CommunityTransfers</a> {
  <b>let</b> c = borrow_global&lt;<a href="Wallet.md#0x1_Wallet_CommunityTransfers">CommunityTransfers</a>&gt;(0x0);
  <b>let</b> list = <b>if</b> (type_of == 0) {
    &c.proposed
  } <b>else</b> <b>if</b> (type_of == 1) {
    &c.approved
  } <b>else</b> {
    &c.rejected
  };

  <b>let</b> len = <a href="Vector.md#0x1_Vector_length">Vector::length</a>(list);
  <b>let</b> i = 0;
  <b>while</b> (i &lt; len) {
    <b>let</b> t = *<a href="Vector.md#0x1_Vector_borrow">Vector::borrow</a>&lt;<a href="Wallet.md#0x1_Wallet_TimedTransfer">TimedTransfer</a>&gt;(list, i);
    <b>if</b> (t.uid == uid) {
      <b>return</b> (<a href="Option.md#0x1_Option_some">Option::some</a>&lt;<a href="Wallet.md#0x1_Wallet_TimedTransfer">TimedTransfer</a>&gt;(t), i)
    };
    i = i + 1;
  };
  (<a href="Option.md#0x1_Option_none">Option::none</a>&lt;<a href="Wallet.md#0x1_Wallet_TimedTransfer">TimedTransfer</a>&gt;(), 0)
}
</code></pre>



</details>

<a name="0x1_Wallet_veto"></a>

## Function `veto`



<pre><code><b>public</b> <b>fun</b> <a href="Wallet.md#0x1_Wallet_veto">veto</a>(sender: &signer, uid: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="Wallet.md#0x1_Wallet_veto">veto</a>(sender: &signer, uid: u64) <b>acquires</b> <a href="Wallet.md#0x1_Wallet_CommunityTransfers">CommunityTransfers</a>, <a href="Wallet.md#0x1_Wallet_CommunityFreeze">CommunityFreeze</a> {
  print(&0x110);
  <b>let</b> addr = <a href="Signer.md#0x1_Signer_address_of">Signer::address_of</a>(sender);
  <b>assert</b>(
    <a href="LibraSystem.md#0x1_LibraSystem_is_validator">LibraSystem::is_validator</a>(addr),
    <a href="Errors.md#0x1_Errors_requires_role">Errors::requires_role</a>(<a href="Wallet.md#0x1_Wallet_ERR_PREFIX">ERR_PREFIX</a> + 001)
  );
  print(&0x111);
  <b>let</b> (opt, i) = <a href="Wallet.md#0x1_Wallet_find">find</a>(uid, 0);
  <b>if</b> (<a href="Option.md#0x1_Option_is_some">Option::is_some</a>&lt;<a href="Wallet.md#0x1_Wallet_TimedTransfer">TimedTransfer</a>&gt;(&opt)) {
    <b>let</b> c = borrow_global_mut&lt;<a href="Wallet.md#0x1_Wallet_CommunityTransfers">CommunityTransfers</a>&gt;(0x0);
    <b>let</b> t = <a href="Vector.md#0x1_Vector_borrow_mut">Vector::borrow_mut</a>&lt;<a href="Wallet.md#0x1_Wallet_TimedTransfer">TimedTransfer</a>&gt;(&<b>mut</b> c.proposed, i);
    <a href="Vector.md#0x1_Vector_push_back">Vector::push_back</a>&lt;address&gt;(&<b>mut</b> t.veto.list, addr);
    print(&0x112);

    <b>if</b> (<a href="Wallet.md#0x1_Wallet_tally_veto">tally_veto</a>(i)) {
    print(&0x113);

      <a href="Wallet.md#0x1_Wallet_reject">reject</a>(uid)
    }
  };
}
</code></pre>



</details>

<a name="0x1_Wallet_reject"></a>

## Function `reject`



<pre><code><b>fun</b> <a href="Wallet.md#0x1_Wallet_reject">reject</a>(uid: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="Wallet.md#0x1_Wallet_reject">reject</a>(uid: u64) <b>acquires</b> <a href="Wallet.md#0x1_Wallet_CommunityTransfers">CommunityTransfers</a>, <a href="Wallet.md#0x1_Wallet_CommunityFreeze">CommunityFreeze</a> {
  print(&0x01131);
  <b>let</b> c = borrow_global_mut&lt;<a href="Wallet.md#0x1_Wallet_CommunityTransfers">CommunityTransfers</a>&gt;(0x0);
  <b>let</b> list = *&c.proposed;
  <b>let</b> len = <a href="Vector.md#0x1_Vector_length">Vector::length</a>(&list);
  <b>let</b> i = 0;
  print(&0x01132);
  <b>while</b> (i &lt; len) {
    print(&0x01133);
    <b>let</b> t = *<a href="Vector.md#0x1_Vector_borrow">Vector::borrow</a>&lt;<a href="Wallet.md#0x1_Wallet_TimedTransfer">TimedTransfer</a>&gt;(&list, i);
    <b>if</b> (t.uid == uid) {
      <a href="Vector.md#0x1_Vector_remove">Vector::remove</a>&lt;<a href="Wallet.md#0x1_Wallet_TimedTransfer">TimedTransfer</a>&gt;(&<b>mut</b> c.proposed, i);
      <b>let</b> f = borrow_global_mut&lt;<a href="Wallet.md#0x1_Wallet_CommunityFreeze">CommunityFreeze</a>&gt;(*&t.payer);
      f.consecutive_rejections = f.consecutive_rejections + 1;
      <a href="Vector.md#0x1_Vector_push_back">Vector::push_back</a>(&<b>mut</b> c.rejected, t);

    };

    i = i + 1;
  };

  print(&0x01134);

}
</code></pre>



</details>

<a name="0x1_Wallet_tally_veto"></a>

## Function `tally_veto`



<pre><code><b>fun</b> <a href="Wallet.md#0x1_Wallet_tally_veto">tally_veto</a>(index: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="Wallet.md#0x1_Wallet_tally_veto">tally_veto</a>(index: u64): bool <b>acquires</b> <a href="Wallet.md#0x1_Wallet_CommunityTransfers">CommunityTransfers</a> {
  <b>let</b> c = borrow_global_mut&lt;<a href="Wallet.md#0x1_Wallet_CommunityTransfers">CommunityTransfers</a>&gt;(0x0);
  <b>let</b> t = <a href="Vector.md#0x1_Vector_borrow_mut">Vector::borrow_mut</a>&lt;<a href="Wallet.md#0x1_Wallet_TimedTransfer">TimedTransfer</a>&gt;(&<b>mut</b> c.proposed, index);

  <b>let</b> votes = 0;
  <b>let</b> threshold = <a href="Wallet.md#0x1_Wallet_calculate_proportional_voting_threshold">calculate_proportional_voting_threshold</a>();

  <b>let</b> k = 0;
  <b>let</b> len = <a href="Vector.md#0x1_Vector_length">Vector::length</a>&lt;address&gt;(&t.veto.list);

  <b>while</b> (k &lt; len) {
    <b>let</b> addr = *<a href="Vector.md#0x1_Vector_borrow">Vector::borrow</a>&lt;address&gt;(&t.veto.list, k);
    // ignore votes that are no longer in the validator set,
    // BUT DON'T REMOVE, since they may rejoin the validator set, and shouldn't need <b>to</b> vote again.

    <b>if</b> (<a href="LibraSystem.md#0x1_LibraSystem_is_validator">LibraSystem::is_validator</a>(addr)) {
      votes = votes + <a href="NodeWeight.md#0x1_NodeWeight_proof_of_weight">NodeWeight::proof_of_weight</a>(addr)
    };
    k = k + 1;
  };

  t.veto.count = votes;
  t.veto.threshold = threshold;

  <b>return</b> votes &gt; threshold
}
</code></pre>



</details>

<a name="0x1_Wallet_calculate_proportional_voting_threshold"></a>

## Function `calculate_proportional_voting_threshold`



<pre><code><b>fun</b> <a href="Wallet.md#0x1_Wallet_calculate_proportional_voting_threshold">calculate_proportional_voting_threshold</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="Wallet.md#0x1_Wallet_calculate_proportional_voting_threshold">calculate_proportional_voting_threshold</a>(): u64 {
    <b>let</b> val_set_size = <a href="LibraSystem.md#0x1_LibraSystem_validator_set_size">LibraSystem::validator_set_size</a>();
    <b>let</b> i = 0;
    <b>let</b> voting_power = 0;
    <b>while</b> (i &lt; val_set_size) {
      <b>let</b> addr = <a href="LibraSystem.md#0x1_LibraSystem_get_ith_validator_address">LibraSystem::get_ith_validator_address</a>(i);
      voting_power = voting_power + <a href="NodeWeight.md#0x1_NodeWeight_proof_of_weight">NodeWeight::proof_of_weight</a>(addr);
      i = i + 1;
    };
    <b>let</b> threshold = voting_power * 2 / 3;
    threshold
}
</code></pre>



</details>

<a name="0x1_Wallet_list_tx_by_epoch"></a>

## Function `list_tx_by_epoch`



<pre><code><b>public</b> <b>fun</b> <a href="Wallet.md#0x1_Wallet_list_tx_by_epoch">list_tx_by_epoch</a>(epoch: u64): vector&lt;<a href="Wallet.md#0x1_Wallet_TimedTransfer">Wallet::TimedTransfer</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="Wallet.md#0x1_Wallet_list_tx_by_epoch">list_tx_by_epoch</a>(epoch: u64): vector&lt;<a href="Wallet.md#0x1_Wallet_TimedTransfer">TimedTransfer</a>&gt; <b>acquires</b> <a href="Wallet.md#0x1_Wallet_CommunityTransfers">CommunityTransfers</a> {
    <b>let</b> c = borrow_global_mut&lt;<a href="Wallet.md#0x1_Wallet_CommunityTransfers">CommunityTransfers</a>&gt;(0x0);
    // reset approved list
    c.approved = <a href="Vector.md#0x1_Vector_empty">Vector::empty</a>&lt;<a href="Wallet.md#0x1_Wallet_TimedTransfer">TimedTransfer</a>&gt;();
    // <b>loop</b> proposed list
    <b>let</b> pending = <a href="Vector.md#0x1_Vector_empty">Vector::empty</a>&lt;<a href="Wallet.md#0x1_Wallet_TimedTransfer">TimedTransfer</a>&gt;();
    <b>let</b> len = <a href="Vector.md#0x1_Vector_length">Vector::length</a>(&c.proposed);
    <b>let</b> i = 0;
    <b>while</b> (i &lt; len) {
      <b>let</b> t = <a href="Vector.md#0x1_Vector_borrow">Vector::borrow</a>(&c.proposed, i);
      <b>if</b> (t.expire_epoch == epoch) {

        <a href="Vector.md#0x1_Vector_push_back">Vector::push_back</a>&lt;<a href="Wallet.md#0x1_Wallet_TimedTransfer">TimedTransfer</a>&gt;(&<b>mut</b> pending, *t);
        // TODO: clear the freeze count on community wallet
        // add <b>to</b> approved list
      };
      i = i + 1;
    };
    <b>return</b> pending
  }
</code></pre>



</details>

<a name="0x1_Wallet_maybe_reset_rejection_counter"></a>

## Function `maybe_reset_rejection_counter`



<pre><code><b>public</b> <b>fun</b> <a href="Wallet.md#0x1_Wallet_maybe_reset_rejection_counter">maybe_reset_rejection_counter</a>(vm: &signer, wallet: address)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="Wallet.md#0x1_Wallet_maybe_reset_rejection_counter">maybe_reset_rejection_counter</a>(vm: &signer, wallet: address) <b>acquires</b> <a href="Wallet.md#0x1_Wallet_CommunityFreeze">CommunityFreeze</a> {
  <a href="CoreAddresses.md#0x1_CoreAddresses_assert_libra_root">CoreAddresses::assert_libra_root</a>(vm);
  <b>let</b> f = borrow_global_mut&lt;<a href="Wallet.md#0x1_Wallet_CommunityFreeze">CommunityFreeze</a>&gt;(wallet);
  f.consecutive_rejections = 0;
}
</code></pre>



</details>

<a name="0x1_Wallet_maybe_freeze"></a>

## Function `maybe_freeze`



<pre><code><b>fun</b> <a href="Wallet.md#0x1_Wallet_maybe_freeze">maybe_freeze</a>(wallet: address)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="Wallet.md#0x1_Wallet_maybe_freeze">maybe_freeze</a>(wallet: address) <b>acquires</b> <a href="Wallet.md#0x1_Wallet_CommunityFreeze">CommunityFreeze</a> {
  <b>let</b> f = borrow_global_mut&lt;<a href="Wallet.md#0x1_Wallet_CommunityFreeze">CommunityFreeze</a>&gt;(wallet);
  <b>if</b> (f.consecutive_rejections &gt; 2) {
    f.is_frozen = <b>true</b>;
  }
}
</code></pre>



</details>

<a name="0x1_Wallet_vote_to_unfreeze"></a>

## Function `vote_to_unfreeze`



<pre><code><b>public</b> <b>fun</b> <a href="Wallet.md#0x1_Wallet_vote_to_unfreeze">vote_to_unfreeze</a>(val: &signer, wallet: address)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="Wallet.md#0x1_Wallet_vote_to_unfreeze">vote_to_unfreeze</a>(val: &signer, wallet: address) <b>acquires</b> <a href="Wallet.md#0x1_Wallet_CommunityFreeze">CommunityFreeze</a> {
  <b>let</b> f = borrow_global_mut&lt;<a href="Wallet.md#0x1_Wallet_CommunityFreeze">CommunityFreeze</a>&gt;(wallet);
  <b>let</b> val_addr = <a href="Signer.md#0x1_Signer_address_of">Signer::address_of</a>(val);
  <a href="Vector.md#0x1_Vector_push_back">Vector::push_back</a>&lt;address&gt;(&<b>mut</b> f.unfreeze_votes, val_addr);

  <b>if</b> (<a href="Wallet.md#0x1_Wallet_tally_unfreeze">tally_unfreeze</a>(wallet)) {
    <b>let</b> f = borrow_global_mut&lt;<a href="Wallet.md#0x1_Wallet_CommunityFreeze">CommunityFreeze</a>&gt;(wallet);
    f.is_frozen = <b>false</b>;
  }
}
</code></pre>



</details>

<a name="0x1_Wallet_tally_unfreeze"></a>

## Function `tally_unfreeze`



<pre><code><b>fun</b> <a href="Wallet.md#0x1_Wallet_tally_unfreeze">tally_unfreeze</a>(wallet: address): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="Wallet.md#0x1_Wallet_tally_unfreeze">tally_unfreeze</a>(wallet: address): bool <b>acquires</b> <a href="Wallet.md#0x1_Wallet_CommunityFreeze">CommunityFreeze</a> {
  <b>let</b> f = borrow_global&lt;<a href="Wallet.md#0x1_Wallet_CommunityFreeze">CommunityFreeze</a>&gt;(wallet);

  <b>let</b> votes = 0;
  <b>let</b> threshold = <a href="Wallet.md#0x1_Wallet_calculate_proportional_voting_threshold">calculate_proportional_voting_threshold</a>();

  <b>let</b> k = 0;
  <b>let</b> len = <a href="Vector.md#0x1_Vector_length">Vector::length</a>&lt;address&gt;(&f.unfreeze_votes);

  <b>while</b> (k &lt; len) {
    <b>let</b> addr = *<a href="Vector.md#0x1_Vector_borrow">Vector::borrow</a>&lt;address&gt;(&f.unfreeze_votes, k);
    // ignore votes that are no longer in the validator set,
    // BUT DON'T REMOVE, since they may rejoin the validator set, and shouldn't need <b>to</b> vote again.

    <b>if</b> (<a href="LibraSystem.md#0x1_LibraSystem_is_validator">LibraSystem::is_validator</a>(addr)) {
      votes = votes + <a href="NodeWeight.md#0x1_NodeWeight_proof_of_weight">NodeWeight::proof_of_weight</a>(addr)
    };
    k = k + 1;
  };

  <b>return</b> votes &gt; threshold
}
</code></pre>



</details>

<a name="0x1_Wallet_get_tx_args"></a>

## Function `get_tx_args`



<pre><code><b>public</b> <b>fun</b> <a href="Wallet.md#0x1_Wallet_get_tx_args">get_tx_args</a>(t: <a href="Wallet.md#0x1_Wallet_TimedTransfer">Wallet::TimedTransfer</a>): (address, address, u64, vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="Wallet.md#0x1_Wallet_get_tx_args">get_tx_args</a>(t: <a href="Wallet.md#0x1_Wallet_TimedTransfer">TimedTransfer</a>): (address, address, u64, vector&lt;u8&gt;) {
  (t.payer, t.payee, t.value, *&t.description)
}
</code></pre>



</details>

<a name="0x1_Wallet_transfer_is_proposed"></a>

## Function `transfer_is_proposed`



<pre><code><b>public</b> <b>fun</b> <a href="Wallet.md#0x1_Wallet_transfer_is_proposed">transfer_is_proposed</a>(uid: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="Wallet.md#0x1_Wallet_transfer_is_proposed">transfer_is_proposed</a>(uid: u64): bool <b>acquires</b>  <a href="Wallet.md#0x1_Wallet_CommunityTransfers">CommunityTransfers</a> {
  <b>let</b> (opt, _) = <a href="Wallet.md#0x1_Wallet_find">find</a>(uid, 0);
  <a href="Option.md#0x1_Option_is_some">Option::is_some</a>&lt;<a href="Wallet.md#0x1_Wallet_TimedTransfer">TimedTransfer</a>&gt;(&opt)
}
</code></pre>



</details>

<a name="0x1_Wallet_transfer_is_rejected"></a>

## Function `transfer_is_rejected`



<pre><code><b>public</b> <b>fun</b> <a href="Wallet.md#0x1_Wallet_transfer_is_rejected">transfer_is_rejected</a>(uid: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="Wallet.md#0x1_Wallet_transfer_is_rejected">transfer_is_rejected</a>(uid: u64): bool <b>acquires</b>  <a href="Wallet.md#0x1_Wallet_CommunityTransfers">CommunityTransfers</a> {
  <b>let</b> (opt, _) = <a href="Wallet.md#0x1_Wallet_find">find</a>(uid, 2);
  <a href="Option.md#0x1_Option_is_some">Option::is_some</a>&lt;<a href="Wallet.md#0x1_Wallet_TimedTransfer">TimedTransfer</a>&gt;(&opt)
}
</code></pre>



</details>

<a name="0x1_Wallet_get_comm_list"></a>

## Function `get_comm_list`



<pre><code><b>public</b> <b>fun</b> <a href="Wallet.md#0x1_Wallet_get_comm_list">get_comm_list</a>(): vector&lt;address&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="Wallet.md#0x1_Wallet_get_comm_list">get_comm_list</a>(): vector&lt;address&gt; <b>acquires</b> <a href="Wallet.md#0x1_Wallet_CommunityWallets">CommunityWallets</a>{
  <b>if</b> (<b>exists</b>&lt;<a href="Wallet.md#0x1_Wallet_CommunityWallets">CommunityWallets</a>&gt;(0x0)) {
    <b>let</b> s = borrow_global&lt;<a href="Wallet.md#0x1_Wallet_CommunityWallets">CommunityWallets</a>&gt;(0x0);
    <b>return</b> *&s.list
  } <b>else</b> {
    <b>return</b> <a href="Vector.md#0x1_Vector_empty">Vector::empty</a>&lt;address&gt;()
  }
}
</code></pre>



</details>

<a name="0x1_Wallet_is_comm"></a>

## Function `is_comm`



<pre><code><b>public</b> <b>fun</b> <a href="Wallet.md#0x1_Wallet_is_comm">is_comm</a>(addr: address): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="Wallet.md#0x1_Wallet_is_comm">is_comm</a>(addr: address): bool <b>acquires</b> <a href="Wallet.md#0x1_Wallet_CommunityWallets">CommunityWallets</a>{
  <b>let</b> s = borrow_global&lt;<a href="Wallet.md#0x1_Wallet_CommunityWallets">CommunityWallets</a>&gt;(0x0);
  <a href="Vector.md#0x1_Vector_contains">Vector::contains</a>&lt;address&gt;(&s.list, &addr)
}
</code></pre>



</details>

<a name="0x1_Wallet_is_frozen"></a>

## Function `is_frozen`



<pre><code><b>public</b> <b>fun</b> <a href="Wallet.md#0x1_Wallet_is_frozen">is_frozen</a>(addr: address): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="Wallet.md#0x1_Wallet_is_frozen">is_frozen</a>(addr: address): bool <b>acquires</b> <a href="Wallet.md#0x1_Wallet_CommunityFreeze">CommunityFreeze</a>{
  <b>let</b> f = borrow_global&lt;<a href="Wallet.md#0x1_Wallet_CommunityFreeze">CommunityFreeze</a>&gt;(addr);
  f.is_frozen
}
</code></pre>



</details>

<a name="0x1_Wallet_set_slow"></a>

## Function `set_slow`



<pre><code><b>public</b> <b>fun</b> <a href="Wallet.md#0x1_Wallet_set_slow">set_slow</a>(sig: &signer)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="Wallet.md#0x1_Wallet_set_slow">set_slow</a>(sig: &signer) {
  <b>if</b> (!<b>exists</b>&lt;<a href="Wallet.md#0x1_Wallet_SlowWallet">SlowWallet</a>&gt;(<a href="Signer.md#0x1_Signer_address_of">Signer::address_of</a>(sig))) {
    move_to&lt;<a href="Wallet.md#0x1_Wallet_SlowWallet">SlowWallet</a>&gt;(sig, <a href="Wallet.md#0x1_Wallet_SlowWallet">SlowWallet</a> {
      is_slow: <b>true</b>
    });
  }
}
</code></pre>



</details>

<a name="0x1_Wallet_is_slow"></a>

## Function `is_slow`



<pre><code><b>public</b> <b>fun</b> <a href="Wallet.md#0x1_Wallet_is_slow">is_slow</a>(addr: address): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="Wallet.md#0x1_Wallet_is_slow">is_slow</a>(addr: address): bool {
  <b>exists</b>&lt;<a href="Wallet.md#0x1_Wallet_SlowWallet">SlowWallet</a>&gt;(addr)
}
</code></pre>



</details>


[//]: # ("File containing references which can be used from documentation")
[ACCESS_CONTROL]: https://github.com/libra/lip/blob/master/lips/lip-2.md
[ROLE]: https://github.com/libra/lip/blob/master/lips/lip-2.md#roles
[PERMISSION]: https://github.com/libra/lip/blob/master/lips/lip-2.md#permissions
