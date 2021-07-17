//! account: alice, 1000000, 0, validator

// Test audit function val_audit_passing satisfying all conditions
//! new-transaction
//! sender: libraroot
//! execute-as: alice
script {
    use 0x1::Audit;
    // use 0x1::Signer;
    use 0x1::ValidatorConfig;
    use 0x1::AutoPay2;
    use 0x1::MinerState;
    use 0x1::GAS::GAS;
    use 0x1::LibraAccount;
    
    fun main(lr: &signer, alice_account: &signer) {
        assert(ValidatorConfig::is_valid({{alice}}), 7357007003001);
        
        // transfer enough coins to operator
        let oper = ValidatorConfig::get_operator({{alice}});
        LibraAccount::vm_make_payment_no_limit<GAS>(
            {{alice}},
            oper, // has a 0 in balance
            50009,
            x"",
            x"",
            lr
        );               
        assert(LibraAccount::balance<GAS>(oper) == 50009, 7357007003002);
        
        // enable autopay
        assert(!AutoPay2::is_enabled({{alice}}), 7357007003003);
        AutoPay2::enable_autopay(alice_account);
        assert(AutoPay2::is_enabled({{alice}}), 7357007003004);

        assert(MinerState::is_init({{alice}}), 7357007003005);

        // audit must pass
        assert(Audit::val_audit_passing({{alice}}), 7357007003006);
    }
}
// check: EXECUTED
