use caretta_id::CarettaId;
use std::{path::PathBuf, process::Command};

#[test]
fn now_unix() {
    let path = PathBuf::from(std::env!("CARGO_BIN_EXE_caretta-id-cli"));
    let output = String::from_utf8_lossy(
        &Command::new(path)
            .arg("timestamp")
            .arg("--unix")
            .output()
            .unwrap()
            .stdout,
    )
    .trim()
    .to_owned();
    let _ = output.parse::<CarettaId>().unwrap();
}
