//! account: alice, 1, 0, validator
//! account: bob, 1, 0, validator
//! account: carol, 1, 0, validator
//! account: dave, 1, 0, validator
//! account: eve, 1, 0, validator

// All nodes except Eve mined above threshold. 

//! new-transaction
//! sender: alice
script {
    use 0x1::MinerState;

    fun main(sender: signer) {
        // Alice is the only one that can update her mining stats. Hence this first transaction.

        MinerState::test_helper_mock_mining(sender, 5);
        assert(MinerState::get_count_in_epoch({{alice}}) == 5, 7357300101011000);
    }
}
//check: EXECUTED


//! new-transaction
//! sender: bob
script {
    use 0x1::MinerState;

    fun main(sender: signer) {
        // Alice is the only one that can update her mining stats. Hence this first transaction.

        MinerState::test_helper_mock_mining(sender, 4);
        assert(MinerState::get_count_in_epoch({{bob}}) == 4, 7357300102011000);
    }
}
//check: EXECUTED

//! new-transaction
//! sender: carol
script {
    use 0x1::MinerState;

    fun main(sender: signer) {
        // Alice is the only one that can update her mining stats. Hence this first transaction.

        MinerState::test_helper_mock_mining(sender, 3);
        assert(MinerState::get_count_in_epoch({{carol}}) == 3, 7357300103011000);
    }
}
//check: EXECUTED

//! new-transaction
//! sender: dave
script {
    use 0x1::MinerState;

    fun main(sender: signer) {
        // Alice is the only one that can update her mining stats. Hence this first transaction.

        MinerState::test_helper_mock_mining(sender, 2);
        assert(MinerState::get_count_in_epoch({{dave}}) == 2, 7357300104011000);
    }
}
//check: EXECUTED

//! new-transaction
//! sender: diemroot
script {
    use 0x1::Vector;
    use 0x1::NodeWeight;
    use 0x1::ValidatorUniverse;
    use 0x1::MinerState;

    fun main(vm: signer) {

        // Base Case: If validator universe vector length is less than the validator set size limit (N), return vector itself.
        // N equals to the vector length.

        //Check the size of the validator universe.
        let vec =  ValidatorUniverse::get_eligible_validators(vm);
        let len = Vector::length<address>(&vec);
        assert(len == 5, 7357140102011000);

        MinerState::reconfig(vm, &vec);

        // This is the base case: check case of the validator set limit being less than universe size.
        let top_n_is_under = NodeWeight::top_n_accounts(vm, 3);
        assert(Vector::length<address>(&top_n_is_under) == 3, 7357140102021000);

        // Check eve is NOT in that list.
        assert(Vector::contains<address>(&top_n_is_under, &{{eve}}) != true, 7357140102031000);

        // case of querying the full validator universe.
        let top_n_is_equal = NodeWeight::top_n_accounts(vm, len);
        // One of the nodes did not vote, so they will be excluded from list.

        assert(Vector::length<address>(&top_n_is_equal) == len, 7357140102041000);

        // Check eve IS on that list.
        assert(Vector::contains<address>(&top_n_is_equal, &{{eve}}), 7357140102051000);
        
        // case of querying a larger n than the validator universe.
        // Check if we ask for a larger set we also get 
        let top_n_is_over = NodeWeight::top_n_accounts(vm, 9);
        assert(Vector::length<address>(&top_n_is_over) == len, 7357140102061000);

    }
}
// check: EXECUTED