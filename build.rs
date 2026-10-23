//! Unpacks the vendored fonts this build enables.
//!
//! The fonts in `assets/` are stored xz-compressed: uncompressed, the published
//! crate would not fit under the crates.io upload limit. Each enabled family's
//! font is decompressed into `OUT_DIR/fonts/`, which is where
//! `src/families.rs` embeds it from. Disabled families are never unpacked.

use std::env;
use std::fs;
use std::path::PathBuf;

include!("build/fonts.rs");

fn main() {
    let out = PathBuf::from(env::var_os("OUT_DIR").expect("cargo sets OUT_DIR")).join("fonts");
    fs::create_dir_all(&out).expect("create OUT_DIR/fonts");

    println!("cargo::rerun-if-changed=build/fonts.rs");

    for &(feature, id, stem) in FONTS {
        // Cargo exposes each enabled feature as CARGO_FEATURE_<NAME>, with the
        // name upper-cased and hyphens turned into underscores.
        let variable = format!("CARGO_FEATURE_{}", feature.to_uppercase().replace('-', "_"));

        if env::var_os(&variable).is_none() {
            continue;
        }

        let source = PathBuf::from("assets")
            .join(id)
            .join(format!("{stem}.ttf.xz"));
        println!("cargo::rerun-if-changed={}", source.display());

        let compressed =
            fs::read(&source).unwrap_or_else(|error| panic!("read {}: {error}", source.display()));

        let mut font = Vec::new();
        lzma_rs::xz_decompress(&mut compressed.as_slice(), &mut font)
            .unwrap_or_else(|error| panic!("decompress {}: {error:?}", source.display()));

        let target = out.join(format!("{stem}.ttf"));
        fs::write(&target, font)
            .unwrap_or_else(|error| panic!("write {}: {error}", target.display()));
    }
}
