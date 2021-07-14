fn main() {
    println!("cargo:rerun-if-changed=src/winmd_path.rs");
    println!("cargo:rerun-if-env-changed=CARGO_MANIFEST_DIR");
    let source = std::path::Path::new(
        &::std::env::var("CARGO_MANIFEST_DIR").expect("No `CARGO_MANIFEST_DIR` env var"),
    )
    .join(".windows/winmd");

    ::std::fs::write(
        "src/winmd_path.rs",
        &format!(
            "pub(crate) const WINMD_PATH: &str = \"{}\";",
            source.display()
        ),
    )
    .expect("Failed to write winmd path to src/winmd_path.rs");
}
