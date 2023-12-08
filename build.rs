genv::def!(CARGO_MANIFEST_DIR);

fn main() -> anyhow::Result<()> {
  prost_build::compile_protos(&["api.proto"], &[CARGO_MANIFEST_DIR::<String>()])?;
  Ok(())
}
