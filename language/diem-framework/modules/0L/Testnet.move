/////// 0L /////////

address 0x1 {

module Testnet {
    ///////////////////////////////////////////////////////////////////////////
    // sets an env variable for test constants for devs and ci testing
    // File Prefix for errors: 2002
    ///////////////////////////////////////////////////////////////////////////
    use 0x1::CoreAddresses;
    use 0x1::Errors;
    use 0x1::Signer;

    struct IsTestnet has key { }

    public fun initialize(account: &signer) {
        assert(Signer::address_of(account) == CoreAddresses::DIEM_ROOT_ADDRESS(), Errors::requires_role(200201));
        move_to(account, IsTestnet{})
    }

    public fun is_testnet(): bool {
        exists<IsTestnet>(CoreAddresses::DIEM_ROOT_ADDRESS())
    }

    // only used for testing purposes
    public fun remove_testnet(account: &signer)
    acquires IsTestnet {
        assert(Signer::address_of(account) == CoreAddresses::DIEM_ROOT_ADDRESS(), Errors::requires_role(200202));
        IsTestnet{} = move_from<IsTestnet>(CoreAddresses::DIEM_ROOT_ADDRESS());
    }
}

module StagingNet {
    ///////////////////////////////////////////////////////////////////////////
    // sets an env variable for testing production settings except with shorter epochs and lower vdf difficulty.
    // File Prefix for errors: 1903
    ///////////////////////////////////////////////////////////////////////////
    use 0x1::CoreAddresses;
    use 0x1::Errors;
    use 0x1::Signer;

    struct IsStagingNet has key { }

    public fun initialize(account: &signer) {
        assert(Signer::address_of(account) == CoreAddresses::DIEM_ROOT_ADDRESS(), Errors::requires_role(190301));
        move_to(account, IsStagingNet{})
    }

    public fun is_staging_net(): bool {
        exists<IsStagingNet>(CoreAddresses::DIEM_ROOT_ADDRESS())
    }

}
}