script {
    use 0x1::Wallet;
    fun set_wallet_type(sender: &signer, type_of: u8) {
      if (type_of == 0) {
        Wallet::set_slow(sender);
      };

      if (type_of == 1) {
          Wallet::set_comm(sender);
      };
    }
}
