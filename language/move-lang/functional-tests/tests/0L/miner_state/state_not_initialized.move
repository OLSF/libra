// Scenario: Alice is NOT a validator, and has not mined before. she tries to submit block_0.json the genesis proof without any MinerState being initialized. The tx should abort.

//! account: dummy-prevents-genesis-reload, 100000 ,0, validator

// Alice Submit VDF Proof
//! new-transaction
//! account: alice, 10000000GAS
//! sender: alice
script {
use 0x1::MinerState;
// use 0x1::Debug;
use 0x1::Transaction;

fun main(sender: &signer) {
    let difficulty = 100;
    let challenge = x"1796824cdcc3ab205c25f260e15dc6705942d356f114089d4a46f2f3b0b15b52000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000304c20746573746e65746400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000050726f74657374732072616765206163726f737320746865206e6174696f6e";
    // Generate solutions with cd ./verfiable-delay/ cargo run -- -l=4096 aa 100
    // the -l=4096 is important because this is the security paramater of 0L miner.
    let solution = x"004102e5a40aa610d88964f2fb650511370811440577cd4cd643321f494accd05e10f5183cdb155b98f519e24619fd12a06bb12d9377c03998cb2f06a2b88485b2fc33704f632f518a7b821500cf5c6ee4e3748768af0ca25240f02041845e0d74975d82e6679996244d3bb41c0c36b520eea0ad11918ec44bff83a392d2e4fb7bc5c022275fc4ba242fa4c2503756a004ab262aeda9fac49b92521e2d6a96ab3b9d2fc0c3906da8ba3e87f4b249f81e9dc8cc879019bdffa716a878c9c73ea406fc60597b29f3f503cd9d293017a37cdec80b30f8bd95c6f1a67b7e1afe97a1f7fb4eed3a1e56cbf13c7f034c5373849008df4c6dafd92df0de5421b123a5839800087a8004a1a0e78ca4e0aa50760bc77e8dd00526aed9fa272541f1476ae5476bd520c72694ded1de34a672bdca451e8809d15d98910921e28ee313186e77726a428204672f66b873f35463d78570f126329c0541a5c1af1fb18b429be150fd1de9498d18717b09178a41aa15fd6fa8d7b6737d730fe87ef4ab875ec2459f8b97c604fc6f999d2c1df563e550ba247e5d7318319fcb0c22e39a3bc2d8b782d8fad912652fc151a99af0f4a28311050d77d75a1651d08d7a45676238765bab161c15648c4dd601eab92d169702f5a951b332845e87942ea48b453c177c51fa73e64d67544857f69d342ecd0ebf1e73ed57d7de1f780a9f01ceaa95b586f60ba2a300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001";
    let reported_tower_height = 0;

    // return solution
    let proof = MinerState::create_proof_blob(challenge, difficulty, solution);
    MinerState::commit_state(sender, proof);
    let verified_tower_height_after = MinerState::test_helper_get_height({{alice}});
    // Debug::print(&verified_tower_height_after);

    Transaction::assert(verified_tower_height_after == reported_tower_height, 10008001);

}
}
// check: ABORTED


