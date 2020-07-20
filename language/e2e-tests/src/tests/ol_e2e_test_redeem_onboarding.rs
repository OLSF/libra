use crate::executor::FakeExecutor;
use crate::account::{AccountData, lbr_currency_code, AccountTypeSpecifier};
use crate::redeem_setup:: {redeem_txn_onboarding};

use libra_types::transaction::TransactionStatus;
use libra_types::vm_error::{VMStatus, StatusCode};

use hex;

#[test]
fn submit_proofs_transaction_onboarding() {
    // TODO: This is using the Fake Executor, like all the other e2e tests. Is there a way to use a libra-swarm node?
    let mut executor = FakeExecutor::from_genesis_file();

    //TODO: test this is not an association account.
    let account = Account::new_genesis_account(libra_types::on_chain_config::config_address() );
    let sequence_number = 10u64;
    let sender = AccountData::with_account(
        account,
        1_000_000,
        lbr_currency_code(),
        sequence_number,
        AccountTypeSpecifier::Empty
    );

    executor.add_account_data(&sender);

    let challenge = hex::decode("5ffd9856978b5020be7f72339e41a401000000000000000000000000deadbeef0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004f6c20746573746e65746400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000070726f74657374732072616765206163726f737320416d6572696361");
    let challenge = challenge.unwrap();
    let mut expected_addess: [u8; AccountAddress::LENGTH] = [0u8; AccountAddress::LENGTH];
    expected_addess[AccountAddress::LENGTH - 4] = 222;
    expected_addess[AccountAddress::LENGTH - 3] = 173;
    expected_addess[AccountAddress::LENGTH - 2] = 190;
    expected_addess[AccountAddress::LENGTH - 1] = 239;
    let expected_addess = AccountAddress::new(expected_addess);
    let difficulty = 100u64;
    let auth_key_prefix = hex::decode("5ffd9856978b5020be7f72339e41a401000000000000000000000000deadbeef");
    let _auth_key_prefix = auth_key_prefix.unwrap();
    // Generate solutions with cd ./verfiable-delay/ cargo run -- -l=4096 aa 100
    // the -l=4096 is important because this is the security paramater of 0L miner.
    // let proof_computed = delay::do_delay(challenge, difficulty);
    use crate::account::Account;
    
    
    
    
    use move_core_types::account_address::AccountAddress;
    
    let proof = hex::decode("000c1c21efba97d307e99592448e30170526d377b3d4931e71ac62e894cf727273481fea358fda15ab171a430a9362808912225ca713489e1649b704a211a0c816c3cfc07ddab099fcf11eea7a08eca3308c83a50a506683fb91e63688627feba16e80db72903e5a7d2305deb3150a3bff60b197efee29507f6e661c1c966df94964bebf4f56120f83563df5ec3d9a6e0851f4db74670ab73181b8acbf71153035c53da099333e570b8fdb884b90119d3c2aef97c282f7ee3ea16252adb60038c3dd3be2a3ec12d9aeb7e2a191e98eca92e3dbe4ff386fa77085e54bf4c6729d0f4cde66d50db7f03cef6c271c5df6545c15c562afd37f7dfc66410b2f1370ae620005908a15d190b30da558fd3bfc6417e081087462d897d0823fe2e20bdea89aaf8285e5da9dadccdb1ba6500bbba9e636f9101497610479eedf9c18ccf6c29866c806870731361387d6f810a60294507a24917b9812fa5f5160d53c7bb3adc9d8f81bfdae81ac77e8df85d779538b7be52ef3e2489d0ef0ef49cee05aac26d3e915e9bd0640d8635e812a80277245e6e285d971846d903fd0716097414692c08907d248f27b1de975c70180a454b677fe631f19276a1fb925fea654fef3fddbd8c07b4915442b4216bc39993d41117bff637ccca807818197c5b280efaa6c5a1c74adee08834b37c17799c6fa3123f80b0cfb0c73a4dd745375a23725459a3cad00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001");
    let proof = proof.unwrap();
    
    // run the transaction script
    let output = executor.execute_and_apply(
        // build the transaction script binary.
        redeem_txn_onboarding(
            &sender.account(),
            sequence_number,
            challenge.to_vec(),
            difficulty,
            proof.to_vec(),
            expected_addess)
    );

    assert_eq!(
        output.status(),
        &TransactionStatus::Keep(VMStatus::new(StatusCode::EXECUTED))
    );
}
