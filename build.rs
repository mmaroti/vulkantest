
fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed=src/test.glsl");
    Ok(())
}
