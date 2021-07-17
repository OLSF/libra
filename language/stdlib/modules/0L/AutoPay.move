address 0x1 {
  // renamed to preventhalting from state corruption
  module AutoPay2 {
///////////////////////////////////////////////////////////////////////////
  // 0L Module
  // Auto Pay - 
  // File Prefix for errors: 0100
  ///////////////////////////////////////////////////////////////////////////
    use 0x1::Vector;
    use 0x1::Option::{Self,Option};
    use 0x1::Signer;
    use 0x1::LibraAccount;
    use 0x1::GAS::GAS;
    use 0x1::FixedPoint32;
    use 0x1::CoreAddresses;
    use 0x1::LibraConfig;
    use 0x1::Errors;
    use 0x1::Wallet;
    // use 0x1::Debug::print;

    /// Attempted to send funds to an account that does not exist
    /// Maximum value for the Payment type selection
    const MAX_TYPE: u8 = 3;

    // types of Payments
    /// send percent of balance at end of epoch payment type
    const PERCENT_OF_BALANCE: u8 = 0;
    /// send percent of the change in balance since the last tick payment type
    const PERCENT_OF_CHANGE: u8 = 1;
    /// send a certain amount each tick until end_epoch is reached payment type
    const FIXED_RECURRING: u8 = 2;
    /// send a certain amount once at the next tick payment type
    const FIXED_ONCE: u8 = 3;

    const MAX_NUMBER_OF_INSTRUCTIONS: u64 = 30;

    const EPAYEE_DOES_NOT_EXIST: u64 = 010017;
    /// The account does not have autopay enabled.
    const EAUTOPAY_NOT_ENABLED: u64 = 010018;
    /// Attempting to re-use autopay id
    const AUTOPAY_ID_EXISTS: u64 = 010019;
    /// Invalid payment type given
    const INVALID_PAYMENT_TYPE: u64 = 010020;
    /// Attempt to add instruction when too many already exist
    const TOO_MANY_INSTRUCTIONS: u64 = 010021;

    resource struct Tick {
      triggered: bool,
    }

    resource struct AccountLimitsEnable {
      enabled: bool,
    }

    // List of payments. Each account will own their own copy of this struct
    resource struct Data {
      payments: vector<Payment>,
    }

    // One copy of this struct will be created. It will be stored in 0x0.
    // It keeps track of all accounts that have autopay enabled and updates the 
    // list as accounts change their Status structs

    // It also keeps track of the current epoch for efficiency (to prevent repeated
    // queries to LibraBlock)
    resource struct AccountList {
      accounts: vector<address>,
      current_epoch: u64,
    }

    // This is the structure of each Payment struct which represents one automatic
    // payment held by an account
    // Possible types:
    // 0: amt% of current balance until end epoch
    // 1: amt% of inflow until end_epoch 
    // 2: amt gas until end_epoch
    // 3: amt gas, one time payment
    struct Payment {
      // TODO: name should be a string to store a memo
      uid: u64,
      in_type: u8,
      payee: address,
      end_epoch: u64,  // end epoch is inclusive, must just be higher than current epoch for type 3
      prev_bal: u64, //only used for type 1
      amt: u64, //percentage for types 0 & 1, count for 2 & 3
    }

    ///////////////////////////////
    // Public functions only OxO //
    //////////////////////////////
    // Function code: 01
    public fun tick(vm: &signer): bool acquires Tick {
      assert(Signer::address_of(vm) == CoreAddresses::LIBRA_ROOT_ADDRESS(), Errors::requires_role(010001));
      if (exists<Tick>(CoreAddresses::LIBRA_ROOT_ADDRESS())) {
        let tick_state = borrow_global_mut<Tick>(Signer::address_of(vm));

        if (!tick_state.triggered) {
          tick_state.triggered = true;
          return true
        };
      } else {
        initialize(vm);
      };
      false
    }

    public fun reconfig_reset_tick(vm: &signer) acquires Tick{
      let tick_state = borrow_global_mut<Tick>(Signer::address_of(vm));
      tick_state.triggered = false;
    }
    // Initialize the entire autopay module by creating an empty AccountList object
    // Called in Genesis
    // Function code 02
    public fun initialize(sender: &signer) {
      assert(Signer::address_of(sender) == CoreAddresses::LIBRA_ROOT_ADDRESS(), Errors::requires_role(010002));
      move_to<AccountList>(sender, AccountList { accounts: Vector::empty<address>(), current_epoch: 0, });
      move_to<Tick>(sender, Tick {triggered: false});
      move_to<AccountLimitsEnable>(sender, AccountLimitsEnable {enabled: false});

      LibraAccount::initialize_escrow_root<GAS>(sender);
    }

    public fun enable_account_limits(sender: &signer) acquires AccountLimitsEnable {
      assert(Signer::address_of(sender) == CoreAddresses::LIBRA_ROOT_ADDRESS(), Errors::requires_role(010002));
      let limits_enable = borrow_global_mut<AccountLimitsEnable>(Signer::address_of(sender));
      limits_enable.enabled = true;
    }

    // helper to get all known destinations users have for autopay
    public fun get_all_payees():vector<address> acquires AccountList, Data {
      let account_list = &borrow_global<AccountList>(CoreAddresses::LIBRA_ROOT_ADDRESS()).accounts;
      let accounts_length = Vector::length<address>(account_list);
      let account_idx = 0;
      let payee_vec = Vector::empty<address>();

// print(&02200);
      while (account_idx < accounts_length) {
        let account_addr = Vector::borrow<address>(account_list, account_idx);
        // Obtain the account balance
        // let account_bal = LibraAccount::balance<GAS>(*account_addr);
        // Go through all payments for this account and pay 
        let payments = &mut borrow_global_mut<Data>(*account_addr).payments;
        let payments_len = Vector::length<Payment>(payments);
        let payments_idx = 0;
        while (payments_idx < payments_len) {
          let payment = Vector::borrow_mut<Payment>(payments, payments_idx);
          if (!Vector::contains<address>(&payee_vec, &payment.payee)) {
            Vector::push_back<address>(&mut payee_vec, payment.payee);
          };
          payments_idx = payments_idx + 1;
        };
        account_idx = account_idx + 1;
      };
      return payee_vec
    }


    // This is the main function for this module. It is called once every epoch
    // by 0x0::LibraBlock in the block_prologue function.
    // This function iterates through all autopay-enabled accounts and processes
    // any payments they have due in the current epoch from their list of payments.
    // Note: payments from epoch n are processed at the epoch_length/2
    // Function code 03
    public fun process_autopay(
      vm: &signer,
    ) acquires AccountList, Data, AccountLimitsEnable {
      // Only account 0x0 should be triggering this autopayment each block
      assert(Signer::address_of(vm) == CoreAddresses::LIBRA_ROOT_ADDRESS(), Errors::requires_role(010003));

      let epoch = LibraConfig::get_current_epoch();
// print(&02100);

      // Go through all accounts in AccountList
      // This is the list of accounts which currently have autopay enabled
      let account_list = &borrow_global<AccountList>(CoreAddresses::LIBRA_ROOT_ADDRESS()).accounts;
      let accounts_length = Vector::length<address>(account_list);
      let account_idx = 0;
// print(&02200);
      while (account_idx < accounts_length) {
// print(&02210);

        let account_addr = Vector::borrow<address>(account_list, account_idx);
        // Obtain the account balance
        let account_bal = LibraAccount::balance<GAS>(*account_addr);
        // Go through all payments for this account and pay 
        let payments = &mut borrow_global_mut<Data>(*account_addr).payments;
        let payments_len = Vector::length<Payment>(payments);
        let payments_idx = 0;
        while (payments_idx < payments_len) {
          let delete_payment = false;
          {
            let payment = Vector::borrow_mut<Payment>(payments, payments_idx);
            // If payment end epoch is greater, it's not an active payment anymore, so delete it, does not apply to fixed once payment (it is deleted once it is sent)
            if (payment.end_epoch >= epoch || payment.in_type == FIXED_ONCE) {
              // A payment will happen now
              // Obtain the amount to pay 
              // IMPORTANT there are two digits for scaling representation.
             
              // an autopay instruction of 12.34% is scaled by two orders, and represented in AutoPay as `1234`.
              let amount = if (payment.in_type == PERCENT_OF_BALANCE) {
                FixedPoint32::multiply_u64(account_bal , FixedPoint32::create_from_rational(payment.amt, 10000))
              } else if (payment.in_type == PERCENT_OF_CHANGE) {
                if (account_bal > payment.prev_bal) {
                  FixedPoint32::multiply_u64(account_bal - payment.prev_bal, FixedPoint32::create_from_rational(payment.amt, 10000))
                } else {
                  // if account balance hasn't gone up, no value is transferred
                  0
                }
              } else {
                // in remaining cases, payment is simple amaount given, not a percentage
                payment.amt
              };

              // check payees are community wallets
              let list = Wallet::get_comm_list();
              if (Vector::contains<address>(&list, &payment.payee)) {
                if (amount != 0 && amount <= account_bal) {
                  if (borrow_global<AccountLimitsEnable>(Signer::address_of(vm)).enabled) {
                    LibraAccount::vm_make_payment<GAS>(*account_addr, payment.payee, amount, x"", x"", vm);
                  } else {
                    LibraAccount::vm_make_payment_no_limit<GAS>(*account_addr, payment.payee, amount, x"", x"", vm);
                  };
                };
              };


              // update previous balance for next calculation
              payment.prev_bal = LibraAccount::balance<GAS>(*account_addr);

              // if it's a one shot payment, delete it once it has done its job
              if (payment.in_type == FIXED_ONCE) {
                delete_payment = true;
              }
              
            };
            // if the payment has reached its last epoch, delete it
            if (payment.end_epoch <= epoch) {
              delete_payment = true;
            };
          };
          if (delete_payment == true) {
            Vector::remove<Payment>(payments, payments_idx);
            payments_len = payments_len - 1;
          }
          else {
            payments_idx = payments_idx + 1;
          };
        };
        account_idx = account_idx + 1;
      };
    }

    ////////////////////////////////////////////
    // Public functions only account owner    //
    // Enable, disable, create/delete instructions //
    ////////////////////////////////////////////

    // Each account needs to initialize autopay on it's account
    // Function code 010102
    public fun enable_autopay(acc: &signer) acquires AccountList{
      let addr = Signer::address_of(acc);
      // append to account list in system state 0x0
      let accounts = &mut borrow_global_mut<AccountList>(CoreAddresses::LIBRA_ROOT_ADDRESS()).accounts;
      if (!Vector::contains<address>(accounts, &addr)) {
        Vector::push_back<address>(accounts, addr);
        // Initialize the instructions Data on user account state 
        move_to<Data>(acc, Data { payments: Vector::empty<Payment>()});
      };

      // Initialize Escrow data
      LibraAccount::initialize_escrow<GAS>(acc);
    }

    // An account can disable autopay on it's account
    // Function code 010103
    public fun disable_autopay(acc: &signer) acquires AccountList, Data {
      
      let addr = Signer::address_of(acc);

      // We destroy the data resource for sender
      let sender_data = move_from<Data>(addr);
      let Data { payments: _ } = sender_data;

      // pop that account from AccountList
      let accounts = &mut borrow_global_mut<AccountList>(CoreAddresses::LIBRA_ROOT_ADDRESS()).accounts;
      let (status, index) = Vector::index_of<address>(accounts, &addr);
      if (status) {
        Vector::remove<address>(accounts, index);
      }      
    }

    // Create a instruction from the sender's account
    // Function code 010104
    public fun create_instruction(
      sender: &signer, 
      uid: u64,
      in_type: u8,
      payee: address,
      end_epoch: u64,
      amt: u64
    ) acquires Data {
      let addr = Signer::address_of(sender);
      // Confirm that no payment exists with the same uid
      let index = find(addr, uid);
      if (Option::is_some<u64>(&index)) {
        // This is the case where the payment uid already exists in the vector
        assert(false, 010104011021);
      };
      let payments = &mut borrow_global_mut<Data>(addr).payments;

      assert(Vector::length<Payment>(payments) < MAX_NUMBER_OF_INSTRUCTIONS, Errors::limit_exceeded(TOO_MANY_INSTRUCTIONS));

      assert(LibraAccount::exists_at(payee), Errors::not_published(EPAYEE_DOES_NOT_EXIST));

      assert(in_type <= MAX_TYPE, Errors::invalid_argument(INVALID_PAYMENT_TYPE));

      let account_bal = LibraAccount::balance<GAS>(addr);

      Vector::push_back<Payment>(payments, Payment {
        // name: name,
        uid: uid,
        in_type: in_type,
        payee: payee,
        end_epoch: end_epoch,
        prev_bal: account_bal,
        amt: amt,
      });
    }

    // Deletes the instruction with uid from the sender's account
    // Function code 010105
    public fun delete_instruction(account: &signer, uid: u64) acquires Data {
      let addr = Signer::address_of(account);
      let index = find(addr, uid);
      if (Option::is_none<u64>(&index)) {
        // Case when the payment to be deleted doesn't actually exist
        assert(false, Errors::invalid_argument(AUTOPAY_ID_EXISTS));
      };
      let payments = &mut borrow_global_mut<Data>(addr).payments;
      Vector::remove<Payment>(payments, Option::extract<u64>(&mut index));
    }

    ///////////////////////////////
    // Public functions to Query //
    // Can be queried by anyone  //
    //////////////////////////////

    // Any account can check to see if any of the accounts has autopay enabled
    // by checking in 0x0's AccountList
    public fun is_enabled(account: address): bool acquires AccountList {
      let accounts = &mut borrow_global_mut<AccountList>(CoreAddresses::LIBRA_ROOT_ADDRESS()).accounts;
      if (Vector::contains<address>(accounts, &account)) {
        return true
      };
      false
    }

    // Returns (sender address,  end_epoch, percentage)
    public fun query_instruction(account: address, uid: u64): (u8, address, u64, u64) acquires Data {
      // TODO: This can be made faster if Data.payments is stored as a BST sorted by 
      let index = find(account, uid);
      if (Option::is_none<u64>(&index)) {
        // Case where payment is not found
        return (0, 0x0, 0, 0)
      } else {
        let payments = &borrow_global_mut<Data>(account).payments;
        let payment = Vector::borrow(payments, Option::extract<u64>(&mut index));
        return (payment.in_type, payment.payee, payment.end_epoch, payment.amt)
      }
    }

    //////////////////////
    // Private function //
    //////////////////////

    // Retuns the index of the desired payment and an immutable reference to it
    // This is used often as a helper function to check existence of payments
    fun find(account: address, uid: u64): Option<u64> acquires Data {
      let payments = &borrow_global<Data>(account).payments;
      let len = Vector::length(payments);
      let i = 0;
      while (i < len) {
        let payment = Vector::borrow<Payment>(payments, i);
        if (payment.uid == uid) {
          return Option::some<u64>(i)
        };
        i = i + 1;
      };
      Option::none<u64>()
    }
  }
}

module AutoPay{
///////////////////////////////////////////////////////////////////////////
  // Old module, simply left to prevent state transition
  // module is purposefully crippled so as to not allow any operation
  // use the above AutoPay2 module instead
  ///////////////////////////////////////////////////////////////////////////

    /// Attempted to send funds to an account that does not exist
    const EPAYEE_DOES_NOT_EXIST: u64 = 17;

    resource struct Tick {
      triggered: bool,
    }
    // List of payments. Each account will own their own copy of this struct
    resource struct Data {
      payments: vector<Payment>,
    }

    // One copy of this struct will be created. It will be stored in 0x0.
    // It keeps track of all accounts that have autopay enabled and updates the 
    // list as accounts change their Status structs

    // It also keeps track of the current epoch for efficiency (to prevent repeated
    // queries to LibraBlock)
    resource struct AccountList {
      accounts: vector<address>,
      current_epoch: u64,
    }

    // This is the structure of each Payment struct which represents one automatic
    // payment held by an account
    struct Payment {
      // TODO: name should be a string to store a memo
      // name: u64,
      uid: u64,
      payee: address,
      end_epoch: u64,  // end epoch is inclusive
      percentage: u64,
    }

}

