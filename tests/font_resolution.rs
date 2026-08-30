//! Checks the subsets against the font database `iced` actually resolves with.
//!
//! Everything else in the test suite verifies the subsets with `ttf-parser`,
//! which is more forgiving than the loader in the real rendering path. `iced`
//! reaches fonts through `cosmic-text`, which reaches them through `fontdb`, so
//! that is what these tests use: a subset that `fontdb` will not load, or will
//! not return for the family name the generated code asks for, renders nothing
//! no matter how well-formed it looks.

use iced_lucide::{Family, families};

/// Load a subset into a fresh database and ask for it by family name.
fn resolve(font: Vec<u8>, family: &str) -> Option<fontdb::ID> {
    let mut database = fontdb::Database::new();
    database.load_font_data(font);

    database.query(&fontdb::Query {
        families: &[fontdb::Family::Name(family)],
        ..fontdb::Query::default()
    })
}

/// Cut a small subset from a family, the way a build script would.
fn subset_of(family: &Family) -> Vec<u8> {
    let codepoints: Vec<u32> = family
        .icons()
        .into_iter()
        .take(8)
        .map(|(_, codepoint)| codepoint)
        .collect();

    family.subset(&codepoints)
}

#[test]
fn every_subset_is_resolvable_by_its_family_name() {
    for family in families() {
        let font = subset_of(family);

        assert!(
            resolve(font, family.font_family()).is_some(),
            "fontdb could not resolve {} by the family name {:?} that the \
             generated code asks for",
            family.id(),
            family.font_family(),
        );
    }
}

#[test]
fn every_subset_keeps_its_glyphs_addressable_through_fontdb() {
    for family in families() {
        let wanted: Vec<(String, u32)> = family.icons().into_iter().take(8).collect();
        let codepoints: Vec<u32> = wanted.iter().map(|(_, code)| *code).collect();
        let font = family.subset(&codepoints);

        let mut database = fontdb::Database::new();
        database.load_font_data(font);

        let id = database
            .query(&fontdb::Query {
                families: &[fontdb::Family::Name(family.font_family())],
                ..fontdb::Query::default()
            })
            .unwrap_or_else(|| panic!("{} should resolve", family.id()));

        database.with_face_data(id, |data, index| {
            let face = ttf_parser::Face::parse(data, index).expect("fontdb handed back a face");

            for (name, codepoint) in &wanted {
                let character = char::from_u32(*codepoint).expect("valid codepoint");

                assert!(
                    face.glyph_index(character).is_some(),
                    "{}:{name} is unreachable through fontdb",
                    family.id(),
                );
            }
        });
    }
}

/// The reason the `name` table is rewritten at all.
///
/// Font Awesome's solid and regular faces are both called "Font Awesome 7 Free"
/// upstream and differ only by weight, so a database holding both cannot tell
/// them apart by family. After renaming, each resolves to itself even when both
/// are loaded together.
#[cfg(feature = "fontawesome")]
#[test]
fn font_awesome_faces_stay_distinct_when_loaded_together() {
    let solid = iced_lucide::family("fa-solid").expect("fontawesome feature is on");
    let regular = iced_lucide::family("fa-regular").expect("fontawesome feature is on");

    // An icon each family has and the other does not.
    let solid_only = solid
        .icons()
        .into_iter()
        .find(|(name, _)| regular.codepoint(name).is_none())
        .expect("solid carries an icon regular does not");

    let mut database = fontdb::Database::new();
    database.load_font_data(subset_of(solid));
    database.load_font_data(subset_of(regular));

    for family in [solid, regular] {
        let id = database
            .query(&fontdb::Query {
                families: &[fontdb::Family::Name(family.font_family())],
                ..fontdb::Query::default()
            })
            .unwrap_or_else(|| panic!("{} should resolve with both loaded", family.id()));

        database.with_face_data(id, |data, index| {
            let face = ttf_parser::Face::parse(data, index).expect("parse");
            let names: Vec<String> = face
                .names()
                .into_iter()
                .filter(|name| name.name_id == ttf_parser::name_id::FAMILY)
                .filter_map(|name| name.to_string())
                .collect();

            assert!(
                names.iter().any(|name| name == family.font_family()),
                "querying {:?} returned a face named {names:?}",
                family.font_family(),
            );
        });
    }

    // And the returned solid face really is the solid one.
    let id = database
        .query(&fontdb::Query {
            families: &[fontdb::Family::Name("fa-solid")],
            ..fontdb::Query::default()
        })
        .expect("fa-solid resolves");

    database.with_face_data(id, |data, index| {
        let face = ttf_parser::Face::parse(data, index).expect("parse");
        let character = char::from_u32(solid_only.1).expect("valid codepoint");

        assert!(
            face.glyph_index(character).is_some(),
            "the face returned for fa-solid does not carry {}, a solid-only icon",
            solid_only.0,
        );
    });
}
