address 0x1 {
module Wallet {
    use 0x1::CoreAddresses;
    use 0x1::Vector;
    use 0x1::Signer;
    use 0x1::Errors;
    use 0x1::LibraConfig;
    use 0x1::Option::{Self,Option};
    use 0x1::LibraSystem;
    use 0x1::NodeWeight;

    const ERR_PREFIX: u64 = 023;

    const PROPOSED: u8 = 0;
    const APPROVED: u8 = 1;
    const REJECTED: u8 = 2;

    //////// COMMUNITY WALLETS ////////

    resource struct CommunityWallets {
        list: vector<address>
    }

        // Timed transfer submission
    resource struct CommunityTransfers {
      proposed: vector<TimedTransfer>,
      approved: vector<TimedTransfer>,
      rejected: vector<TimedTransfer>,
      max_uid: u64,

    }
    struct TimedTransfer {
      uid: u64,
      expire_epoch: u64,
      payer: address,
      payee: address,
      value: u64,
      description: vector<u8>,
      veto: Veto,
    }

    struct Veto {
      list: vector<address>,
      count: u64,
      threshold: u64,
    }

    resource struct CommunityFreeze {
        is_frozen: bool,
        consecutive_rejections: u64,
        unfreeze_votes: vector<address>,
    }

    // Utility used at genesis (and on upgrade) to initialize the system state.
    public fun init(vm: &signer) {
        CoreAddresses::assert_libra_root(vm);
        
        if ((!exists<CommunityTransfers>(0x0))) {
          move_to<CommunityTransfers>(vm, CommunityTransfers{
            proposed: Vector::empty<TimedTransfer>(),
            approved: Vector::empty<TimedTransfer>(),
            rejected: Vector::empty<TimedTransfer>(),
            max_uid: 0,
          })
        }; 

      if (!exists<CommunityWallets>(0x0)) {
        move_to<CommunityWallets>(vm, CommunityWallets {
          list: Vector::empty<address>()
        });  
      }
    }

    public fun is_init_comm():bool {
      exists<CommunityTransfers>(0x0)
    }

    public fun set_comm(sig: &signer) acquires CommunityWallets {
      if (exists<CommunityWallets>(0x0)) {
        let addr = Signer::address_of(sig);
        let list = get_comm_list();
        if (!Vector::contains<address>(&list, &addr)) {
            let s = borrow_global_mut<CommunityWallets>(0x0);
            Vector::push_back(&mut s.list, addr);
        };

        move_to<CommunityFreeze>(sig, CommunityFreeze {
          is_frozen: false,
          consecutive_rejections: 0,
          unfreeze_votes: Vector::empty<address>()
        })
      }
    }


    // Utility for vm to remove the CommunityWallet tag from an address
    public fun vm_remove_comm(vm: &signer, addr: address) acquires CommunityWallets {
      CoreAddresses::assert_libra_root(vm);
      if (exists<CommunityWallets>(0x0)) {
        let list = get_comm_list();
        let (yes, i) = Vector::index_of<address>(&list, &addr);
        if (yes) {
          let s = borrow_global_mut<CommunityWallets>(0x0);
          Vector::remove(&mut s.list, i);
        }
      }
    }


  // The community wallet Signer can propose a timed transaction.
  // the timed transaction defaults to occurring in the 3rd following epoch.
  // TODO: Increase this time?
  // the transaction will automatically occur at the epoch boundary, unless a veto vote by the validator set is successful.
  // at that point the transaction leves the proposed queue, and is added the rejected list.
  public fun new_timed_transfer(sender: &signer, payee: address, value: u64, description: vector<u8>): u64 acquires CommunityTransfers, CommunityWallets {
      let sender_addr = Signer::address_of(sender);
      let list = get_comm_list();
        
      assert(
        Vector::contains<address>(&list, &sender_addr), Errors::requires_role(ERR_PREFIX + 001)
      );

      let d = borrow_global_mut<CommunityTransfers>(0x0);
      d.max_uid = d.max_uid + 1;
      
      // add current epoch + 1
      let current_epoch = LibraConfig::get_current_epoch();

      let t = TimedTransfer {
          uid: d.max_uid,
          expire_epoch: current_epoch + 3,
          payer: sender_addr,
          payee: payee,
          value: value,
          description: description,
          veto: Veto {
            list: Vector::empty<address>(),
            count: 0,
            threshold: 0,
          }
      };

      Vector::push_back<TimedTransfer>(&mut d.proposed, t);
      return d.max_uid
    }

  // utlity to query a CommunityWallet transfer wallet.
  // Note: doesn not need to be a public function, except for use in tests.
  public fun find(uid: u64, type_of: u8): (Option<TimedTransfer>, u64) acquires CommunityTransfers {
    let c = borrow_global<CommunityTransfers>(0x0);
    let list = if (type_of == 0) {
      &c.proposed
    } else if (type_of == 1) {
      &c.approved
    } else {
      &c.rejected
    };

    let len = Vector::length(list);
    let i = 0;
    while (i < len) {
      let t = *Vector::borrow<TimedTransfer>(list, i);
      if (t.uid == uid) {
        return (Option::some<TimedTransfer>(t), i)
      };
      i = i + 1;
    };
    (Option::none<TimedTransfer>(), 0)
  }
  
  // A validator casts a vote to veto a proposed/pending transaction by a community wallet.
  // The validator identifies the transaction by a unique id.
  // tallies are computed on the fly, such that if a veto happens, the community which
  // is faster than waiting for epoch boundaries.

  public fun veto(sender: &signer, uid: u64) acquires CommunityTransfers, CommunityFreeze {
    let addr = Signer::address_of(sender);
    assert(
      LibraSystem::is_validator(addr),
      Errors::requires_role(ERR_PREFIX + 001)
    );
    let (opt, i) = find(uid, PROPOSED);
    if (Option::is_some<TimedTransfer>(&opt)) {
      let c = borrow_global_mut<CommunityTransfers>(0x0);
      let t = Vector::borrow_mut<TimedTransfer>(&mut c.proposed, i);
      // add voters address to the veto list
      Vector::push_back<address>(&mut t.veto.list, addr);
      // if not at rejection threshold
      // add latency to the payment, to get further reviews
      t.expire_epoch = t.expire_epoch + 1;

      if (tally_veto(i)) {
        reject(uid)
      }
    };
  }

  // private function. Once vetoed, the CommunityWallet transaction is remove from proposed list.
  fun reject(uid: u64) acquires CommunityTransfers, CommunityFreeze {
    let c = borrow_global_mut<CommunityTransfers>(0x0);
    let list = *&c.proposed;
    let len = Vector::length(&list);
    let i = 0;
    while (i < len) {
      let t = *Vector::borrow<TimedTransfer>(&list, i);
      if (t.uid == uid) {
        Vector::remove<TimedTransfer>(&mut c.proposed, i);
        let f = borrow_global_mut<CommunityFreeze>(*&t.payer);
        f.consecutive_rejections = f.consecutive_rejections + 1;
        Vector::push_back(&mut c.rejected, t);
        
      };

      i = i + 1;
    };
    
  }

  // private function to tally vetos.
  // checks if a voter is in the validator set.
  // tallies everytime called. Only counts votes in the validator set.
  // does not remove an address if not in the validator set, in case the validator returns
  // to the set on the next tally.
  fun tally_veto(index: u64): bool acquires CommunityTransfers {
    let c = borrow_global_mut<CommunityTransfers>(0x0);
    let t = Vector::borrow_mut<TimedTransfer>(&mut c.proposed, index);

    let votes = 0;
    let threshold = calculate_proportional_voting_threshold();
    
    let k = 0;
    let len = Vector::length<address>(&t.veto.list);

    while (k < len) {
      let addr = *Vector::borrow<address>(&t.veto.list, k);
      // ignore votes that are no longer in the validator set,
      // BUT DON'T REMOVE, since they may rejoin the validator set, and shouldn't need to vote again.

      if (LibraSystem::is_validator(addr)) {
        votes = votes + NodeWeight::proof_of_weight(addr)
      };
      k = k + 1;
    };

    t.veto.count = votes;
    t.veto.threshold = threshold;

    return votes > threshold
  }

  // private function to get the total voting power of the validator set, and find the 2/3rds threshold
  fun calculate_proportional_voting_threshold(): u64 {
      let val_set_size = LibraSystem::validator_set_size();
      let i = 0;
      let voting_power = 0;
      while (i < val_set_size) {
        let addr = LibraSystem::get_ith_validator_address(i);        
        voting_power = voting_power + NodeWeight::proof_of_weight(addr);
        i = i + 1;
      };
      let threshold = voting_power * 2 / 3;
      threshold
  }

  // Utility to list CommunityWallet transfers due, by epoch. Anyone can call this.
  // This is used by VM in LibraAccount at epoch boundaries to process the wallet transfers.
  public fun list_tx_by_epoch(epoch: u64): vector<TimedTransfer> acquires CommunityTransfers {
      let c = borrow_global_mut<CommunityTransfers>(0x0);
      // reset approved list
      c.approved = Vector::empty<TimedTransfer>();
      // loop proposed list
      let pending = Vector::empty<TimedTransfer>();
      let len = Vector::length(&c.proposed);
      let i = 0;
      while (i < len) {
        let t = Vector::borrow(&c.proposed, i);
        if (t.expire_epoch == epoch) {
          
          Vector::push_back<TimedTransfer>(&mut pending, *t);
          // TODO: clear the freeze count on community wallet
          // add to approved list
        };
        i = i + 1;
      };
      return pending
    }
    
    public fun maybe_reset_rejection_counter(vm: &signer, wallet: address) acquires CommunityFreeze {
      CoreAddresses::assert_libra_root(vm);
      let f = borrow_global_mut<CommunityFreeze>(wallet);
      f.consecutive_rejections = 0;
    }

    // Private function to freeze a community wallet
    // community wallets get frozen if 3 consecutive attempts to transfer are rejected.
    fun maybe_freeze(wallet: address) acquires CommunityFreeze {
      let f = borrow_global_mut<CommunityFreeze>(wallet);
      if (f.consecutive_rejections > 2) {
        f.is_frozen = true;
      }
    }

    // Unfreezing a wallet requires the same threshold, as rejecting a transaction.
    // validators can vote to unfreeze.
    // unfreezing happens as soon as a vote passes threshold (not at epoch boundary)
    public fun vote_to_unfreeze(val: &signer, wallet: address) acquires CommunityFreeze {
      let f = borrow_global_mut<CommunityFreeze>(wallet);
      let val_addr = Signer::address_of(val);
      Vector::push_back<address>(&mut f.unfreeze_votes, val_addr);
      
      if (tally_unfreeze(wallet)) {
        let f = borrow_global_mut<CommunityFreeze>(wallet);
        f.is_frozen = false;
      }
    }

    // private function to tall the unfreezing of a wallet.
    fun tally_unfreeze(wallet: address): bool acquires CommunityFreeze {
      let f = borrow_global<CommunityFreeze>(wallet);

      let votes = 0;
      let threshold = calculate_proportional_voting_threshold();
      
      let k = 0;
      let len = Vector::length<address>(&f.unfreeze_votes);

      while (k < len) {
        let addr = *Vector::borrow<address>(&f.unfreeze_votes, k);
        // ignore votes that are no longer in the validator set,
        // BUT DON'T REMOVE, since they may rejoin the validator set, and shouldn't need to vote again.

        if (LibraSystem::is_validator(addr)) {
          votes = votes + NodeWeight::proof_of_weight(addr)
        };
        k = k + 1;
      };

      return votes > threshold
    }



    //////// GETTERS ////////
    public fun get_tx_args(t: TimedTransfer): (address, address, u64, vector<u8>) {
      (t.payer, t.payee, t.value, *&t.description)
    }

    public fun get_tx_epoch(uid: u64): u64 acquires CommunityTransfers {
      let (opt, _) = find(uid, PROPOSED);
      if (Option::is_some<TimedTransfer>(&opt)) {
        let t = Option::borrow<TimedTransfer>(&opt);
        return *&t.expire_epoch
      };
      0
    }
    
    public fun transfer_is_proposed(uid: u64): bool acquires  CommunityTransfers {
      let (opt, _) = find(uid, PROPOSED);
      Option::is_some<TimedTransfer>(&opt)
    }

    public fun transfer_is_rejected(uid: u64): bool acquires  CommunityTransfers {
      let (opt, _) = find(uid, REJECTED);
      Option::is_some<TimedTransfer>(&opt)
    }

    // Getter for retrieving the list of community wallets.
    public fun get_comm_list(): vector<address> acquires CommunityWallets{
      if (exists<CommunityWallets>(0x0)) {
        let s = borrow_global<CommunityWallets>(0x0);
        return *&s.list
      } else {
        return Vector::empty<address>()
      }
    }

    // getter to check if is a CommunityWallet
    public fun is_comm(addr: address): bool acquires CommunityWallets{
      let s = borrow_global<CommunityWallets>(0x0);
      Vector::contains<address>(&s.list, &addr)
    }

    // getter to check if wallet is frozen
    // used in LibraAccount before attempting a transfer.
    public fun is_frozen(addr: address): bool acquires CommunityFreeze{
      let f = borrow_global<CommunityFreeze>(addr);
      f.is_frozen
    }


    //////// SLOW WALLETS ////////
    resource struct SlowWallet {
        is_slow: bool
    }

    public fun set_slow(sig: &signer) {
      if (!exists<SlowWallet>(Signer::address_of(sig))) {
        move_to<SlowWallet>(sig, SlowWallet {
          is_slow: true
        });  
      }
    }

    public fun is_slow(addr: address): bool {
      exists<SlowWallet>(addr)
    }
}
}