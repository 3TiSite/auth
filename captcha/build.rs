fn main() -> anyhow::Result<()> {
  let root = std::env::var("CARGO_MANIFEST_DIR")?;
  prost_build::compile_protos(&["api.proto"], &[root])?;
  Ok(())
}
