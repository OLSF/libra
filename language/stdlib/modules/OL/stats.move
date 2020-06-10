
// This module returns statistics about the network at any given block, or window of blocks. A number of core OL modules depend on Statistics. The relevant statistics in an MVP are "liveness accountability" statistics. From within the VM context the statistics available are those in the BlockMetadata type.

address 0x0 {
  module Stats {
    use 0x0::Vector;
    use 0x0::Transaction;

    // Each Chunk represents one set of contiguous blocks which the validator voted on
    struct Chunk {
      start_block: u64,
      end_block: u64
    }

    // Each Node represents one validator
    struct Node {
      valid: bool,
      validator: address,
      chunks: vector<Chunk>
    }

    // This stores the full history. For POC, it is a vector which stores one
    //  entry for each validator.
    resource struct History {
      val_list: vector<Node>,
    }

    public fun initialize() {
      // Eventually want to ensue that only the Association and make a history block.
      // This should happen in genesis
      // Transaction::assert(Transaction::sender() == 0xA550C18)
      move_to_sender<History>(History{ val_list: Vector::empty() });
    }

    // This should actually return a float as a percentage, but move doesn't support floats
    // as primitive types. For now, it will be returned as an unsigned int and be a confidence level 
    public fun Node_Heuristics(node_addr: address, start_height: u64, 
      end_height: u64): u64 acquires History {
      // Returns the percentage of blocks in the given range that the block voted on

      if (start_height > end_height) return 0;

      let history = borrow_global<History>(Transaction::sender());

      // This is the case where the validator has voted on nothing and does not have a Node
      if (!exists(history, node_addr)) return 0;

      let node = get_node(history, node_addr);
      let chunks = &node.chunks;
      let i = 0;
      let len = Vector::length<Chunk>(chunks);
      let num_voted = 0;

      // Go though all the chunks of the validator and accumulate
      while (i < len) {
        let chunk = Vector::borrow<Chunk>(chunks, i);
        // Check if the chunk has segments in desired region
        if (chunk.end_block > start_height && chunk.start_block < end_height) {
          // Find the lower and upper blockheights within desired region
          let lower = chunk.start_block;
          if (start_height > lower) lower = start_height;

          let upper = chunk.end_block;
          if (end_height < upper) upper = end_height;

          // +1 because bounds are inclusive.
          // E.g. a node which participated in only block 30 would have
          // upper - lower = 0 even though it voted in a block.
          num_voted = num_voted + (upper - lower + 1);
        }
      };
      num_voted 
      // This should be added to get a percentage: num_voted / (end_height - start_height + 1)
    }

    // This function goes through the vector in history and gets the desired node.
    // By the time this runs, we already know that the node exists in the history
    fun get_node(hist: &History, add: address): &Node {
      let i = 0;
      let node_list = &hist.val_list;
      let len = Vector::length<Node>(node_list);
      let node = Vector::borrow<Node>(node_list, i);

      while (i < len) {
        node = Vector::borrow<Node>(node_list, i);
        if (node.validator == add) break;
      };
      node
    }

    // This must be included since does not suppot the Option<T> data type.
    // Since there is no way to return Some<Node> or None, we must do this check separately.
    fun exists(hist: &History, add: address): bool {
      let i = 0;
      let node_list = &hist.val_list;
      let len = Vector::length<Node>(node_list);

      while (i < len) {
        if (Vector::borrow<Node>(node_list, i).validator == add) return true;
      };
      false
    }


    // The actual Stats data structures and workings are a work in progress

    // TODO: Check if libra core "leader reputation" can be wrapped or implemented in our own contract: https://github.com/libra/libra/pull/3026/files
    // pub fun Node_Heuristics(node_address: address type, start_blockheight: u32, end_blockheight: u32)  {
    // fun liveness(node_address){
        // Returns the percentage of blocks have been signed by the node within the range of blocks.

        // Accomplished by querying the data structue
    // }

    // pub fun Network_Heuristics() {
    //  fun signer_density_window(start_blockheight, end_blockheight) {
        // Find the min count of nodes that signed *every* block in the range.
    //  }

    //  fun signer_density_lookback(number_of_blocks: u32 ) {
        // Sugar Needed for subsidy contract. Counts back from current block that accepted transaction. E.g. 1,000 blocks.
    //  }
    // }
  }
}


// Code which might be useful when moving beyond POC stage

//     struct TreeNode{
//       validator: address,
//       start_block: u32,
//       end_block: u32
//     }

//     resource struct Validator_Tree{
//       val_list: vector<u64>,    // not sure the type, so I leave it generic rn. Not the most robust
//       size: u64,              // number of blocks stored in this tree
//       root: TreeNode,
//     }
