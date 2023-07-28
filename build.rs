fn main() -> miette::Result<()> {
    let root = std::path::PathBuf::from(".");
    let src = std::path::PathBuf::from("./src");
    let include_paths = &[&root, &src];
    let mut b = autocxx_build::Builder::new("src/lib.rs", include_paths).build()?;
    b.flag_if_supported("-std=c++17").compile("rtneural-rs");
    println!("cargo:rerun-if-changed=src/lib.rs");
    Ok(())
}
