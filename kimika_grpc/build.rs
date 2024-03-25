fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=proto");
    tonic_build::compile_protos("proto/local.proto")?;
    Ok(())
}
