// use libra_crypto::HashValue;
// use libra_types::{
//     account_state_blob::AccountStateBlob,
//     transaction::PRE_GENESIS_VERSION
// };
// use backup_cli::backup_types::state_snapshot::restore::StateSnapshotRestoreController;
// use backup_cli::storage::FileHandle;
// use serde_json::Value;


#[cfg(test)]
mod tests {

    #[test]
    fn check() -> Result<(), String> {
        let blobs = read_from_json("../../../fixtures/state-snapshot/194/state_ver_74694920.0889/state.manifest".to_string());
        println!("{:?}", blobs);
        Ok(())
    }

    pub fn read_from_json(file_path: String) -> serde_json::Value{
        let json: serde_json::Value =
            serde_json::from_str(&file_path).expect("JSON was not well-formatted");
        return json;
    }
    
}



