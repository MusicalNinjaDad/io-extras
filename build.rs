use ninja_build_rs::prelude::*;

fn main() -> Result<()> {
    let ac = AutoCfg::new()?;

    let allowed_features = cargo_allowed_features()?;

    ac.emit_unstable_feature(can_vector, &allowed_features); // https://github.com/rust-lang/rust/issues/69941
    ac.emit_unstable_feature(write_all_vectored, &allowed_features); // https://github.com/rust-lang/rust/issues/70436

    use_feature("io_lifetimes_use_std");

    // Don't rerun this on changes other than build.rs, as we only depend on
    // the rustc version.
    println!("cargo:rerun-if-changed=build.rs");

    Ok(())
}

fn use_feature(feature: &str) {
    println!("cargo:rustc-cfg={}", feature);
}
