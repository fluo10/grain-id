use std::path::PathBuf;
use std::process::Command;

use caretta_id::CarettaId;

fn assert_encode(id: CarettaId) {
    let path = PathBuf::from(std::env!("CARGO_BIN_EXE_caretta-id-cli"));
    let output = Command::new(path)
        .arg("encode")
        .arg(format!("{}", id.as_u64()))
        .output()
        .unwrap()
        .stdout;
    assert_eq!(output, format!("{}\n", &id.to_string()).into_bytes());
}

#[test]
fn nil() {
    assert_encode(CarettaId::NIL);
}

#[test]
fn max() {
    assert_encode(CarettaId::MAX);
}

#[test]
fn random() {
    assert_encode(CarettaId::random());
}
