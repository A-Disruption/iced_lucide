//! Subsetting behaviour, exercised through the public API.
//!
//! These live outside `src/subset.rs` so that module stays free of anything
//! but its own logic: `tools/vendor` includes its source directly, and a test
//! module reaching for `crate::families()` would not compile there.

use iced_lucide::{Family, families};

/// Any enabled family will do. `None` when the crate is built with no families
/// at all, which is legal if not much use.
fn sample() -> Option<&'static Family> {
    families().first().copied()
}

#[test]
fn subset_is_smaller_and_still_parses() {
    let Some(family) = sample() else {
        return;
    };

    let codepoints: Vec<u32> = family.icons().iter().take(5).map(|(_, c)| *c).collect();
    let subsetted = family.subset(&codepoints);

    assert!(
        subsetted.len() < family.font_bytes().len(),
        "subset ({} bytes) should be smaller than the full font ({} bytes)",
        subsetted.len(),
        family.font_bytes().len(),
    );

    assert!(
        ttf_parser::Face::parse(&subsetted, 0).is_ok(),
        "ttf-parser must accept the subsetted font"
    );
}

#[test]
fn subset_keeps_every_requested_glyph_reachable() {
    for family in families() {
        let wanted: Vec<(String, u32)> = family.icons().into_iter().take(20).collect();
        let codepoints: Vec<u32> = wanted.iter().map(|(_, c)| *c).collect();

        let subsetted = family.subset(&codepoints);
        let face = ttf_parser::Face::parse(&subsetted, 0).expect("parse subset");

        for (name, codepoint) in &wanted {
            let character = char::from_u32(*codepoint).expect("valid codepoint");

            assert!(
                face.glyph_index(character).is_some(),
                "{}:{name} (U+{codepoint:04X}) should survive subsetting",
                family.id(),
            );
        }
    }
}

#[test]
fn subset_drops_glyphs_that_were_not_requested() {
    let Some(family) = sample() else {
        return;
    };

    let icons = family.icons();
    if icons.len() < 50 {
        return;
    }

    let subsetted = family.subset(&[icons[0].1]);
    let face = ttf_parser::Face::parse(&subsetted, 0).expect("parse subset");

    assert!(
        face.glyph_index(char::from_u32(icons[40].1).expect("valid codepoint"))
            .is_none(),
        "an icon that was not requested should not be reachable in the subset"
    );
}

#[test]
fn subset_is_addressable_by_the_family_name() {
    for family in families() {
        let codepoints: Vec<u32> = family.icons().iter().take(3).map(|(_, c)| *c).collect();
        let subsetted = family.subset(&codepoints);
        let face = ttf_parser::Face::parse(&subsetted, 0).expect("parse subset");

        let names: Vec<String> = face
            .names()
            .into_iter()
            .filter(|name| name.name_id == ttf_parser::name_id::FAMILY)
            .filter_map(|name| name.to_string())
            .collect();

        assert!(
            names.iter().any(|name| name == family.font_family()),
            "{} should be addressable as {:?}, found {names:?}",
            family.id(),
            family.font_family(),
        );
    }
}
