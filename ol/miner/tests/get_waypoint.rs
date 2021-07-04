//! Functional test for delay module

#![forbid(unsafe_code)]
use std::{fs, io::Write};
use diem_types::waypoint::Waypoint;
#[test]
#[ignore]
fn test_waypoint() {
    let s = ol_types::config::AppCfg::default();

    let path =  s.get_key_store_path();
    fs::create_dir_all(&s.workspace.node_home.clone()).unwrap();

    dbg!(&path);

    let mut file = fs::File::create("/root/.0L/key_store.json").unwrap();
    let json_data = r#"{
        "alice-oper/genesis-waypoint": {
            "data": "GetResponse",
            "last_update": 1604878411,
            "value": "0:08148a7b1ac857caee13337c77e691734899b7cc82f4968b35455fb91c060df5"
        }
    }"#;

    file.write_all(json_data.as_bytes())
            .expect("Could not write json");
    
    let data = s.get_waypoint(None);
    dbg!(data);
    let correct: Waypoint = "0:08148a7b1ac857caee13337c77e691734899b7cc82f4968b35455fb91c060df5".parse().unwrap();
    assert_eq!(data.unwrap(), correct, "json value not equal");
    fs::remove_file(path).unwrap();
}
