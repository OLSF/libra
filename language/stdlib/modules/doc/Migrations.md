
<a name="0x1_MigrateWallets"></a>

# Module `0x1::MigrateWallets`

Module providing debug functionality.


-  [Constants](#@Constants_0)
-  [Function `migrate_community_wallets`](#0x1_MigrateWallets_migrate_community_wallets)


<pre><code><b>use</b> <a href="AutoPay.md#0x1_AutoPay2">0x1::AutoPay2</a>;
<b>use</b> <a href="LibraAccount.md#0x1_LibraAccount">0x1::LibraAccount</a>;
<b>use</b> <a href="Migrations.md#0x1_Migrations">0x1::Migrations</a>;
<b>use</b> <a href="Vector.md#0x1_Vector">0x1::Vector</a>;
</code></pre>



<a name="@Constants_0"></a>

## Constants


<a name="0x1_MigrateWallets_UID"></a>



<pre><code><b>const</b> <a href="Migrations.md#0x1_MigrateWallets_UID">UID</a>: u64 = 10;
</code></pre>



<a name="0x1_MigrateWallets_migrate_community_wallets"></a>

## Function `migrate_community_wallets`



<pre><code><b>public</b> <b>fun</b> <a href="Migrations.md#0x1_MigrateWallets_migrate_community_wallets">migrate_community_wallets</a>(vm: &signer)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="Migrations.md#0x1_MigrateWallets_migrate_community_wallets">migrate_community_wallets</a>(vm: &signer) {
  // find autopay wallets
  <b>let</b> vec_addr = <a href="AutoPay.md#0x1_AutoPay2_get_all_payees">AutoPay2::get_all_payees</a>();
  // print(&vec_addr);
  // tag <b>as</b>
  <b>let</b> len = <a href="Vector.md#0x1_Vector_length">Vector::length</a>&lt;address&gt;(&vec_addr);
  <b>let</b> i = 0;
  <b>while</b> (i &lt; len) {
    <b>let</b> addr = *<a href="Vector.md#0x1_Vector_borrow">Vector::borrow</a>&lt;address&gt;(&vec_addr, i);
    <a href="LibraAccount.md#0x1_LibraAccount_vm_init_community_wallet">LibraAccount::vm_init_community_wallet</a>(vm, addr);
    i = i + 1;
  };
  <a href="Migrations.md#0x1_Migrations_push">Migrations::push</a>(<a href="Migrations.md#0x1_MigrateWallets_UID">UID</a>, b"<a href="Migrations.md#0x1_MigrateWallets">MigrateWallets</a>");
}
</code></pre>



</details>


[//]: # ("File containing references which can be used from documentation")
[ACCESS_CONTROL]: https://github.com/libra/lip/blob/master/lips/lip-2.md
[ROLE]: https://github.com/libra/lip/blob/master/lips/lip-2.md#roles
[PERMISSION]: https://github.com/libra/lip/blob/master/lips/lip-2.md#permissions
