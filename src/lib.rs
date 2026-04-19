#![allow(clippy::needless_doctest_main)]
//! A compile-time, type-safe [Lucide] icon font library for [`iced`].
//!
//! Parses Lucide's `unicode.html` at build time — no network calls required.
//!
//! [`iced`]: https://github.com/iced-rs/iced
//! [Lucide]: https://lucide.dev
//!
//! # Usage
//!
//! Create a `.toml` file in your crate with the icon definition:
//!
//! ```toml
//! # fonts/my-icons.toml
//! module = "icon"
//!
//! [icons]
//! edit   = "pencil"
//! save   = "save"
//! trash  = "trash-2"
//! search = "search"
//! ```
//!
//! Each key is the Rust function name; each value is the Lucide icon name
//! (as shown on <https://lucide.dev/icons>).
//!
//! Add `iced_lucide` to your `build-dependencies`:
//!
//! ```toml
//! [build-dependencies]
//! iced_lucide = "0.1"
//! ```
//!
//! Then call [`build`] in your build script:
//!
//! ```rust,no_run
//! pub fn main() {
//!     println!("cargo::rerun-if-changed=fonts/my-icons.toml");
//!     iced_lucide::build("fonts/my-icons.toml").expect("Build icon module");
//! }
//! ```
//!
//! This generates `src/icon.rs` and copies `lucide.ttf` next to your TOML.
//!
//! Finally, register the font in your application and use the generated
//! functions:
//!
//! ```rust,ignore
//! mod icon;
//!
//! fn main() -> iced::Result {
//!     iced::application(App::default, App::update, App::view)
//!         .font(icon::FONT)
//!         .run()
//! }
//!
//! fn view(&self) -> iced::Element<'_, ()> {
//!     iced::widget::row![icon::edit(), icon::save(), icon::trash()]
//!         .spacing(10)
//!         .into()
//! }
//! ```
//!
//! # Generating All Icons
//!
//! For icon pickers or UI builders that need every icon, use [`build_all`]:
//!
//! ```rust,no_run
//! pub fn main() {
//!     iced_lucide::build_all("icon").expect("Build all icons");
//! }
//! ```
//!
//! The generated module exposes `ALL_ICONS: &[(&str, &str)]` — a static list
//! of `(icon_name, unicode_codepoint)` pairs — in addition to typed functions
//! for every icon.
//!
//! # Runtime Icon Enumeration
//!
//! If `iced_lucide` is also a regular dependency, [`icons`] returns every
//! available icon for use in a picker widget:
//!
//! ```rust,ignore
//! for (name, codepoint) in iced_lucide::icons() {
//!     // render or store each icon
//! }
//! ```

use serde::Deserialize;

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

// ---------------------------------------------------------------------------
// Embedded assets
// ---------------------------------------------------------------------------

/// The raw bytes of the bundled Lucide TTF font.
///
/// Add `iced_lucide` as a regular dependency and use this constant to register
/// the font with iced when you need all icons available at runtime.
pub const FONT_BYTES: &[u8] = include_bytes!("../assets/lucide.ttf");

const UNICODE_HTML: &str = include_str!("../assets/unicode.html");

// ---------------------------------------------------------------------------
// Public build-time API
// ---------------------------------------------------------------------------

/// Build a type-safe icon module from a TOML definition file.
///
/// Reads the font definition, validates every requested icon name against
/// Lucide's icon set, copies `lucide.ttf` next to the TOML, and generates
/// `src/{module}.rs`.
///
/// Call this from your `build.rs`:
///
/// ```rust,no_run
/// pub fn main() {
///     println!("cargo::rerun-if-changed=fonts/my-icons.toml");
///     iced_lucide::build("fonts/my-icons.toml").expect("Build icon module");
/// }
/// ```
pub fn build(path: impl AsRef<Path>) -> Result<(), Error> {
    let path = path.as_ref();

    let definition: Definition = {
        let contents = fs::read_to_string(path).unwrap_or_else(|error| {
            panic!(
                "Icon definition {path} could not be read: {error}",
                path = path.display()
            )
        });

        toml::from_str(&contents).unwrap_or_else(|error| {
            panic!(
                "Icon definition {path} is invalid: {error}",
                path = path.display()
            )
        })
    };

    let all = parse_icons();

    let icons: BTreeMap<String, u32> = definition
        .icons
        .into_iter()
        .map(|(fn_name, icon_name)| {
            let Some(&code) = all.get(&icon_name) else {
                let candidates: Vec<_> = all
                    .keys()
                    .filter(|k| k.contains(icon_name.split('-').next().unwrap_or("")))
                    .take(5)
                    .map(String::as_str)
                    .collect();

                let hint = if candidates.is_empty() {
                    String::new()
                } else {
                    format!("\nSimilar icons: {}", candidates.join(", "))
                };

                panic!(
                    "Lucide icon \"{icon_name}\" was not found.\
                    \nBrowse all icons at https://lucide.dev/icons{hint}"
                );
            };
            (fn_name, code)
        })
        .collect();

    let hash = compute_hash(&icons);

    let module_depth = definition.module.split("::").count();
    let module_target = PathBuf::from("src")
        .join(definition.module.replace("::", "/"))
        .with_extension("rs");

    // Tell Cargo to re-run if the generated file is missing or modified.
    println!("cargo::rerun-if-changed={}", module_target.display());

    // Relative path from the generated module back to the project root
    let rel_root: PathBuf = std::iter::repeat("../")
        .take(module_depth)
        .collect::<String>()
        .into();

    // TTF lives next to the TOML
    let ttf_target = path.with_file_name("lucide.ttf");
    let ttf_rel = rel_root.join(&ttf_target);

    let existing = fs::read_to_string(&module_target).unwrap_or_default();
    let existing_hash = existing
        .lines()
        .nth(2)
        .unwrap_or_default()
        .trim_start_matches("// ");

    if hash != existing_hash || !ttf_target.exists() {
        // Build a subset TTF with only the requested glyphs
        let codepoints: Vec<u32> = icons.values().copied().collect();
        let font_data = subset_font(&codepoints);

        fs::write(&ttf_target, &font_data)
            .unwrap_or_else(|e| panic!("Write lucide.ttf to {}: {e}", ttf_target.display()));

        let module = generate_module(&icons, &hash, ttf_rel.to_string_lossy().replace('\\', "/"));

        if let Some(dir) = module_target.parent() {
            fs::create_dir_all(dir).expect("Create src directory");
        }
        fs::write(&module_target, module).expect("Write icon module");
    }

    Ok(())
}

/// Generate a module containing **every** Lucide icon.
///
/// Writes `lucide.ttf` into `fonts/` (creating the directory if needed),
/// then generates `src/{module_name}.rs`.
///
/// ```rust,no_run
/// pub fn main() {
///     iced_lucide::build_all("lucide_icon").expect("Build all Lucide icons");
/// }
/// ```
///
/// The generated module exports:
/// - `FONT: &[u8]` — the TTF bytes
/// - `ALL_ICONS: &[(&str, &str)]` — `(icon_name, codepoint_str)` pairs for
///   use in picker widgets
/// - One typed function per icon, e.g. `pub fn pencil<'a>() -> Text<'a>`
pub fn build_all(module_name: &str) -> Result<(), Error> {
    let all_icons: BTreeMap<String, u32> = parse_icons()
        .into_iter()
        .map(|(name, code)| (sanitize_fn_name(&name), code))
        .collect();

    let hash = compute_hash(&all_icons);

    let module_depth = module_name.split("::").count();
    let module_target = PathBuf::from("src")
        .join(module_name.replace("::", "/"))
        .with_extension("rs");

    // Tell Cargo to re-run if the generated file is missing or modified.
    println!("cargo::rerun-if-changed={}", module_target.display());

    let rel_root: PathBuf = std::iter::repeat("../")
        .take(module_depth)
        .collect::<String>()
        .into();

    // TTF written to fonts/lucide.ttf
    let ttf_dir = PathBuf::from("fonts");
    fs::create_dir_all(&ttf_dir).expect("Create fonts directory");
    let ttf_target = ttf_dir.join("lucide.ttf");
    let ttf_rel = rel_root.join(&ttf_target);

    let existing = fs::read_to_string(&module_target).unwrap_or_default();
    let existing_hash = existing
        .lines()
        .nth(2)
        .unwrap_or_default()
        .trim_start_matches("// ");

    if hash != existing_hash || !ttf_target.exists() {
        fs::write(&ttf_target, FONT_BYTES).unwrap_or_else(|e| panic!("Write lucide.ttf: {e}"));

        let module = generate_module(
            &all_icons,
            &hash,
            ttf_rel.to_string_lossy().replace('\\', "/"),
        );

        if let Some(dir) = module_target.parent() {
            fs::create_dir_all(dir).expect("Create src directory");
        }
        fs::write(&module_target, module).expect("Write icon module");
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Public runtime API
// ---------------------------------------------------------------------------

/// Returns every Lucide icon as `(name, codepoint)` pairs, sorted by name.
///
/// Useful for populating icon-picker widgets at runtime. Add `iced_lucide`
/// as a regular dependency (not just a build-dependency) to use this.
///
/// ```rust,ignore
/// for (name, codepoint) in iced_lucide::icons() {
///     println!("{name} -> U+{codepoint:04X}");
/// }
/// ```
pub fn icons() -> Vec<(String, u32)> {
    let mut list: Vec<(String, u32)> = parse_icons().into_iter().collect();
    list.sort_by(|a, b| a.0.cmp(&b.0));
    list
}

// ---------------------------------------------------------------------------
// Error type
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum Error {}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize)]
struct Definition {
    module: String,
    icons: BTreeMap<String, String>,
}

/// Parse `unicode.html` into a map of `icon-name → unicode codepoint`.
///
/// The HTML contains entries like:
/// ```html
/// <h4>pencil</h4><span class="unicode">&amp;#57347;</span>
/// ```
fn parse_icons() -> HashMap<String, u32> {
    let mut map = HashMap::new();
    let mut remaining = UNICODE_HTML;

    while let Some(h4_start) = remaining.find("<h4>") {
        remaining = &remaining[h4_start + 4..];

        let Some(h4_end) = remaining.find("</h4>") else {
            break;
        };

        let name = remaining[..h4_end].trim().to_string();
        remaining = &remaining[h4_end + 5..];

        // The codepoint span immediately follows the </h4> within the same <li>
        let li_end = remaining.find("</li>").unwrap_or(remaining.len());
        let li_tail = &remaining[..li_end];

        if let Some(amp_pos) = li_tail.find("&amp;#") {
            let after_hash = &li_tail[amp_pos + 6..];
            if let Some(semi) = after_hash.find(';') {
                if let Ok(code) = after_hash[..semi].parse::<u32>() {
                    map.insert(name, code);
                }
            }
        }
    }

    map
}

/// Build a subset of the bundled Lucide TTF containing only the requested glyphs.
///
/// Uses `subsetter` to strip unused glyph outlines, then injects a cmap table
/// so the result works as a standalone screen font (subsetter removes cmap because
/// it targets PDF embedding, which provides its own cmap).
fn subset_font(codepoints: &[u32]) -> Vec<u8> {
    let face = ttf_parser::Face::parse(FONT_BYTES, 0).expect("Parse bundled lucide.ttf");

    // GlyphRemapper::new() already includes .notdef (glyph 0).
    let mut remapper = subsetter::GlyphRemapper::new();
    let mut cp_to_old_gid: Vec<(u32, u16)> = Vec::new();

    for &cp in codepoints {
        if let Some(ch) = char::from_u32(cp) {
            if let Some(gid) = face.glyph_index(ch) {
                remapper.remap(gid.0);
                cp_to_old_gid.push((cp, gid.0));
            }
        }
    }

    // Subset strips unused outlines and removes the cmap table.
    let subset_data = match subsetter::subset(FONT_BYTES, 0, &remapper) {
        Ok(data) => data,
        Err(_) => return FONT_BYTES.to_vec(),
    };

    // Translate codepoints to their new (remapped) glyph IDs.
    let mut cp_to_new_gid: Vec<(u32, u16)> = cp_to_old_gid
        .into_iter()
        .filter_map(|(cp, old_gid)| remapper.get(old_gid).map(|new_gid| (cp, new_gid)))
        .collect();

    // Re-inject a cmap so iced can look up glyphs by Unicode codepoint.
    let cmap = build_cmap(&mut cp_to_new_gid);
    let with_cmap = inject_table(&subset_data, b"cmap", &cmap);

    // Preserve the name table from the original font so Lucide's license
    // metadata is carried over into the subset (subsetter removes it).
    match extract_table(FONT_BYTES, b"name") {
        Some(name_data) => inject_table(&with_cmap, b"name", &name_data),
        None => with_cmap,
    }
}

/// Build a cmap table (format 12) mapping codepoints → new glyph IDs.
fn build_cmap(entries: &mut Vec<(u32, u16)>) -> Vec<u8> {
    entries.sort_by_key(|&(cp, _)| cp);
    entries.dedup_by_key(|(cp, _)| *cp);

    let n = entries.len() as u32;
    // format(2) + reserved(2) + length(4) + language(4) + numGroups(4) + n*12
    let subtable_len: u32 = 16 + n * 12;

    let mut cmap = Vec::with_capacity(12 + subtable_len as usize);

    // cmap table header
    cmap.extend_from_slice(&0u16.to_be_bytes()); // version
    cmap.extend_from_slice(&1u16.to_be_bytes()); // numTables

    // Encoding record: Windows / Unicode full repertoire (platformID=3, encodingID=10)
    cmap.extend_from_slice(&3u16.to_be_bytes()); // platformID
    cmap.extend_from_slice(&10u16.to_be_bytes()); // encodingID
    // Offset from start of cmap table to subtable: header(4) + record(8) = 12
    cmap.extend_from_slice(&12u32.to_be_bytes());

    // Subtable (format 12)
    cmap.extend_from_slice(&12u16.to_be_bytes()); // format
    cmap.extend_from_slice(&0u16.to_be_bytes()); // reserved
    cmap.extend_from_slice(&subtable_len.to_be_bytes()); // length
    cmap.extend_from_slice(&0u32.to_be_bytes()); // language
    cmap.extend_from_slice(&n.to_be_bytes()); // numGroups

    // One SequentialMapGroup per codepoint (startCharCode = endCharCode = cp)
    for &(cp, gid) in entries.iter() {
        cmap.extend_from_slice(&cp.to_be_bytes());
        cmap.extend_from_slice(&cp.to_be_bytes());
        cmap.extend_from_slice(&(gid as u32).to_be_bytes());
    }

    cmap
}

/// Extract a named table's raw bytes from an OpenType font binary.
fn extract_table(font: &[u8], tag: &[u8; 4]) -> Option<Vec<u8>> {
    if font.len() < 12 {
        return None;
    }
    let num_tables = u16::from_be_bytes([font[4], font[5]]) as usize;
    for i in 0..num_tables {
        let base = 12 + i * 16;
        if base + 16 > font.len() {
            break;
        }
        let t: [u8; 4] = font[base..base + 4].try_into().ok()?;
        if &t == tag {
            let offset = u32::from_be_bytes(font[base + 8..base + 12].try_into().ok()?) as usize;
            let length = u32::from_be_bytes(font[base + 12..base + 16].try_into().ok()?) as usize;
            return font.get(offset..offset + length).map(|d| d.to_vec());
        }
    }
    None
}

/// Inject (or replace) a named table in an OpenType font binary.
fn inject_table(font: &[u8], tag: &[u8; 4], table_data: &[u8]) -> Vec<u8> {
    if font.len() < 12 {
        return font.to_vec();
    }

    let flavor = u32::from_be_bytes(font[0..4].try_into().expect("4 bytes"));
    let num_tables = u16::from_be_bytes([font[4], font[5]]) as usize;

    let mut tables: Vec<([u8; 4], Vec<u8>)> = Vec::with_capacity(num_tables + 1);
    for i in 0..num_tables {
        let base = 12 + i * 16;
        if base + 16 > font.len() {
            break;
        }
        let t: [u8; 4] = font[base..base + 4].try_into().expect("4 bytes");
        let offset =
            u32::from_be_bytes(font[base + 8..base + 12].try_into().expect("4 bytes")) as usize;
        let length =
            u32::from_be_bytes(font[base + 12..base + 16].try_into().expect("4 bytes")) as usize;
        let data = font.get(offset..offset + length).unwrap_or(&[]).to_vec();
        tables.push((t, data));
    }

    // Replace existing cmap if present, otherwise append.
    tables.retain(|(t, _)| t != tag);
    tables.push((*tag, table_data.to_vec()));

    // OpenType spec requires table records sorted by tag.
    tables.sort_by_key(|(t, _)| *t);

    reconstruct_otf(flavor, &tables)
}

/// Rebuild a complete OpenType font binary from a sorted table list.
fn reconstruct_otf(flavor: u32, tables: &[([u8; 4], Vec<u8>)]) -> Vec<u8> {
    let n = tables.len() as u16;
    let entry_selector = if n > 0 {
        (n as f64).log2().floor() as u16
    } else {
        0
    };
    let search_range = 2u16.pow(u32::from(entry_selector)) * 16;
    let range_shift = n * 16 - search_range;

    // Pre-compute each table's offset in the final binary.
    let dir_size = 12 + tables.len() * 16;
    let mut offsets = Vec::with_capacity(tables.len());
    let mut cur = dir_size;
    for (_, data) in tables {
        offsets.push(cur as u32);
        cur += data.len();
        while cur % 4 != 0 {
            cur += 1;
        }
    }

    let mut font = Vec::with_capacity(cur);

    // Offset table
    font.extend_from_slice(&flavor.to_be_bytes());
    font.extend_from_slice(&n.to_be_bytes());
    font.extend_from_slice(&search_range.to_be_bytes());
    font.extend_from_slice(&entry_selector.to_be_bytes());
    font.extend_from_slice(&range_shift.to_be_bytes());

    // Table directory — head checksum adjustment field must be zeroed before checksumming.
    let mut head_adj_offset: Option<usize> = None;
    for ((tag, data), &off) in tables.iter().zip(offsets.iter()) {
        let cs = if tag == b"head" && data.len() >= 12 {
            let mut zeroed = data.clone();
            zeroed[8..12].fill(0);
            otf_checksum(&zeroed)
        } else {
            otf_checksum(data)
        };
        font.extend_from_slice(tag);
        font.extend_from_slice(&cs.to_be_bytes());
        font.extend_from_slice(&off.to_be_bytes());
        font.extend_from_slice(&(data.len() as u32).to_be_bytes());
        if tag == b"head" {
            head_adj_offset = Some(off as usize + 8);
        }
    }

    // Table data
    for (tag, data) in tables {
        if tag == b"head" && data.len() >= 12 {
            font.extend_from_slice(&data[..8]);
            font.extend_from_slice(&[0u8; 4]); // zero adjustment before whole-font checksum
            font.extend_from_slice(&data[12..]);
        } else {
            font.extend_from_slice(data);
        }
        while font.len() % 4 != 0 {
            font.push(0);
        }
    }

    // Write head checksum adjustment = 0xB1B0AFBA − (whole-font checksum).
    if let Some(i) = head_adj_offset {
        let sum = otf_checksum(&font);
        let val = 0xB1B0AFBA_u32.wrapping_sub(sum);
        if i + 4 <= font.len() {
            font[i..i + 4].copy_from_slice(&val.to_be_bytes());
        }
    }

    font
}

/// OpenType table checksum: sum of big-endian u32 words, zero-padding the last chunk.
fn otf_checksum(data: &[u8]) -> u32 {
    let mut sum = 0u32;
    for chunk in data.chunks(4) {
        let mut bytes = [0u8; 4];
        bytes[..chunk.len()].copy_from_slice(chunk);
        sum = sum.wrapping_add(u32::from_be_bytes(bytes));
    }
    sum
}

/// Convert a lucide icon name (kebab-case) to a valid Rust identifier.
///
/// - `pencil`    → `pencil`
/// - `trash-2`   → `trash_2`
/// - `3d-rotation` → `icon_3d_rotation`
/// - `move`      → `move_icon`  (Rust keyword)
/// - `type`      → `type_icon`  (Rust keyword)
fn sanitize_fn_name(name: &str) -> String {
    // Strict and reserved keywords in all Rust editions.
    const KEYWORDS: &[&str] = &[
        "abstract", "as", "async", "await", "become", "box", "break", "const", "continue", "crate",
        "do", "dyn", "else", "enum", "extern", "false", "final", "fn", "for", "if", "impl", "in",
        "let", "loop", "macro", "match", "mod", "move", "mut", "override", "priv", "pub", "ref",
        "return", "self", "static", "struct", "super", "trait", "true", "try", "type", "typeof",
        "unsafe", "unsized", "use", "virtual", "where", "while", "yield",
    ];

    let mut s = name.replace('-', "_");
    if s.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        s = format!("icon_{s}");
    }
    if KEYWORDS.contains(&s.as_str()) {
        s = format!("{s}_icon");
    }
    s
}

/// SHA-256 hash of the sorted icon list, returned as a hex string.
fn compute_hash(icons: &BTreeMap<String, u32>) -> String {
    use sha2::Digest as _;
    let mut hasher = sha2::Sha256::new();
    for (name, code) in icons {
        hasher.update(name.as_bytes());
        hasher.update(b":");
        hasher.update(code.to_le_bytes());
        hasher.update(b"|");
    }
    format!("{:x}", hasher.finalize())
}

/// Render the Rust module source.
fn generate_module(icons: &BTreeMap<String, u32>, hash: &str, ttf_path: String) -> String {
    let mut out = String::new();

    out.push_str(&format!(
        "// Generated automatically by iced_lucide at build time.\n\
         // Do not edit manually.\n\
         // {hash}\n\
         use iced::Font;\n\
         use iced::widget::{{Text, text}};\n\n\
         pub const FONT: &[u8] = include_bytes!(\"{ttf_path}\");\n\n"
    ));

    // Static slice of (name, codepoint_str) for icon pickers
    out.push_str(
        "/// All icons as `(name, codepoint_str)` pairs.\n\
         /// Use this to populate an icon-picker widget.\n\
         #[allow(dead_code)]\n\
         pub const ALL_ICONS: &[(&str, &str)] = &[\n",
    );
    for (name, code) in icons {
        out.push_str(&format!("    (\"{name}\", \"\\u{{{code:X}}}\"),\n"));
    }
    out.push_str("];\n\n");

    // One typed function per icon
    for (name, code) in icons {
        out.push_str(&format!(
            "pub fn {name}<'a>() -> Text<'a> {{\n    icon(\"\\u{{{code:X}}}\")\n}}\n\n"
        ));
    }

    // Public render helper — for use with ALL_ICONS in picker widgets
    out.push_str(
        "/// Render any Lucide icon by its codepoint string.\n\
         /// Use this together with [`ALL_ICONS`] to display icons dynamically:\n\
         /// ```ignore\n\
         /// for (name, cp) in ALL_ICONS {\n\
         ///     button(render(cp)).on_press(Msg::Pick(name.to_string()))\n\
         /// }\n\
         /// ```\n\
         pub fn render(codepoint: &str) -> Text<'_> {\n    text(codepoint).font(Font::new(\"lucide\"))\n}\n\n",
    );

    // Private helper used by typed icon functions
    out.push_str("fn icon(codepoint: &str) -> Text<'_> {\n    render(codepoint)\n}\n");

    out
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_icons() {
        let icons = parse_icons();
        assert!(!icons.is_empty(), "should find at least one icon");
        assert!(icons.contains_key("pencil"), "should contain 'pencil'");
        assert!(icons.contains_key("trash-2"), "should contain 'trash-2'");
        assert!(icons.contains_key("search"), "should contain 'search'");
    }

    #[test]
    fn icon_count_reasonable() {
        let icons = parse_icons();
        assert!(
            icons.len() > 1000,
            "expected >1000 icons, got {}",
            icons.len()
        );
    }

    #[test]
    fn sanitize_fn_name_works() {
        assert_eq!(sanitize_fn_name("pencil"), "pencil");
        assert_eq!(sanitize_fn_name("trash-2"), "trash_2");
        assert_eq!(sanitize_fn_name("a-arrow-down"), "a_arrow_down");
        assert_eq!(sanitize_fn_name("3d-rotation"), "icon_3d_rotation");
        assert_eq!(sanitize_fn_name("move"), "move_icon");
        assert_eq!(sanitize_fn_name("type"), "type_icon");
    }

    #[test]
    fn hash_is_stable() {
        let mut icons = BTreeMap::new();
        icons.insert("edit".to_string(), 0xE001u32);
        icons.insert("save".to_string(), 0xE002u32);
        let h1 = compute_hash(&icons);
        let h2 = compute_hash(&icons);
        assert_eq!(h1, h2);
    }

    #[test]
    fn runtime_icons_sorted() {
        let list = icons();
        let names: Vec<_> = list.iter().map(|(n, _)| n.as_str()).collect();
        let mut sorted = names.clone();
        sorted.sort();
        assert_eq!(names, sorted);
    }

    #[test]
    fn subset_is_smaller_and_valid() {
        // A handful of icons — far fewer than the full 1685.
        let codepoints = [0xE001, 0xE002, 0xE003, 0xE004, 0xE005];
        let subsetted = subset_font(&codepoints);

        // Must be smaller than the full font.
        assert!(
            subsetted.len() < FONT_BYTES.len(),
            "subset ({} bytes) should be smaller than full font ({} bytes)",
            subsetted.len(),
            FONT_BYTES.len(),
        );

        // Must still be a valid TrueType font (correct magic bytes).
        assert_eq!(
            &subsetted[0..4],
            &[0x00, 0x01, 0x00, 0x00],
            "subsetted font must start with TrueType magic"
        );

        // Must contain a cmap table (we injected one).
        let num_tables = u16::from_be_bytes([subsetted[4], subsetted[5]]) as usize;
        let has_cmap = (0..num_tables).any(|i| {
            let base = 12 + i * 16;
            subsetted.get(base..base + 4) == Some(b"cmap")
        });
        assert!(has_cmap, "subsetted font must contain a cmap table");

        // ttf-parser should be able to parse it.
        let face = ttf_parser::Face::parse(&subsetted, 0);
        assert!(face.is_ok(), "ttf-parser must accept the subsetted font");
    }
}
