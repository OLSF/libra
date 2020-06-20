// The genesis module. This defines the majority of the Move functions that
// are executed, and the order in which they are executed in genesis. Note
// however, that there are certain calls that remain in Rust code in
// genesis (for now).
address 0x0 {
module Genesis {
    use 0x0::Association;
    use 0x0::Coin1;
    use 0x0::Coin2;
    use 0x0::Event;
    use 0x0::LBR;
    use 0x0::GAS;
    use 0x0::Libra;
    use 0x0::LibraAccount;
    use 0x0::LibraBlock;
    use 0x0::LibraConfig;
    use 0x0::LibraSystem;
    use 0x0::LibraTimestamp;
    use 0x0::LibraTransactionTimeout;
    use 0x0::LibraVersion;
    use 0x0::LibraWriteSetManager;
    use 0x0::Signer;
    use 0x0::Stats;
    use 0x0::Testnet;
    use 0x0::TransactionFee;
    use 0x0::Unhosted;

    fun initialize(
        association: &signer,
        config_account: &signer,
        fee_account: &signer,
        tc_account: &signer,
        subsidy_account: &signer,
        tc_addr: address,
        subsidy_addr: address,
        genesis_auth_key: vector<u8>,
    ) {
        let dummy_auth_key_prefix = x"00000000000000000000000000000000";

        // Association root setup
        Association::initialize(association);
        Association::grant_privilege<Libra::AddCurrency>(association, association);

        // On-chain config setup
        Event::publish_generator(config_account);
        LibraConfig::initialize(config_account, association);

        // Currency setup
        Libra::initialize(config_account);

        // Stats module setup
        Stats::initialize(association);

        // Set that this is testnet
        Testnet::initialize(association);

        // Event and currency setup
        Event::publish_generator(association);
        let (coin1_mint_cap, coin1_burn_cap) = Coin1::initialize(association);
        let (coin2_mint_cap, coin2_burn_cap) = Coin2::initialize(association);
        LBR::initialize(association);
        GAS::initialize(association);

        LibraAccount::initialize(association);
        Unhosted::publish_global_limits_definition(association);
        LibraAccount::create_genesis_account<GAS::T>(
            Signer::address_of(association),
            copy dummy_auth_key_prefix,
        );
        Libra::grant_mint_capability_to_association<Coin1::T>(association);
        Libra::grant_mint_capability_to_association<Coin2::T>(association);
        Libra::grant_mint_capability_to_association<GAS::T>(association);

        // Register transaction fee accounts
        LibraAccount::create_testnet_account<GAS::T>(0xFEE, copy dummy_auth_key_prefix);
        TransactionFee::add_txn_fee_currency(fee_account, &coin1_burn_cap);
        TransactionFee::add_txn_fee_currency(fee_account, &coin2_burn_cap);
        TransactionFee::initialize(tc_account, fee_account);
        
        // Create the treasury compliance account
        LibraAccount::create_treasury_compliance_account<GAS::T>(
            association,
            tc_addr,
            copy dummy_auth_key_prefix,
            coin1_mint_cap,
            coin1_burn_cap,
            coin2_mint_cap,
            coin2_burn_cap,
        );

        LibraAccount::create_subsidy_account<GAS::T>(
            association,
            subsidy_addr,
            copy dummy_auth_key_prefix
        );

        // Create the config account
        LibraAccount::create_genesis_account<GAS::T>(
            LibraConfig::default_config_address(),
            dummy_auth_key_prefix
        );

        LibraTransactionTimeout::initialize(association);
        LibraSystem::initialize_validator_set(config_account);
        LibraVersion::initialize(config_account);

        LibraBlock::initialize_block_metadata(association);
        LibraWriteSetManager::initialize(association);
        LibraTimestamp::initialize(association);
        LibraAccount::rotate_authentication_key(association, copy genesis_auth_key);
        LibraAccount::rotate_authentication_key(config_account, copy genesis_auth_key);
        LibraAccount::rotate_authentication_key(fee_account, copy genesis_auth_key);
        LibraAccount::rotate_authentication_key(tc_account, copy genesis_auth_key);
        LibraAccount::rotate_authentication_key(subsidy_account, copy genesis_auth_key);
    }

}
}
