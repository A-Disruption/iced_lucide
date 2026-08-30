//! Cutting a font down to the glyphs a project actually uses.

use crate::otf;

/// Build a font containing only `codepoints`, addressable as `family`.
///
/// `subsetter` targets PDF embedding, where the container supplies its own
/// character map and the font needs no name of its own. A screen font needs
/// both, so they are rebuilt here: a `cmap` mapping each requested codepoint to
/// its remapped glyph, and a `name` table announcing the family the generated
/// code will ask for.
///
/// Returns the original font unchanged if subsetting fails, which keeps a build
/// working — larger than intended, but correct — rather than emitting a font
/// that renders nothing.
pub fn subset(font: &[u8], codepoints: &[u32], family: &str) -> Vec<u8> {
    let Ok(face) = ttf_parser::Face::parse(font, 0) else {
        return font.to_vec();
    };

    // GlyphRemapper::new() already includes .notdef (glyph 0).
    let mut remapper = subsetter::GlyphRemapper::new();
    let mut mapping: Vec<(u32, u16)> = Vec::with_capacity(codepoints.len());

    for &codepoint in codepoints {
        let Some(glyph) = char::from_u32(codepoint).and_then(|c| face.glyph_index(c)) else {
            continue;
        };

        remapper.remap(glyph.0);
        mapping.push((codepoint, glyph.0));
    }

    let Ok(subsetted) = subsetter::subset(font, 0, &remapper) else {
        return font.to_vec();
    };

    // Translate each codepoint to the glyph ID it was moved to.
    let mut remapped: Vec<(u32, u16)> = mapping
        .into_iter()
        .filter_map(|(codepoint, old)| remapper.get(old).map(|new| (codepoint, new)))
        .collect();

    let with_cmap = otf::inject_table(&subsetted, b"cmap", &otf::build_cmap(&mut remapped));

    // A font database consults OS/2 and post when deciding whether a face is
    // usable; restore them if the subsetter judged them unnecessary.
    let with_metrics = otf::carry_over(&with_cmap, font, b"OS/2");
    let with_metrics = otf::carry_over(&with_metrics, font, b"post");

    otf::rewrite_name_table(&with_metrics, family)
}
