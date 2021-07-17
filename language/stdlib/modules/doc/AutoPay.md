
<a name="0x1_AutoPay2"></a>

# Module `0x1::AutoPay2`



-  [Resource `Tick`](#0x1_AutoPay2_Tick)
-  [Resource `AccountLimitsEnable`](#0x1_AutoPay2_AccountLimitsEnable)
-  [Resource `Data`](#0x1_AutoPay2_Data)
-  [Resource `AccountList`](#0x1_AutoPay2_AccountList)
-  [Struct `Payment`](#0x1_AutoPay2_Payment)
-  [Constants](#@Constants_0)
-  [Function `tick`](#0x1_AutoPay2_tick)
-  [Function `reconfig_reset_tick`](#0x1_AutoPay2_reconfig_reset_tick)
-  [Function `initialize`](#0x1_AutoPay2_initialize)
-  [Function `enable_account_limits`](#0x1_AutoPay2_enable_account_limits)
-  [Function `get_all_payees`](#0x1_AutoPay2_get_all_payees)
-  [Function `process_autopay`](#0x1_AutoPay2_process_autopay)
-  [Function `enable_autopay`](#0x1_AutoPay2_enable_autopay)
-  [Function `disable_autopay`](#0x1_AutoPay2_disable_autopay)
-  [Function `create_instruction`](#0x1_AutoPay2_create_instruction)
-  [Function `delete_instruction`](#0x1_AutoPay2_delete_instruction)
-  [Function `is_enabled`](#0x1_AutoPay2_is_enabled)
-  [Function `query_instruction`](#0x1_AutoPay2_query_instruction)
-  [Function `find`](#0x1_AutoPay2_find)


<pre><code><b>use</b> <a href="CoreAddresses.md#0x1_CoreAddresses">0x1::CoreAddresses</a>;
<b>use</b> <a href="Errors.md#0x1_Errors">0x1::Errors</a>;
<b>use</b> <a href="FixedPoint32.md#0x1_FixedPoint32">0x1::FixedPoint32</a>;
<b>use</b> <a href="GAS.md#0x1_GAS">0x1::GAS</a>;
<b>use</b> <a href="LibraAccount.md#0x1_LibraAccount">0x1::LibraAccount</a>;
<b>use</b> <a href="LibraConfig.md#0x1_LibraConfig">0x1::LibraConfig</a>;
<b>use</b> <a href="Option.md#0x1_Option">0x1::Option</a>;
<b>use</b> <a href="Signer.md#0x1_Signer">0x1::Signer</a>;
<b>use</b> <a href="Vector.md#0x1_Vector">0x1::Vector</a>;
<b>use</b> <a href="Wallet.md#0x1_Wallet">0x1::Wallet</a>;
</code></pre>



<a name="0x1_AutoPay2_Tick"></a>

## Resource `Tick`



<pre><code><b>resource</b> <b>struct</b> <a href="AutoPay.md#0x1_AutoPay2_Tick">Tick</a>
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>triggered: bool</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x1_AutoPay2_AccountLimitsEnable"></a>

## Resource `AccountLimitsEnable`



<pre><code><b>resource</b> <b>struct</b> <a href="AutoPay.md#0x1_AutoPay2_AccountLimitsEnable">AccountLimitsEnable</a>
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>enabled: bool</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x1_AutoPay2_Data"></a>

## Resource `Data`



<pre><code><b>resource</b> <b>struct</b> <a href="AutoPay.md#0x1_AutoPay2_Data">Data</a>
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>payments: vector&lt;<a href="AutoPay.md#0x1_AutoPay2_Payment">AutoPay2::Payment</a>&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x1_AutoPay2_AccountList"></a>

## Resource `AccountList`



<pre><code><b>resource</b> <b>struct</b> <a href="AutoPay.md#0x1_AutoPay2_AccountList">AccountList</a>
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>accounts: vector&lt;address&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>current_epoch: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x1_AutoPay2_Payment"></a>

## Struct `Payment`



<pre><code><b>struct</b> <a href="AutoPay.md#0x1_AutoPay2_Payment">Payment</a>
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
<code>in_type: u8</code>
</dt>
<dd>

</dd>
<dt>
<code>payee: address</code>
</dt>
<dd>

</dd>
<dt>
<code>end_epoch: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>prev_bal: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>amt: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0x1_AutoPay2_EPAYEE_DOES_NOT_EXIST"></a>



<pre><code><b>const</b> <a href="AutoPay.md#0x1_AutoPay2_EPAYEE_DOES_NOT_EXIST">EPAYEE_DOES_NOT_EXIST</a>: u64 = 10017;
</code></pre>



<a name="0x1_AutoPay2_AUTOPAY_ID_EXISTS"></a>

Attempting to re-use autopay id


<pre><code><b>const</b> <a href="AutoPay.md#0x1_AutoPay2_AUTOPAY_ID_EXISTS">AUTOPAY_ID_EXISTS</a>: u64 = 10019;
</code></pre>



<a name="0x1_AutoPay2_EAUTOPAY_NOT_ENABLED"></a>

The account does not have autopay enabled.


<pre><code><b>const</b> <a href="AutoPay.md#0x1_AutoPay2_EAUTOPAY_NOT_ENABLED">EAUTOPAY_NOT_ENABLED</a>: u64 = 10018;
</code></pre>



<a name="0x1_AutoPay2_FIXED_ONCE"></a>

send a certain amount once at the next tick payment type


<pre><code><b>const</b> <a href="AutoPay.md#0x1_AutoPay2_FIXED_ONCE">FIXED_ONCE</a>: u8 = 3;
</code></pre>



<a name="0x1_AutoPay2_FIXED_RECURRING"></a>

send a certain amount each tick until end_epoch is reached payment type


<pre><code><b>const</b> <a href="AutoPay.md#0x1_AutoPay2_FIXED_RECURRING">FIXED_RECURRING</a>: u8 = 2;
</code></pre>



<a name="0x1_AutoPay2_INVALID_PAYMENT_TYPE"></a>

Invalid payment type given


<pre><code><b>const</b> <a href="AutoPay.md#0x1_AutoPay2_INVALID_PAYMENT_TYPE">INVALID_PAYMENT_TYPE</a>: u64 = 10020;
</code></pre>



<a name="0x1_AutoPay2_MAX_NUMBER_OF_INSTRUCTIONS"></a>



<pre><code><b>const</b> <a href="AutoPay.md#0x1_AutoPay2_MAX_NUMBER_OF_INSTRUCTIONS">MAX_NUMBER_OF_INSTRUCTIONS</a>: u64 = 30;
</code></pre>



<a name="0x1_AutoPay2_MAX_TYPE"></a>

Attempted to send funds to an account that does not exist
Maximum value for the Payment type selection


<pre><code><b>const</b> <a href="AutoPay.md#0x1_AutoPay2_MAX_TYPE">MAX_TYPE</a>: u8 = 3;
</code></pre>



<a name="0x1_AutoPay2_PERCENT_OF_BALANCE"></a>

send percent of balance at end of epoch payment type


<pre><code><b>const</b> <a href="AutoPay.md#0x1_AutoPay2_PERCENT_OF_BALANCE">PERCENT_OF_BALANCE</a>: u8 = 0;
</code></pre>



<a name="0x1_AutoPay2_PERCENT_OF_CHANGE"></a>

send percent of the change in balance since the last tick payment type


<pre><code><b>const</b> <a href="AutoPay.md#0x1_AutoPay2_PERCENT_OF_CHANGE">PERCENT_OF_CHANGE</a>: u8 = 1;
</code></pre>



<a name="0x1_AutoPay2_TOO_MANY_INSTRUCTIONS"></a>

Attempt to add instruction when too many already exist


<pre><code><b>const</b> <a href="AutoPay.md#0x1_AutoPay2_TOO_MANY_INSTRUCTIONS">TOO_MANY_INSTRUCTIONS</a>: u64 = 10021;
</code></pre>



<a name="0x1_AutoPay2_tick"></a>

## Function `tick`



<pre><code><b>public</b> <b>fun</b> <a href="AutoPay.md#0x1_AutoPay2_tick">tick</a>(vm: &signer): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="AutoPay.md#0x1_AutoPay2_tick">tick</a>(vm: &signer): bool <b>acquires</b> <a href="AutoPay.md#0x1_AutoPay2_Tick">Tick</a> {
  <b>assert</b>(<a href="Signer.md#0x1_Signer_address_of">Signer::address_of</a>(vm) == <a href="CoreAddresses.md#0x1_CoreAddresses_LIBRA_ROOT_ADDRESS">CoreAddresses::LIBRA_ROOT_ADDRESS</a>(), <a href="Errors.md#0x1_Errors_requires_role">Errors::requires_role</a>(010001));
  <b>if</b> (<b>exists</b>&lt;<a href="AutoPay.md#0x1_AutoPay2_Tick">Tick</a>&gt;(<a href="CoreAddresses.md#0x1_CoreAddresses_LIBRA_ROOT_ADDRESS">CoreAddresses::LIBRA_ROOT_ADDRESS</a>())) {
    <b>let</b> tick_state = borrow_global_mut&lt;<a href="AutoPay.md#0x1_AutoPay2_Tick">Tick</a>&gt;(<a href="Signer.md#0x1_Signer_address_of">Signer::address_of</a>(vm));

    <b>if</b> (!tick_state.triggered) {
      tick_state.triggered = <b>true</b>;
      <b>return</b> <b>true</b>
    };
  } <b>else</b> {
    <a href="AutoPay.md#0x1_AutoPay2_initialize">initialize</a>(vm);
  };
  <b>false</b>
}
</code></pre>



</details>

<a name="0x1_AutoPay2_reconfig_reset_tick"></a>

## Function `reconfig_reset_tick`



<pre><code><b>public</b> <b>fun</b> <a href="AutoPay.md#0x1_AutoPay2_reconfig_reset_tick">reconfig_reset_tick</a>(vm: &signer)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="AutoPay.md#0x1_AutoPay2_reconfig_reset_tick">reconfig_reset_tick</a>(vm: &signer) <b>acquires</b> <a href="AutoPay.md#0x1_AutoPay2_Tick">Tick</a>{
  <b>let</b> tick_state = borrow_global_mut&lt;<a href="AutoPay.md#0x1_AutoPay2_Tick">Tick</a>&gt;(<a href="Signer.md#0x1_Signer_address_of">Signer::address_of</a>(vm));
  tick_state.triggered = <b>false</b>;
}
</code></pre>



</details>

<a name="0x1_AutoPay2_initialize"></a>

## Function `initialize`



<pre><code><b>public</b> <b>fun</b> <a href="AutoPay.md#0x1_AutoPay2_initialize">initialize</a>(sender: &signer)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="AutoPay.md#0x1_AutoPay2_initialize">initialize</a>(sender: &signer) {
  <b>assert</b>(<a href="Signer.md#0x1_Signer_address_of">Signer::address_of</a>(sender) == <a href="CoreAddresses.md#0x1_CoreAddresses_LIBRA_ROOT_ADDRESS">CoreAddresses::LIBRA_ROOT_ADDRESS</a>(), <a href="Errors.md#0x1_Errors_requires_role">Errors::requires_role</a>(010002));
  move_to&lt;<a href="AutoPay.md#0x1_AutoPay2_AccountList">AccountList</a>&gt;(sender, <a href="AutoPay.md#0x1_AutoPay2_AccountList">AccountList</a> { accounts: <a href="Vector.md#0x1_Vector_empty">Vector::empty</a>&lt;address&gt;(), current_epoch: 0, });
  move_to&lt;<a href="AutoPay.md#0x1_AutoPay2_Tick">Tick</a>&gt;(sender, <a href="AutoPay.md#0x1_AutoPay2_Tick">Tick</a> {triggered: <b>false</b>});
  move_to&lt;<a href="AutoPay.md#0x1_AutoPay2_AccountLimitsEnable">AccountLimitsEnable</a>&gt;(sender, <a href="AutoPay.md#0x1_AutoPay2_AccountLimitsEnable">AccountLimitsEnable</a> {enabled: <b>false</b>});

  <a href="LibraAccount.md#0x1_LibraAccount_initialize_escrow_root">LibraAccount::initialize_escrow_root</a>&lt;<a href="GAS.md#0x1_GAS">GAS</a>&gt;(sender);
}
</code></pre>



</details>

<a name="0x1_AutoPay2_enable_account_limits"></a>

## Function `enable_account_limits`



<pre><code><b>public</b> <b>fun</b> <a href="AutoPay.md#0x1_AutoPay2_enable_account_limits">enable_account_limits</a>(sender: &signer)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="AutoPay.md#0x1_AutoPay2_enable_account_limits">enable_account_limits</a>(sender: &signer) <b>acquires</b> <a href="AutoPay.md#0x1_AutoPay2_AccountLimitsEnable">AccountLimitsEnable</a> {
  <b>assert</b>(<a href="Signer.md#0x1_Signer_address_of">Signer::address_of</a>(sender) == <a href="CoreAddresses.md#0x1_CoreAddresses_LIBRA_ROOT_ADDRESS">CoreAddresses::LIBRA_ROOT_ADDRESS</a>(), <a href="Errors.md#0x1_Errors_requires_role">Errors::requires_role</a>(010002));
  <b>let</b> limits_enable = borrow_global_mut&lt;<a href="AutoPay.md#0x1_AutoPay2_AccountLimitsEnable">AccountLimitsEnable</a>&gt;(<a href="Signer.md#0x1_Signer_address_of">Signer::address_of</a>(sender));
  limits_enable.enabled = <b>true</b>;
}
</code></pre>



</details>

<a name="0x1_AutoPay2_get_all_payees"></a>

## Function `get_all_payees`



<pre><code><b>public</b> <b>fun</b> <a href="AutoPay.md#0x1_AutoPay2_get_all_payees">get_all_payees</a>(): vector&lt;address&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="AutoPay.md#0x1_AutoPay2_get_all_payees">get_all_payees</a>():vector&lt;address&gt; <b>acquires</b> <a href="AutoPay.md#0x1_AutoPay2_AccountList">AccountList</a>, <a href="AutoPay.md#0x1_AutoPay2_Data">Data</a> {
  <b>let</b> account_list = &borrow_global&lt;<a href="AutoPay.md#0x1_AutoPay2_AccountList">AccountList</a>&gt;(<a href="CoreAddresses.md#0x1_CoreAddresses_LIBRA_ROOT_ADDRESS">CoreAddresses::LIBRA_ROOT_ADDRESS</a>()).accounts;
  <b>let</b> accounts_length = <a href="Vector.md#0x1_Vector_length">Vector::length</a>&lt;address&gt;(account_list);
  <b>let</b> account_idx = 0;
  <b>let</b> payee_vec = <a href="Vector.md#0x1_Vector_empty">Vector::empty</a>&lt;address&gt;();

// print(&02200);
  <b>while</b> (account_idx &lt; accounts_length) {
    <b>let</b> account_addr = <a href="Vector.md#0x1_Vector_borrow">Vector::borrow</a>&lt;address&gt;(account_list, account_idx);
    // Obtain the account balance
    // <b>let</b> account_bal = <a href="LibraAccount.md#0x1_LibraAccount_balance">LibraAccount::balance</a>&lt;<a href="GAS.md#0x1_GAS">GAS</a>&gt;(*account_addr);
    // Go through all payments for this account and pay
    <b>let</b> payments = &<b>mut</b> borrow_global_mut&lt;<a href="AutoPay.md#0x1_AutoPay2_Data">Data</a>&gt;(*account_addr).payments;
    <b>let</b> payments_len = <a href="Vector.md#0x1_Vector_length">Vector::length</a>&lt;<a href="AutoPay.md#0x1_AutoPay2_Payment">Payment</a>&gt;(payments);
    <b>let</b> payments_idx = 0;
    <b>while</b> (payments_idx &lt; payments_len) {
      <b>let</b> payment = <a href="Vector.md#0x1_Vector_borrow_mut">Vector::borrow_mut</a>&lt;<a href="AutoPay.md#0x1_AutoPay2_Payment">Payment</a>&gt;(payments, payments_idx);
      <b>if</b> (!<a href="Vector.md#0x1_Vector_contains">Vector::contains</a>&lt;address&gt;(&payee_vec, &payment.payee)) {
        <a href="Vector.md#0x1_Vector_push_back">Vector::push_back</a>&lt;address&gt;(&<b>mut</b> payee_vec, payment.payee);
      };
      payments_idx = payments_idx + 1;
    };
    account_idx = account_idx + 1;
  };
  <b>return</b> payee_vec
}
</code></pre>



</details>

<a name="0x1_AutoPay2_process_autopay"></a>

## Function `process_autopay`



<pre><code><b>public</b> <b>fun</b> <a href="AutoPay.md#0x1_AutoPay2_process_autopay">process_autopay</a>(vm: &signer)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="AutoPay.md#0x1_AutoPay2_process_autopay">process_autopay</a>(
  vm: &signer,
) <b>acquires</b> <a href="AutoPay.md#0x1_AutoPay2_AccountList">AccountList</a>, <a href="AutoPay.md#0x1_AutoPay2_Data">Data</a>, <a href="AutoPay.md#0x1_AutoPay2_AccountLimitsEnable">AccountLimitsEnable</a> {
  // Only account 0x0 should be triggering this autopayment each block
  <b>assert</b>(<a href="Signer.md#0x1_Signer_address_of">Signer::address_of</a>(vm) == <a href="CoreAddresses.md#0x1_CoreAddresses_LIBRA_ROOT_ADDRESS">CoreAddresses::LIBRA_ROOT_ADDRESS</a>(), <a href="Errors.md#0x1_Errors_requires_role">Errors::requires_role</a>(010003));

  <b>let</b> epoch = <a href="LibraConfig.md#0x1_LibraConfig_get_current_epoch">LibraConfig::get_current_epoch</a>();
// print(&02100);

  // Go through all accounts in <a href="AutoPay.md#0x1_AutoPay2_AccountList">AccountList</a>
  // This is the list of accounts which currently have autopay enabled
  <b>let</b> account_list = &borrow_global&lt;<a href="AutoPay.md#0x1_AutoPay2_AccountList">AccountList</a>&gt;(<a href="CoreAddresses.md#0x1_CoreAddresses_LIBRA_ROOT_ADDRESS">CoreAddresses::LIBRA_ROOT_ADDRESS</a>()).accounts;
  <b>let</b> accounts_length = <a href="Vector.md#0x1_Vector_length">Vector::length</a>&lt;address&gt;(account_list);
  <b>let</b> account_idx = 0;
// print(&02200);
  <b>while</b> (account_idx &lt; accounts_length) {
// print(&02210);

    <b>let</b> account_addr = <a href="Vector.md#0x1_Vector_borrow">Vector::borrow</a>&lt;address&gt;(account_list, account_idx);
    // Obtain the account balance
    <b>let</b> account_bal = <a href="LibraAccount.md#0x1_LibraAccount_balance">LibraAccount::balance</a>&lt;<a href="GAS.md#0x1_GAS">GAS</a>&gt;(*account_addr);
    // Go through all payments for this account and pay
    <b>let</b> payments = &<b>mut</b> borrow_global_mut&lt;<a href="AutoPay.md#0x1_AutoPay2_Data">Data</a>&gt;(*account_addr).payments;
    <b>let</b> payments_len = <a href="Vector.md#0x1_Vector_length">Vector::length</a>&lt;<a href="AutoPay.md#0x1_AutoPay2_Payment">Payment</a>&gt;(payments);
    <b>let</b> payments_idx = 0;
    <b>while</b> (payments_idx &lt; payments_len) {
      <b>let</b> delete_payment = <b>false</b>;
      {
        <b>let</b> payment = <a href="Vector.md#0x1_Vector_borrow_mut">Vector::borrow_mut</a>&lt;<a href="AutoPay.md#0x1_AutoPay2_Payment">Payment</a>&gt;(payments, payments_idx);
        // If payment end epoch is greater, it's not an active payment anymore, so delete it, does not <b>apply</b> <b>to</b> fixed once payment (it is deleted once it is sent)
        <b>if</b> (payment.end_epoch &gt;= epoch || payment.in_type == <a href="AutoPay.md#0x1_AutoPay2_FIXED_ONCE">FIXED_ONCE</a>) {
          // A payment will happen now
          // Obtain the amount <b>to</b> pay
          // IMPORTANT there are two digits for scaling representation.

          // an autopay instruction of 12.34% is scaled by two orders, and represented in <a href="AutoPay.md#0x234567_AutoPay">AutoPay</a> <b>as</b> `1234`.
          <b>let</b> amount = <b>if</b> (payment.in_type == <a href="AutoPay.md#0x1_AutoPay2_PERCENT_OF_BALANCE">PERCENT_OF_BALANCE</a>) {
            <a href="FixedPoint32.md#0x1_FixedPoint32_multiply_u64">FixedPoint32::multiply_u64</a>(account_bal , <a href="FixedPoint32.md#0x1_FixedPoint32_create_from_rational">FixedPoint32::create_from_rational</a>(payment.amt, 10000))
          } <b>else</b> <b>if</b> (payment.in_type == <a href="AutoPay.md#0x1_AutoPay2_PERCENT_OF_CHANGE">PERCENT_OF_CHANGE</a>) {
            <b>if</b> (account_bal &gt; payment.prev_bal) {
              <a href="FixedPoint32.md#0x1_FixedPoint32_multiply_u64">FixedPoint32::multiply_u64</a>(account_bal - payment.prev_bal, <a href="FixedPoint32.md#0x1_FixedPoint32_create_from_rational">FixedPoint32::create_from_rational</a>(payment.amt, 10000))
            } <b>else</b> {
              // <b>if</b> account balance hasn't gone up, no value is transferred
              0
            }
          } <b>else</b> {
            // in remaining cases, payment is simple amaount given, not a percentage
            payment.amt
          };

          // check payees are community wallets
          <b>let</b> list = <a href="Wallet.md#0x1_Wallet_get_comm_list">Wallet::get_comm_list</a>();
          <b>if</b> (<a href="Vector.md#0x1_Vector_contains">Vector::contains</a>&lt;address&gt;(&list, &payment.payee)) {
            <b>if</b> (amount != 0 && amount &lt;= account_bal) {
              <b>if</b> (borrow_global&lt;<a href="AutoPay.md#0x1_AutoPay2_AccountLimitsEnable">AccountLimitsEnable</a>&gt;(<a href="Signer.md#0x1_Signer_address_of">Signer::address_of</a>(vm)).enabled) {
                <a href="LibraAccount.md#0x1_LibraAccount_vm_make_payment">LibraAccount::vm_make_payment</a>&lt;<a href="GAS.md#0x1_GAS">GAS</a>&gt;(*account_addr, payment.payee, amount, x"", x"", vm);
              } <b>else</b> {
                <a href="LibraAccount.md#0x1_LibraAccount_vm_make_payment_no_limit">LibraAccount::vm_make_payment_no_limit</a>&lt;<a href="GAS.md#0x1_GAS">GAS</a>&gt;(*account_addr, payment.payee, amount, x"", x"", vm);
              };
            };
          };


          // <b>update</b> previous balance for next calculation
          payment.prev_bal = <a href="LibraAccount.md#0x1_LibraAccount_balance">LibraAccount::balance</a>&lt;<a href="GAS.md#0x1_GAS">GAS</a>&gt;(*account_addr);

          // <b>if</b> it's a one shot payment, delete it once it has done its job
          <b>if</b> (payment.in_type == <a href="AutoPay.md#0x1_AutoPay2_FIXED_ONCE">FIXED_ONCE</a>) {
            delete_payment = <b>true</b>;
          }

        };
        // <b>if</b> the payment has reached its last epoch, delete it
        <b>if</b> (payment.end_epoch &lt;= epoch) {
          delete_payment = <b>true</b>;
        };
      };
      <b>if</b> (delete_payment == <b>true</b>) {
        <a href="Vector.md#0x1_Vector_remove">Vector::remove</a>&lt;<a href="AutoPay.md#0x1_AutoPay2_Payment">Payment</a>&gt;(payments, payments_idx);
        payments_len = payments_len - 1;
      }
      <b>else</b> {
        payments_idx = payments_idx + 1;
      };
    };
    account_idx = account_idx + 1;
  };
}
</code></pre>



</details>

<a name="0x1_AutoPay2_enable_autopay"></a>

## Function `enable_autopay`



<pre><code><b>public</b> <b>fun</b> <a href="AutoPay.md#0x1_AutoPay2_enable_autopay">enable_autopay</a>(acc: &signer)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="AutoPay.md#0x1_AutoPay2_enable_autopay">enable_autopay</a>(acc: &signer) <b>acquires</b> <a href="AutoPay.md#0x1_AutoPay2_AccountList">AccountList</a>{
  <b>let</b> addr = <a href="Signer.md#0x1_Signer_address_of">Signer::address_of</a>(acc);
  // append <b>to</b> account list in system state 0x0
  <b>let</b> accounts = &<b>mut</b> borrow_global_mut&lt;<a href="AutoPay.md#0x1_AutoPay2_AccountList">AccountList</a>&gt;(<a href="CoreAddresses.md#0x1_CoreAddresses_LIBRA_ROOT_ADDRESS">CoreAddresses::LIBRA_ROOT_ADDRESS</a>()).accounts;
  <b>if</b> (!<a href="Vector.md#0x1_Vector_contains">Vector::contains</a>&lt;address&gt;(accounts, &addr)) {
    <a href="Vector.md#0x1_Vector_push_back">Vector::push_back</a>&lt;address&gt;(accounts, addr);
    // Initialize the instructions <a href="AutoPay.md#0x1_AutoPay2_Data">Data</a> on user account state
    move_to&lt;<a href="AutoPay.md#0x1_AutoPay2_Data">Data</a>&gt;(acc, <a href="AutoPay.md#0x1_AutoPay2_Data">Data</a> { payments: <a href="Vector.md#0x1_Vector_empty">Vector::empty</a>&lt;<a href="AutoPay.md#0x1_AutoPay2_Payment">Payment</a>&gt;()});
  };

  // Initialize Escrow data
  <a href="LibraAccount.md#0x1_LibraAccount_initialize_escrow">LibraAccount::initialize_escrow</a>&lt;<a href="GAS.md#0x1_GAS">GAS</a>&gt;(acc);
}
</code></pre>



</details>

<a name="0x1_AutoPay2_disable_autopay"></a>

## Function `disable_autopay`



<pre><code><b>public</b> <b>fun</b> <a href="AutoPay.md#0x1_AutoPay2_disable_autopay">disable_autopay</a>(acc: &signer)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="AutoPay.md#0x1_AutoPay2_disable_autopay">disable_autopay</a>(acc: &signer) <b>acquires</b> <a href="AutoPay.md#0x1_AutoPay2_AccountList">AccountList</a>, <a href="AutoPay.md#0x1_AutoPay2_Data">Data</a> {

  <b>let</b> addr = <a href="Signer.md#0x1_Signer_address_of">Signer::address_of</a>(acc);

  // We destroy the data <b>resource</b> for sender
  <b>let</b> sender_data = move_from&lt;<a href="AutoPay.md#0x1_AutoPay2_Data">Data</a>&gt;(addr);
  <b>let</b> <a href="AutoPay.md#0x1_AutoPay2_Data">Data</a> { payments: _ } = sender_data;

  // pop that account from <a href="AutoPay.md#0x1_AutoPay2_AccountList">AccountList</a>
  <b>let</b> accounts = &<b>mut</b> borrow_global_mut&lt;<a href="AutoPay.md#0x1_AutoPay2_AccountList">AccountList</a>&gt;(<a href="CoreAddresses.md#0x1_CoreAddresses_LIBRA_ROOT_ADDRESS">CoreAddresses::LIBRA_ROOT_ADDRESS</a>()).accounts;
  <b>let</b> (status, index) = <a href="Vector.md#0x1_Vector_index_of">Vector::index_of</a>&lt;address&gt;(accounts, &addr);
  <b>if</b> (status) {
    <a href="Vector.md#0x1_Vector_remove">Vector::remove</a>&lt;address&gt;(accounts, index);
  }
}
</code></pre>



</details>

<a name="0x1_AutoPay2_create_instruction"></a>

## Function `create_instruction`



<pre><code><b>public</b> <b>fun</b> <a href="AutoPay.md#0x1_AutoPay2_create_instruction">create_instruction</a>(sender: &signer, uid: u64, in_type: u8, payee: address, end_epoch: u64, amt: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="AutoPay.md#0x1_AutoPay2_create_instruction">create_instruction</a>(
  sender: &signer,
  uid: u64,
  in_type: u8,
  payee: address,
  end_epoch: u64,
  amt: u64
) <b>acquires</b> <a href="AutoPay.md#0x1_AutoPay2_Data">Data</a> {
  <b>let</b> addr = <a href="Signer.md#0x1_Signer_address_of">Signer::address_of</a>(sender);
  // Confirm that no payment <b>exists</b> <b>with</b> the same uid
  <b>let</b> index = <a href="AutoPay.md#0x1_AutoPay2_find">find</a>(addr, uid);
  <b>if</b> (<a href="Option.md#0x1_Option_is_some">Option::is_some</a>&lt;u64&gt;(&index)) {
    // This is the case <b>where</b> the payment uid already <b>exists</b> in the vector
    <b>assert</b>(<b>false</b>, 010104011021);
  };
  <b>let</b> payments = &<b>mut</b> borrow_global_mut&lt;<a href="AutoPay.md#0x1_AutoPay2_Data">Data</a>&gt;(addr).payments;

  <b>assert</b>(<a href="Vector.md#0x1_Vector_length">Vector::length</a>&lt;<a href="AutoPay.md#0x1_AutoPay2_Payment">Payment</a>&gt;(payments) &lt; <a href="AutoPay.md#0x1_AutoPay2_MAX_NUMBER_OF_INSTRUCTIONS">MAX_NUMBER_OF_INSTRUCTIONS</a>, <a href="Errors.md#0x1_Errors_limit_exceeded">Errors::limit_exceeded</a>(<a href="AutoPay.md#0x1_AutoPay2_TOO_MANY_INSTRUCTIONS">TOO_MANY_INSTRUCTIONS</a>));

  <b>assert</b>(<a href="LibraAccount.md#0x1_LibraAccount_exists_at">LibraAccount::exists_at</a>(payee), <a href="Errors.md#0x1_Errors_not_published">Errors::not_published</a>(<a href="AutoPay.md#0x1_AutoPay2_EPAYEE_DOES_NOT_EXIST">EPAYEE_DOES_NOT_EXIST</a>));

  <b>assert</b>(in_type &lt;= <a href="AutoPay.md#0x1_AutoPay2_MAX_TYPE">MAX_TYPE</a>, <a href="Errors.md#0x1_Errors_invalid_argument">Errors::invalid_argument</a>(<a href="AutoPay.md#0x1_AutoPay2_INVALID_PAYMENT_TYPE">INVALID_PAYMENT_TYPE</a>));

  <b>let</b> account_bal = <a href="LibraAccount.md#0x1_LibraAccount_balance">LibraAccount::balance</a>&lt;<a href="GAS.md#0x1_GAS">GAS</a>&gt;(addr);

  <a href="Vector.md#0x1_Vector_push_back">Vector::push_back</a>&lt;<a href="AutoPay.md#0x1_AutoPay2_Payment">Payment</a>&gt;(payments, <a href="AutoPay.md#0x1_AutoPay2_Payment">Payment</a> {
    // name: name,
    uid: uid,
    in_type: in_type,
    payee: payee,
    end_epoch: end_epoch,
    prev_bal: account_bal,
    amt: amt,
  });
}
</code></pre>



</details>

<a name="0x1_AutoPay2_delete_instruction"></a>

## Function `delete_instruction`



<pre><code><b>public</b> <b>fun</b> <a href="AutoPay.md#0x1_AutoPay2_delete_instruction">delete_instruction</a>(account: &signer, uid: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="AutoPay.md#0x1_AutoPay2_delete_instruction">delete_instruction</a>(account: &signer, uid: u64) <b>acquires</b> <a href="AutoPay.md#0x1_AutoPay2_Data">Data</a> {
  <b>let</b> addr = <a href="Signer.md#0x1_Signer_address_of">Signer::address_of</a>(account);
  <b>let</b> index = <a href="AutoPay.md#0x1_AutoPay2_find">find</a>(addr, uid);
  <b>if</b> (<a href="Option.md#0x1_Option_is_none">Option::is_none</a>&lt;u64&gt;(&index)) {
    // Case when the payment <b>to</b> be deleted doesn't actually exist
    <b>assert</b>(<b>false</b>, <a href="Errors.md#0x1_Errors_invalid_argument">Errors::invalid_argument</a>(<a href="AutoPay.md#0x1_AutoPay2_AUTOPAY_ID_EXISTS">AUTOPAY_ID_EXISTS</a>));
  };
  <b>let</b> payments = &<b>mut</b> borrow_global_mut&lt;<a href="AutoPay.md#0x1_AutoPay2_Data">Data</a>&gt;(addr).payments;
  <a href="Vector.md#0x1_Vector_remove">Vector::remove</a>&lt;<a href="AutoPay.md#0x1_AutoPay2_Payment">Payment</a>&gt;(payments, <a href="Option.md#0x1_Option_extract">Option::extract</a>&lt;u64&gt;(&<b>mut</b> index));
}
</code></pre>



</details>

<a name="0x1_AutoPay2_is_enabled"></a>

## Function `is_enabled`



<pre><code><b>public</b> <b>fun</b> <a href="AutoPay.md#0x1_AutoPay2_is_enabled">is_enabled</a>(account: address): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="AutoPay.md#0x1_AutoPay2_is_enabled">is_enabled</a>(account: address): bool <b>acquires</b> <a href="AutoPay.md#0x1_AutoPay2_AccountList">AccountList</a> {
  <b>let</b> accounts = &<b>mut</b> borrow_global_mut&lt;<a href="AutoPay.md#0x1_AutoPay2_AccountList">AccountList</a>&gt;(<a href="CoreAddresses.md#0x1_CoreAddresses_LIBRA_ROOT_ADDRESS">CoreAddresses::LIBRA_ROOT_ADDRESS</a>()).accounts;
  <b>if</b> (<a href="Vector.md#0x1_Vector_contains">Vector::contains</a>&lt;address&gt;(accounts, &account)) {
    <b>return</b> <b>true</b>
  };
  <b>false</b>
}
</code></pre>



</details>

<a name="0x1_AutoPay2_query_instruction"></a>

## Function `query_instruction`



<pre><code><b>public</b> <b>fun</b> <a href="AutoPay.md#0x1_AutoPay2_query_instruction">query_instruction</a>(account: address, uid: u64): (u8, address, u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="AutoPay.md#0x1_AutoPay2_query_instruction">query_instruction</a>(account: address, uid: u64): (u8, address, u64, u64) <b>acquires</b> <a href="AutoPay.md#0x1_AutoPay2_Data">Data</a> {
  // TODO: This can be made faster <b>if</b> <a href="AutoPay.md#0x1_AutoPay2_Data">Data</a>.payments is stored <b>as</b> a BST sorted by
  <b>let</b> index = <a href="AutoPay.md#0x1_AutoPay2_find">find</a>(account, uid);
  <b>if</b> (<a href="Option.md#0x1_Option_is_none">Option::is_none</a>&lt;u64&gt;(&index)) {
    // Case <b>where</b> payment is not found
    <b>return</b> (0, 0x0, 0, 0)
  } <b>else</b> {
    <b>let</b> payments = &borrow_global_mut&lt;<a href="AutoPay.md#0x1_AutoPay2_Data">Data</a>&gt;(account).payments;
    <b>let</b> payment = <a href="Vector.md#0x1_Vector_borrow">Vector::borrow</a>(payments, <a href="Option.md#0x1_Option_extract">Option::extract</a>&lt;u64&gt;(&<b>mut</b> index));
    <b>return</b> (payment.in_type, payment.payee, payment.end_epoch, payment.amt)
  }
}
</code></pre>



</details>

<a name="0x1_AutoPay2_find"></a>

## Function `find`



<pre><code><b>fun</b> <a href="AutoPay.md#0x1_AutoPay2_find">find</a>(account: address, uid: u64): <a href="Option.md#0x1_Option_Option">Option::Option</a>&lt;u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="AutoPay.md#0x1_AutoPay2_find">find</a>(account: address, uid: u64): <a href="Option.md#0x1_Option">Option</a>&lt;u64&gt; <b>acquires</b> <a href="AutoPay.md#0x1_AutoPay2_Data">Data</a> {
  <b>let</b> payments = &borrow_global&lt;<a href="AutoPay.md#0x1_AutoPay2_Data">Data</a>&gt;(account).payments;
  <b>let</b> len = <a href="Vector.md#0x1_Vector_length">Vector::length</a>(payments);
  <b>let</b> i = 0;
  <b>while</b> (i &lt; len) {
    <b>let</b> payment = <a href="Vector.md#0x1_Vector_borrow">Vector::borrow</a>&lt;<a href="AutoPay.md#0x1_AutoPay2_Payment">Payment</a>&gt;(payments, i);
    <b>if</b> (payment.uid == uid) {
      <b>return</b> <a href="Option.md#0x1_Option_some">Option::some</a>&lt;u64&gt;(i)
    };
    i = i + 1;
  };
  <a href="Option.md#0x1_Option_none">Option::none</a>&lt;u64&gt;()
}
</code></pre>



</details>


[//]: # ("File containing references which can be used from documentation")
[ACCESS_CONTROL]: https://github.com/libra/lip/blob/master/lips/lip-2.md
[ROLE]: https://github.com/libra/lip/blob/master/lips/lip-2.md#roles
[PERMISSION]: https://github.com/libra/lip/blob/master/lips/lip-2.md#permissions
