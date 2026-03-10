use caretta_id::CarettaId;
use clap::Args;

/// Encode integer to caretta-id string.
#[derive(Args, Debug)]
pub struct EncodeArgs {
    value: u64,
}

impl EncodeArgs {
    pub fn run(self) {
        println!("{}", CarettaId::from_u64_lossy(self.value))
    }
}
