use caretta_id::CarettaId;
use clap::Args;

#[derive(Args, Debug)]
/// (deprecated) Generate random caretta-id
pub struct GenerateArgs {}

impl GenerateArgs {
    pub fn run(self) {
        println!("{}", rand::random::<CarettaId>())
    }
}
