
<a name="0x1_Burn"></a>

# Module `0x1::Burn`



-  [Resource `BurnPreference`](#0x1_Burn_BurnPreference)
-  [Resource `DepositInfo`](#0x1_Burn_DepositInfo)
-  [Function `reset_ratios`](#0x1_Burn_reset_ratios)
-  [Function `get_address_list`](#0x1_Burn_get_address_list)
-  [Function `get_value`](#0x1_Burn_get_value)
-  [Function `epoch_start_burn`](#0x1_Burn_epoch_start_burn)
-  [Function `burn`](#0x1_Burn_burn)
-  [Function `send`](#0x1_Burn_send)
-  [Function `get_ratios`](#0x1_Burn_get_ratios)


<pre><code><b>use</b> <a href="CoreAddresses.md#0x1_CoreAddresses">0x1::CoreAddresses</a>;
<b>use</b> <a href="FixedPoint32.md#0x1_FixedPoint32">0x1::FixedPoint32</a>;
<b>use</b> <a href="GAS.md#0x1_GAS">0x1::GAS</a>;
<b>use</b> <a href="LibraAccount.md#0x1_LibraAccount">0x1::LibraAccount</a>;
<b>use</b> <a href="Vector.md#0x1_Vector">0x1::Vector</a>;
<b>use</b> <a href="Wallet.md#0x1_Wallet">0x1::Wallet</a>;
</code></pre>



<a name="0x1_Burn_BurnPreference"></a>

## Resource `BurnPreference`



<pre><code><b>resource</b> <b>struct</b> <a href="Burn.md#0x1_Burn_BurnPreference">BurnPreference</a>
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>is_burn: bool</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x1_Burn_DepositInfo"></a>

## Resource `DepositInfo`



<pre><code><b>resource</b> <b>struct</b> <a href="Burn.md#0x1_Burn_DepositInfo">DepositInfo</a>
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>addr: vector&lt;address&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>deposits: vector&lt;u64&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>ratio: vector&lt;<a href="FixedPoint32.md#0x1_FixedPoint32_FixedPoint32">FixedPoint32::FixedPoint32</a>&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x1_Burn_reset_ratios"></a>

## Function `reset_ratios`



<pre><code><b>public</b> <b>fun</b> <a href="Burn.md#0x1_Burn_reset_ratios">reset_ratios</a>(vm: &signer)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="Burn.md#0x1_Burn_reset_ratios">reset_ratios</a>(vm: &signer) <b>acquires</b> <a href="Burn.md#0x1_Burn_DepositInfo">DepositInfo</a> {
  <a href="CoreAddresses.md#0x1_CoreAddresses_assert_libra_root">CoreAddresses::assert_libra_root</a>(vm);
  <b>let</b> list = <a href="Wallet.md#0x1_Wallet_get_comm_list">Wallet::get_comm_list</a>();

  <b>let</b> len = <a href="Vector.md#0x1_Vector_length">Vector::length</a>(&list);
  <b>let</b> i = 0;
  <b>let</b> global_deposits = 0;
  <b>let</b> deposit_vec = <a href="Vector.md#0x1_Vector_empty">Vector::empty</a>&lt;u64&gt;();

  <b>while</b> (i &lt; len) {

    <b>let</b> addr = *<a href="Vector.md#0x1_Vector_borrow">Vector::borrow</a>(&list, i);
    <b>let</b> cumu = <a href="LibraAccount.md#0x1_LibraAccount_get_index_cumu_deposits">LibraAccount::get_index_cumu_deposits</a>(addr);

    global_deposits = global_deposits + cumu;
    <a href="Vector.md#0x1_Vector_push_back">Vector::push_back</a>(&<b>mut</b> deposit_vec, cumu);
    i = i + 1;
  };

  <b>let</b> ratios_vec = <a href="Vector.md#0x1_Vector_empty">Vector::empty</a>&lt;<a href="FixedPoint32.md#0x1_FixedPoint32_FixedPoint32">FixedPoint32::FixedPoint32</a>&gt;();
  <b>let</b> k = 0;
  <b>while</b> (k &lt; len) {
    <b>let</b> cumu = *<a href="Vector.md#0x1_Vector_borrow">Vector::borrow</a>(&deposit_vec, k);

    <b>if</b> (cumu == 0) {
      k = k + 1;
      <b>continue</b>
    };

    <b>let</b> ratio = <a href="FixedPoint32.md#0x1_FixedPoint32_create_from_rational">FixedPoint32::create_from_rational</a>(cumu, global_deposits);
    <a href="Vector.md#0x1_Vector_push_back">Vector::push_back</a>(&<b>mut</b> ratios_vec, ratio);
    k = k + 1;
  };

  <b>if</b> (<b>exists</b>&lt;<a href="Burn.md#0x1_Burn_DepositInfo">DepositInfo</a>&gt;(0x0)) {
    <b>let</b> d = borrow_global_mut&lt;<a href="Burn.md#0x1_Burn_DepositInfo">DepositInfo</a>&gt;(0x0);
    d.addr = list;
    d.deposits = deposit_vec;
    d.ratio = ratios_vec;
  } <b>else</b> {
    move_to&lt;<a href="Burn.md#0x1_Burn_DepositInfo">DepositInfo</a>&gt;(vm, <a href="Burn.md#0x1_Burn_DepositInfo">DepositInfo</a> {
      addr: list,
      deposits: deposit_vec,
      ratio: ratios_vec,
    })
  }
}
</code></pre>



</details>

<a name="0x1_Burn_get_address_list"></a>

## Function `get_address_list`



<pre><code><b>fun</b> <a href="Burn.md#0x1_Burn_get_address_list">get_address_list</a>(): vector&lt;address&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="Burn.md#0x1_Burn_get_address_list">get_address_list</a>(): vector&lt;address&gt; <b>acquires</b> <a href="Burn.md#0x1_Burn_DepositInfo">DepositInfo</a> {
  *&borrow_global&lt;<a href="Burn.md#0x1_Burn_DepositInfo">DepositInfo</a>&gt;(0x0).addr
}
</code></pre>



</details>

<a name="0x1_Burn_get_value"></a>

## Function `get_value`



<pre><code><b>fun</b> <a href="Burn.md#0x1_Burn_get_value">get_value</a>(payee: address, value: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="Burn.md#0x1_Burn_get_value">get_value</a>(payee: address, value: u64): u64 <b>acquires</b> <a href="Burn.md#0x1_Burn_DepositInfo">DepositInfo</a> {
  <b>let</b> d = borrow_global&lt;<a href="Burn.md#0x1_Burn_DepositInfo">DepositInfo</a>&gt;(0x0);
  <b>let</b> (_, i) = <a href="Vector.md#0x1_Vector_index_of">Vector::index_of</a>(&d.addr, &payee);
  <b>let</b> ratio = *<a href="Vector.md#0x1_Vector_borrow">Vector::borrow</a>(&d.ratio, i);
  <a href="FixedPoint32.md#0x1_FixedPoint32_multiply_u64">FixedPoint32::multiply_u64</a>(value, ratio)
}
</code></pre>



</details>

<a name="0x1_Burn_epoch_start_burn"></a>

## Function `epoch_start_burn`



<pre><code><b>public</b> <b>fun</b> <a href="Burn.md#0x1_Burn_epoch_start_burn">epoch_start_burn</a>(vm: &signer, payer: address, value: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="Burn.md#0x1_Burn_epoch_start_burn">epoch_start_burn</a>(vm: &signer, payer: address, value: u64) <b>acquires</b> <a href="Burn.md#0x1_Burn_DepositInfo">DepositInfo</a>, <a href="Burn.md#0x1_Burn_BurnPreference">BurnPreference</a> {
  <b>if</b> (<b>exists</b>&lt;<a href="Burn.md#0x1_Burn_BurnPreference">BurnPreference</a>&gt;(payer)) {
    <b>if</b> (borrow_global&lt;<a href="Burn.md#0x1_Burn_BurnPreference">BurnPreference</a>&gt;(payer).is_burn) {
      <b>return</b> <a href="Burn.md#0x1_Burn_burn">burn</a>(vm, payer, value)
    }
  };
  <a href="Burn.md#0x1_Burn_send">send</a>(vm, payer, value);
}
</code></pre>



</details>

<a name="0x1_Burn_burn"></a>

## Function `burn`



<pre><code><b>fun</b> <a href="Burn.md#0x1_Burn_burn">burn</a>(vm: &signer, payer: address, value: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="Burn.md#0x1_Burn_burn">burn</a>(vm: &signer, payer: address, value: u64) {
    <a href="LibraAccount.md#0x1_LibraAccount_vm_make_payment_no_limit">LibraAccount::vm_make_payment_no_limit</a>&lt;<a href="GAS.md#0x1_GAS">GAS</a>&gt;(
        payer,
        0xDEADDEAD,
        value,
        b"epoch start burn",
        b"",
        vm,
    );
}
</code></pre>



</details>

<a name="0x1_Burn_send"></a>

## Function `send`



<pre><code><b>fun</b> <a href="Burn.md#0x1_Burn_send">send</a>(vm: &signer, payer: address, value: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="Burn.md#0x1_Burn_send">send</a>(vm: &signer, payer: address, value: u64) <b>acquires</b> <a href="Burn.md#0x1_Burn_DepositInfo">DepositInfo</a> {
  <b>let</b> list = <a href="Burn.md#0x1_Burn_get_address_list">get_address_list</a>();
  <b>let</b> len = <a href="Vector.md#0x1_Vector_length">Vector::length</a>&lt;address&gt;(&list);

  <b>let</b> i = 0;
  <b>while</b> (i &lt; len) {
    <b>let</b> payee = *<a href="Vector.md#0x1_Vector_borrow">Vector::borrow</a>&lt;address&gt;(&list, i);
    <b>let</b> val = <a href="Burn.md#0x1_Burn_get_value">get_value</a>(payee, value);

    <a href="LibraAccount.md#0x1_LibraAccount_vm_make_payment_no_limit">LibraAccount::vm_make_payment_no_limit</a>&lt;<a href="GAS.md#0x1_GAS">GAS</a>&gt;(
        payer,
        payee,
        val,
        b"epoch start send",
        b"",
        vm,
    );

    i = i + 1;
  };
}
</code></pre>



</details>

<a name="0x1_Burn_get_ratios"></a>

## Function `get_ratios`



<pre><code><b>public</b> <b>fun</b> <a href="Burn.md#0x1_Burn_get_ratios">get_ratios</a>(): (vector&lt;address&gt;, vector&lt;u64&gt;, vector&lt;<a href="FixedPoint32.md#0x1_FixedPoint32_FixedPoint32">FixedPoint32::FixedPoint32</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="Burn.md#0x1_Burn_get_ratios">get_ratios</a>(): (vector&lt;address&gt;, vector&lt;u64&gt;, vector&lt;<a href="FixedPoint32.md#0x1_FixedPoint32_FixedPoint32">FixedPoint32::FixedPoint32</a>&gt;) <b>acquires</b> <a href="Burn.md#0x1_Burn_DepositInfo">DepositInfo</a> {
  <b>let</b> d = borrow_global&lt;<a href="Burn.md#0x1_Burn_DepositInfo">DepositInfo</a>&gt;(0x0);
  (*&d.addr, *&d.deposits, *&d.ratio)

}
</code></pre>



</details>


[//]: # ("File containing references which can be used from documentation")
[ACCESS_CONTROL]: https://github.com/libra/lip/blob/master/lips/lip-2.md
[ROLE]: https://github.com/libra/lip/blob/master/lips/lip-2.md#roles
[PERMISSION]: https://github.com/libra/lip/blob/master/lips/lip-2.md#permissions
