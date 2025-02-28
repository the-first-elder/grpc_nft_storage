fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().compile_protos(
        &["proto/nft.proto"],       // Ensure path is correct
        &["proto", "/usr/include"], // Search for proto files in the "proto" folder
    )?;
    Ok(())
}
