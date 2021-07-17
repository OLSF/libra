
<a name="set_wallet_type"></a>

# Script `set_wallet_type`





<pre><code><b>use</b> <a href="../../modules/doc/Wallet.md#0x1_Wallet">0x1::Wallet</a>;
</code></pre>




<pre><code><b>public</b> <b>fun</b> <a href="ol_wallet_set.md#set_wallet_type">set_wallet_type</a>(sender: &signer, type_of: u8)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="ol_wallet_set.md#set_wallet_type">set_wallet_type</a>(sender: &signer, type_of: u8) {
  <b>if</b> (type_of == 0) {
    <a href="../../modules/doc/Wallet.md#0x1_Wallet_set_slow">Wallet::set_slow</a>(sender);
  };

  <b>if</b> (type_of == 1) {
      <a href="../../modules/doc/Wallet.md#0x1_Wallet_set_comm">Wallet::set_comm</a>(sender);
  };
}
</code></pre>



</details>


[//]: # ("File containing references which can be used from documentation")
[ACCESS_CONTROL]: https://github.com/libra/lip/blob/master/lips/lip-2.md
[ROLE]: https://github.com/libra/lip/blob/master/lips/lip-2.md#roles
[PERMISSION]: https://github.com/libra/lip/blob/master/lips/lip-2.md#permissions
