fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "prost")]
    prost_build::Config::new()
    .compile_protos(
        &[
            "caretta-id-proto/caretta_id/caretta_id.proto",
        ],
        &["caretta-id-proto/"],
    )?;
    Ok(())
}
