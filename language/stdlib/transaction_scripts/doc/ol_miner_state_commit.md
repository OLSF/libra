
<a name="minerstate_commit"></a>

# Script `minerstate_commit`





<pre><code><b>use</b> <a href="../../modules/doc/Globals.md#0x1_Globals">0x1::Globals</a>;
<b>use</b> <a href="../../modules/doc/MinerState.md#0x1_MinerState">0x1::MinerState</a>;
</code></pre>




<pre><code><b>public</b> <b>fun</b> <a href="ol_miner_state_commit.md#minerstate_commit">minerstate_commit</a>(sender: &signer, challenge: vector&lt;u8&gt;, solution: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="ol_miner_state_commit.md#minerstate_commit">minerstate_commit</a>(sender: &signer, challenge: vector&lt;u8&gt;, solution: vector&lt;u8&gt;) {


    <b>let</b> proof = <a href="../../modules/doc/MinerState.md#0x1_MinerState_create_proof_blob">MinerState::create_proof_blob</a>(
      challenge,
      <a href="../../modules/doc/Globals.md#0x1_Globals_get_difficulty">Globals::get_difficulty</a>(),
      solution
    );

    <a href="../../modules/doc/MinerState.md#0x1_MinerState_commit_state">MinerState::commit_state</a>(sender, proof);

}
</code></pre>



</details>


[//]: # ("File containing references which can be used from documentation")
[ACCESS_CONTROL]: https://github.com/libra/lip/blob/master/lips/lip-2.md
[ROLE]: https://github.com/libra/lip/blob/master/lips/lip-2.md#roles
[PERMISSION]: https://github.com/libra/lip/blob/master/lips/lip-2.md#permissions
