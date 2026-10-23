#[path = "src/design.rs"]
mod design;

fn main() -> std::io::Result<()> {
    use std::hash::{Hash, Hasher};

    println!("cargo:rerun-if-changed=src/design.rs");
    let output_directory = std::env::var_os("OUT_DIR")
        .ok_or_else(|| std::io::Error::other("Cargo did not provide OUT_DIR"))?;
    let output_path = std::path::PathBuf::from(output_directory).join("portfolio.css");
    let stylesheet = design::stylesheet();
    // A content version prevents browsers reusing CSS from an older deployment.
    let mut version = std::collections::hash_map::DefaultHasher::new();
    stylesheet.hash(&mut version);
    println!(
        "cargo:rustc-env=PORTFOLIO_STYLE_VERSION={:016x}",
        version.finish()
    );
    std::fs::write(output_path, stylesheet)
}
