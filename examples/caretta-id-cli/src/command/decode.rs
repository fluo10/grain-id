use caretta_id::CarettaId;
use clap::Args;

/// Decode caretta-id string to integer.
#[derive(Args, Debug)]
pub struct DecodeArgs {
    /// Caretta id String
    ///
    /// Examples: "123" "456-789" "abc-def-ghj" "kmn-pqr-stv-wxy"
    caretta_id: CarettaId,
}

impl DecodeArgs {
    pub fn run(self) {
        println!("{}", u64::from(self.caretta_id))
    }
}
