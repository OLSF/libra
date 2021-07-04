
# Txs App

#### Helpers  

```
// Show help for txs flags
cargo r -p txs foo  // todo: this is a workaround, what is the correct way?

// Show help for a subcommand
cargo r -p txs -- help create-account
```

## Implemented Commands:

```
txs ... create-account -a /opt/account.json
txs ... oracle-upgrade -u /libra/fixtures/upgrade_payload/foo_stdlib.mv
```

## Txs Logic & Usage

``` Rust
/// All the parameters needed for a client transaction.
pub struct TxParams {
    /// User's 0L authkey used in mining.
    pub auth_key: AuthenticationKey,
    /// User's 0L account used in mining
    pub address: AccountAddress,
    /// Url
    pub url: Url,
    /// waypoint
    pub waypoint: Waypoint,
    /// KeyPair
    pub keypair: KeyPair<Ed25519PrivateKey, Ed25519PublicKey>,
    /// User's Maximum gas_units willing to run. Different than coin. 
    pub max_gas_unit_for_tx: u64,
    /// User's GAS Coin price to submit transaction.
    pub coin_price_per_unit: u64,
    /// User's transaction timeout.
    pub user_tx_timeout: u64, // for compatibility with UTC's timestamp.
}
```
Ref: https://github.com/OLSF/libra/blob/tx-sender/txs/src/submit_tx.rs#L26


#### Get TxParams from Local swarm
```
cargo r -p txs -- -s ~/libra/swarm_temp create-account -a ~/account.json  
```
- If we have `-s` flag, get `url` and `waypoint` tx params from local swarm and all others from hardcoded in source code.  


#### Get TxParams from `txs.toml` or `AppConfig::Profile::default()`

```
cargo r -p txs -- create-account -a ~/account.json   
```
- Get all tx params from `txs.toml` (e.g. ~/.0L/txs.toml) or hardcoded `AppConfig::Profile::default()`, except `auth_key`, `address`, `keypair` which are derived from mnemonic which is entered by user/std-in.

#### Get dynamic waypoint from `key_store.json`

- If there is `key_store.json`, get and override `waypoint` 
- e.g. `~/.0L/key_store.json`  

#### Get some TxParams from Command line
```
// In this example, tx params taken from swarm but `url` and `waypoint` are overriden 
cargo r -p txs -- -s ~/libra/swarm_temp -u "http://localhost:39513/" -w "0:5e65aa4ccfba16ed167f87f7dff8846b7eda315af90f88ac15d889758a744dda" create-account -a ~/account.json 
```

- Get and override `url` and/or `waypoint` from command line - as a last step 


## Notes

#### Example `txs.toml`

```
[workspace]
node_home = "/home/user/.0L/"

[profile]
url = "http://localhost:38211"        
waypoint = "0:0d3c7ee0ff2e3f3f256cedb63125215e85f76ae5246022dd2ffd37a17bd6498e"
max_gas_unit_for_tx = 1_000_000
coin_price_per_unit = 1
user_tx_timeout = 6_000
```

