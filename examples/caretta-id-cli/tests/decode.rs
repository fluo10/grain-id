use std::path::PathBuf;
use std::process::Command;

use caretta_id::{CarettaId};

fn assert_decode(id: CarettaId) {
    let path = PathBuf::from(std::env!("CARGO_BIN_EXE_caretta-id-cli"));
    let output = Command::new(path)
        .arg("decode")
        .arg(&id.to_string())
        .output()
        .unwrap()
        .stdout;
    assert_eq!(output, format!("{}\n", id.as_u64()).into_bytes());
}

#[test]
fn nil() {
    assert_decode(CarettaId::NIL);
}

#[test]
fn max() {
    assert_decode(CarettaId::MAX);
}

#[test]
fn random() {
    assert_decode(CarettaId::random());
}
