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
//! ## Custom output paths
//!
//! By default the generated module is written to `src/<module>.rs` and the
//! subsetted TTF sits next to the TOML. Both destinations can be overridden
//! with optional fields, which is useful for Cargo examples or any layout
//! where the generated module is not a member of `src/`:
//!
//! ```toml
//! module        = "icon"
//! module_target = "examples/playground/icon.rs"
//! ttf_target    = "examples/playground/fonts/lucide.ttf"
//!
//! [icons]
//! edit = "pencil"
//! ```
//!
//! Both paths are resolved relative to the crate root (i.e. the directory
//! containing `Cargo.toml`). The generated `include_bytes!` call is rewritten
//! to match the relative distance between the two locations.
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

    let hash = compute_hash(&icons, definition.family.as_deref());

    // Resolve the destination for the generated `.rs` module. A custom
    // `module_target` wins over the historical `src/<module>.rs` default.
    let module_target = definition.module_target.clone().unwrap_or_else(|| {
        PathBuf::from("src")
            .join(definition.module.replace("::", "/"))
            .with_extension("rs")
    });

    // Resolve the destination for the subsetted TTF. A custom `ttf_target`
    // wins; otherwise the font is written next to the TOML, matching the
    // legacy behavior of previous releases.
    let ttf_target = definition
        .ttf_target
        .clone()
        .unwrap_or_else(|| path.with_file_name("lucide.ttf"));

    // Tell Cargo to re-run if either output is missing or modified.
    println!("cargo::rerun-if-changed={}", module_target.display());
    println!("cargo::rerun-if-changed={}", ttf_target.display());

    // Compute the path from the generated module's directory to the TTF.
    // Both inputs are relative to the crate root, so `relative_from` will
    // emit a correct `../..`-prefixed include path regardless of how deep
    // either destination is nested.
    let module_dir = module_target.parent().unwrap_or_else(|| Path::new(""));
    let ttf_rel = relative_from(module_dir, &ttf_target);

    let existing = fs::read_to_string(&module_target).unwrap_or_default();
    let existing_hash = existing
        .lines()
        .nth(2)
        .unwrap_or_default()
        .trim_start_matches("// ");

    if hash != existing_hash || !ttf_target.exists() {
        // Build a subset TTF with only the requested glyphs
        let codepoints: Vec<u32> = icons.values().copied().collect();
        let font_data = subset_font(&codepoints, definition.family.as_deref());

        if let Some(dir) = ttf_target.parent().filter(|p| !p.as_os_str().is_empty()) {
            fs::create_dir_all(dir)
                .unwrap_or_else(|e| panic!("Create TTF directory {}: {e}", dir.display()));
        }
        fs::write(&ttf_target, &font_data)
            .unwrap_or_else(|e| panic!("Write lucide.ttf to {}: {e}", ttf_target.display()));

        let module = generate_module(
            &icons,
            &hash,
            ttf_rel.to_string_lossy().replace('\\', "/"),
            definition.family.as_deref(),
        );

        if let Some(dir) = module_target.parent().filter(|p| !p.as_os_str().is_empty()) {
            fs::create_dir_all(dir)
                .unwrap_or_else(|e| panic!("Create module directory {}: {e}", dir.display()));
        }
        fs::write(&module_target, module).expect("Write icon module");
    }

    Ok(())
}

/// Compute the relative path from `base` to `target`.
///
/// Both arguments are treated as relative paths rooted at the same common
/// ancestor (typically the crate root during a `build.rs` invocation). The
/// returned path is the shortest sequence of `..` and forward components
/// that, when joined onto `base`, yields `target`.
///
/// - Leading `./` components are stripped.
/// - Inputs containing `..` are left unnormalized above the common prefix.
///   This helper is only intended for simple downward paths like
///   `examples/playground/icon.rs` ↔ `examples/playground/fonts/lucide.ttf`.
///
/// # Examples
///
/// ```ignore
/// // sibling directories
/// relative_from(Path::new("src"), Path::new("fonts/lucide.ttf"))
///     == PathBuf::from("../fonts/lucide.ttf");
///
/// // nested target under a shared prefix
/// relative_from(
///     Path::new("examples/playground"),
///     Path::new("examples/playground/fonts/lucide.ttf"),
/// ) == PathBuf::from("fonts/lucide.ttf");
/// ```
fn relative_from(base: &Path, target: &Path) -> PathBuf {
    fn components(path: &Path) -> Vec<&std::ffi::OsStr> {
        path.components()
            .filter_map(|c| match c {
                std::path::Component::Normal(part) => Some(part),
                // Drop leading `./` segments; leave everything else alone.
                std::path::Component::CurDir => None,
                std::path::Component::ParentDir
                | std::path::Component::RootDir
                | std::path::Component::Prefix(_) => Some(c.as_os_str()),
            })
            .collect()
    }

    let base_parts = components(base);
    let target_parts = components(target);

    // Skip the shared prefix.
    let mut i = 0;
    while i < base_parts.len() && i < target_parts.len() && base_parts[i] == target_parts[i] {
        i += 1;
    }

    let mut out = PathBuf::new();
    for _ in i..base_parts.len() {
        out.push("..");
    }
    for part in &target_parts[i..] {
        out.push(part);
    }

    if out.as_os_str().is_empty() {
        PathBuf::from(".")
    } else {
        out
    }
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

    let hash = compute_hash(&all_icons, None);

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
            None,
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
    /// Rust module path for the generated helper functions.
    ///
    /// Used as the `mod` name at call sites and — unless [`module_target`]
    /// is set — as the destination filename under `src/`.
    ///
    /// [`module_target`]: Definition::module_target
    module: String,
    /// Optional custom destination for the generated `.rs` file.
    ///
    /// When present, the path is taken verbatim (relative to the crate
    /// root / `CARGO_MANIFEST_DIR`) and parent directories are created
    /// automatically. When absent, the default `src/<module>.rs` layout is
    /// used, preserving backwards compatibility.
    #[serde(default)]
    module_target: Option<PathBuf>,
    /// Optional custom destination for the generated `lucide.ttf`.
    ///
    /// When present, the subsetted font is written to this path (relative
    /// to the crate root) and the `include_bytes!` call inside the
    /// generated module is rewritten accordingly. When absent the TTF is
    /// placed next to the TOML definition, matching the historical
    /// behavior.
    #[serde(default)]
    ttf_target: Option<PathBuf>,
    /// Custom family name to embed in the generated TTF's `name` table AND
    /// in the generated module's `render(...)` helper. When `None`, the
    /// upstream Lucide font's `name` table is preserved verbatim (matching
    /// historic behavior) and the render helper emits `.font("lucide")`.
    ///
    /// Set this to a unique-per-crate string when more than one crate in
    /// your workspace uses `iced_lucide` simultaneously. cosmic-text /
    /// fontdb resolve `iced::Font::with_name(...)` lookups via a single
    /// match per family name; two distinct subsets registered under the
    /// same `"lucide"` family collide and one shadows the other (the
    /// shadowed one renders as tofu).
    #[serde(default)]
    family: Option<String>,
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
///
/// When `family = Some(name)`, replaces the original `name` table with a
/// minimal one announcing the custom family — see [`build_name_table`] for
/// the byte layout. When `family = None`, the upstream Lucide font's name
/// table is preserved verbatim so the Lucide license metadata is carried
/// over into the subset.
fn subset_font(codepoints: &[u32], family: Option<&str>) -> Vec<u8> {
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

    let name_data = match family {
        // Custom family: emit a minimal `name` table announcing the new
        // identity. The upstream Lucide license records are dropped — the
        // caller is opting in to a custom family and is responsible for
        // attribution in their own metadata.
        Some(family) => build_name_table(family),
        // No custom family: preserve the original name table so Lucide's
        // license metadata is carried over (subsetter removes it).
        None => match extract_table(FONT_BYTES, b"name") {
            Some(data) => data,
            None => return with_cmap,
        },
    };

    inject_table(&with_cmap, b"name", &name_data)
}

/// Build a minimal OpenType `name` table (format 0) with four records:
///
/// - `name_id = 1` (Family)         = `family`
/// - `name_id = 2` (Subfamily)      = `"Regular"`
/// - `name_id = 4` (Full Name)      = `family`
/// - `name_id = 6` (PostScript)     = `family` with spaces and hyphens
///   replaced by `_` (PostScript names disallow both)
///
/// All four records are encoded for platform 3 (Windows) / encoding 1
/// (Unicode BMP) / language 0x0409 (en-US), with strings stored as
/// UTF-16 BE. cosmic-text / fontdb match by `name_id = 1` for the
/// `Family::Name(...)` lookup that `iced::Font::with_name` produces.
///
/// Byte layout (per the OpenType spec, "name — Naming Table", format 0):
///
/// ```text
/// name table header (6 bytes):
///   uint16  format        = 0
///   uint16  count         = number of name records
///   Offset16 stringOffset = byte offset from start of table to the
///                           string heap (header + count * 12)
///
/// name records (count * 12 bytes), sorted by
/// (platformID, encodingID, languageID, nameID):
///   uint16  platformID
///   uint16  encodingID
///   uint16  languageID
///   uint16  nameID
///   uint16  length         (in bytes)
///   Offset16 offset        (from start of string heap)
///
/// string heap: concatenated UTF-16 BE bodies referenced by the records.
/// ```
fn build_name_table(family: &str) -> Vec<u8> {
    // PostScript names: no spaces, no hyphens, no parens — replace with _.
    let postscript: String = family
        .chars()
        .map(|c| match c {
            ' ' | '-' | '(' | ')' | '[' | ']' | '{' | '}' | '<' | '>' | '/' | '%' => '_',
            other => other,
        })
        .collect();

    // (name_id, value) tuples, in (platformID, encodingID, languageID,
    // nameID) sort order — for our single platform/encoding/language tuple
    // that reduces to ascending name_id order.
    let records: [(u16, &str); 4] = [
        (1, family),
        (2, "Regular"),
        (4, family),
        (6, postscript.as_str()),
    ];

    // Encode each string as UTF-16 BE and gather (length, offset) pairs.
    let mut heap: Vec<u8> = Vec::new();
    let mut entries: Vec<(u16, u16, u16)> = Vec::with_capacity(records.len()); // (name_id, length, offset)
    for (name_id, value) in records.iter() {
        let offset: u16 = heap
            .len()
            .try_into()
            .expect("name table string heap fits in u16");
        for unit in value.encode_utf16() {
            heap.extend_from_slice(&unit.to_be_bytes());
        }
        let length: u16 = (heap.len() - offset as usize)
            .try_into()
            .expect("individual name record body fits in u16");
        entries.push((*name_id, length, offset));
    }

    let count: u16 = entries
        .len()
        .try_into()
        .expect("name record count fits in u16");
    let header_len: u16 = 6;
    let records_len: u16 = count * 12;
    let string_offset: u16 = header_len + records_len;

    let mut out = Vec::with_capacity(string_offset as usize + heap.len());

    // Header.
    out.extend_from_slice(&0u16.to_be_bytes()); // format
    out.extend_from_slice(&count.to_be_bytes()); // count
    out.extend_from_slice(&string_offset.to_be_bytes()); // stringOffset

    // Records — platform 3 / encoding 1 / language 0x0409 for all four.
    for (name_id, length, offset) in entries.iter() {
        out.extend_from_slice(&3u16.to_be_bytes()); // platformID = Windows
        out.extend_from_slice(&1u16.to_be_bytes()); // encodingID = Unicode BMP
        out.extend_from_slice(&0x0409u16.to_be_bytes()); // languageID = en-US
        out.extend_from_slice(&name_id.to_be_bytes());
        out.extend_from_slice(&length.to_be_bytes());
        out.extend_from_slice(&offset.to_be_bytes());
    }

    // String heap.
    out.extend_from_slice(&heap);

    out
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

/// SHA-256 hash of the sorted icon list and family name, returned as
/// a hex string.
///
/// `family` participates in the hash so that flipping or renaming the
/// custom family in the TOML invalidates the cached generated module
/// and TTF. Without it, a build that only changes `family` would skip
/// regeneration and ship a TTF with the old `name` table.
fn compute_hash(icons: &BTreeMap<String, u32>, family: Option<&str>) -> String {
    use sha2::Digest as _;
    let mut hasher = sha2::Sha256::new();
    for (name, code) in icons {
        hasher.update(name.as_bytes());
        hasher.update(b":");
        hasher.update(code.to_le_bytes());
        hasher.update(b"|");
    }
    if let Some(family) = family {
        hasher.update(b"family=");
        hasher.update(family.as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Render the Rust module source.
///
/// `family` selects the family-name string baked into the generated
/// `render(...)` helper: `Some(name)` produces `Text::new(cp).font(name)`,
/// `None` falls back to the historic `Text::new(cp).font("lucide")`.
fn generate_module(
    icons: &BTreeMap<String, u32>,
    hash: &str,
    ttf_path: String,
    family: Option<&str>,
) -> String {
    let family_literal = family.unwrap_or("lucide");
    let mut out = String::new();

    out.push_str(&format!(
        "// Generated automatically by iced_lucide at build time.\n\
         // Do not edit manually.\n\
         // {hash}\n\
         use iced::widget::text::{{self, Text}};\n\n\
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
            "pub fn {name}<'a, Theme>() -> Text<'a, Theme>\n\
             where\n    Theme: text::Catalog + 'a,\n\
             {{\n    icon(\"\\u{{{code:X}}}\")\n}}\n\n"
        ));
    }

    // Public render helper — for use with ALL_ICONS in picker widgets
    out.push_str(&format!(
        "/// Render any Lucide icon by its codepoint string.\n\
         /// Use this together with [`ALL_ICONS`] to display icons dynamically:\n\
         /// ```ignore\n\
         /// for (name, cp) in ALL_ICONS {{\n\
         ///     button(render(cp)).on_press(Msg::Pick(name.to_string()))\n\
         /// }}\n\
         /// ```\n\
         pub fn render<'a, Theme>(codepoint: &'a str) -> Text<'a, Theme>\n\
         where\n    Theme: text::Catalog + 'a,\n\
         {{\n    Text::new(codepoint).font(\"{family_literal}\")\n}}\n\n"
    ));

    // Private helper used by typed icon functions
    out.push_str(
        "fn icon<'a, Theme>(codepoint: &'a str) -> Text<'a, Theme>\n\
         where\n    Theme: text::Catalog + 'a,\n\
         {\n    render(codepoint)\n}\n",
    );

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
        let h1 = compute_hash(&icons, None);
        let h2 = compute_hash(&icons, None);
        assert_eq!(h1, h2);

        // Family participates: distinct family → distinct hash.
        let h_default = compute_hash(&icons, None);
        let h_custom = compute_hash(&icons, Some("granita-lucide"));
        assert_ne!(h_default, h_custom);

        // Same family → stable hash.
        let h_custom2 = compute_hash(&icons, Some("granita-lucide"));
        assert_eq!(h_custom, h_custom2);
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
    fn definition_parses_without_custom_paths() {
        let src = r#"
            module = "icon"
            [icons]
            edit = "pencil"
        "#;
        let def: Definition = toml::from_str(src).expect("parse");
        assert_eq!(def.module, "icon");
        assert!(def.module_target.is_none());
        assert!(def.ttf_target.is_none());
        assert_eq!(def.icons.get("edit").map(String::as_str), Some("pencil"));
    }

    #[test]
    fn definition_parses_with_custom_paths() {
        let src = r#"
            module        = "icon"
            module_target = "examples/playground/icon.rs"
            ttf_target    = "examples/playground/fonts/lucide.ttf"
            [icons]
            edit = "pencil"
        "#;
        let def: Definition = toml::from_str(src).expect("parse");
        assert_eq!(
            def.module_target,
            Some(PathBuf::from("examples/playground/icon.rs"))
        );
        assert_eq!(
            def.ttf_target,
            Some(PathBuf::from("examples/playground/fonts/lucide.ttf"))
        );
    }

    #[test]
    fn relative_from_sibling_directories() {
        // module in src/, font in fonts/ → "../fonts/lucide.ttf"
        assert_eq!(
            relative_from(Path::new("src"), Path::new("fonts/lucide.ttf")),
            PathBuf::from("../fonts/lucide.ttf")
        );
    }

    #[test]
    fn relative_from_shared_prefix() {
        // module + ttf under the same examples/playground/ parent
        assert_eq!(
            relative_from(
                Path::new("examples/playground"),
                Path::new("examples/playground/fonts/lucide.ttf"),
            ),
            PathBuf::from("fonts/lucide.ttf")
        );
    }

    #[test]
    fn relative_from_nested_module_to_root_ttf() {
        // src/icons/foo.rs ↔ lucide.ttf at root
        assert_eq!(
            relative_from(Path::new("src/icons"), Path::new("lucide.ttf")),
            PathBuf::from("../../lucide.ttf")
        );
    }

    #[test]
    fn relative_from_deep_module_to_deep_ttf() {
        // examples/playground/src/icon.rs ↔ assets/fonts/lucide.ttf
        assert_eq!(
            relative_from(
                Path::new("examples/playground/src"),
                Path::new("assets/fonts/lucide.ttf"),
            ),
            PathBuf::from("../../../assets/fonts/lucide.ttf")
        );
    }

    #[test]
    fn relative_from_identical_directories() {
        // module.parent() == ttf.parent() → same directory
        assert_eq!(
            relative_from(
                Path::new("examples/playground"),
                Path::new("examples/playground/lucide.ttf"),
            ),
            PathBuf::from("lucide.ttf")
        );
    }

    #[test]
    fn relative_from_empty_base() {
        // An empty base (module at crate root) just returns the target.
        assert_eq!(
            relative_from(Path::new(""), Path::new("fonts/lucide.ttf")),
            PathBuf::from("fonts/lucide.ttf")
        );
    }

    #[test]
    fn relative_from_strips_leading_cur_dir() {
        assert_eq!(
            relative_from(Path::new("./src"), Path::new("./fonts/lucide.ttf")),
            PathBuf::from("../fonts/lucide.ttf")
        );
    }

    #[test]
    fn relative_from_identical_paths_returns_dot() {
        // Same location → "." (not an empty PathBuf).
        assert_eq!(
            relative_from(Path::new("src/foo"), Path::new("src/foo")),
            PathBuf::from(".")
        );
    }

    #[test]
    fn subset_is_smaller_and_valid() {
        // A handful of icons — far fewer than the full 1685.
        let codepoints = [0xE001, 0xE002, 0xE003, 0xE004, 0xE005];
        let subsetted = subset_font(&codepoints, None);

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

    #[test]
    fn subset_with_custom_family_writes_name_table() {
        let codepoints = [0xE001, 0xE002, 0xE003];
        let subsetted = subset_font(&codepoints, Some("test-fam"));

        let face = ttf_parser::Face::parse(&subsetted, 0)
            .expect("ttf-parser parses subset with custom family");

        let names = face.names();
        let mut found_family = false;
        let mut found_postscript = false;
        for record in names {
            // ttf-parser only exposes Unicode-decodable records via `to_string()`.
            // Our writer emits Windows / Unicode BMP / en-US records, which it
            // decodes from UTF-16 BE.
            let Some(value) = record.to_string() else {
                continue;
            };
            match record.name_id {
                1 => {
                    assert_eq!(
                        value, "test-fam",
                        "name_id=1 (Family) must be the custom family"
                    );
                    found_family = true;
                }
                6 => {
                    assert_eq!(
                        value, "test_fam",
                        "name_id=6 (PostScript) must replace hyphens with underscores"
                    );
                    found_postscript = true;
                }
                _ => {}
            }
        }
        assert!(found_family, "subset must contain a Family name record");
        assert!(
            found_postscript,
            "subset must contain a PostScript name record"
        );
    }

    #[test]
    fn build_name_table_layout_matches_spec() {
        // Sanity-check the on-disk layout: header (6 bytes) + 4 records *
        // 12 bytes + UTF-16 BE string heap.
        let table = build_name_table("foo");

        let format = u16::from_be_bytes([table[0], table[1]]);
        let count = u16::from_be_bytes([table[2], table[3]]);
        let string_offset = u16::from_be_bytes([table[4], table[5]]);

        assert_eq!(format, 0, "format 0 selected");
        assert_eq!(count, 4, "Family / Subfamily / Full / PostScript");
        assert_eq!(
            string_offset as usize,
            6 + 4 * 12,
            "string heap follows header + records"
        );

        // First record: Family ("foo") — 6 bytes UTF-16 BE.
        let rec0 = 6;
        let plat = u16::from_be_bytes([table[rec0], table[rec0 + 1]]);
        let enc = u16::from_be_bytes([table[rec0 + 2], table[rec0 + 3]]);
        let lang = u16::from_be_bytes([table[rec0 + 4], table[rec0 + 5]]);
        let nid = u16::from_be_bytes([table[rec0 + 6], table[rec0 + 7]]);
        let len = u16::from_be_bytes([table[rec0 + 8], table[rec0 + 9]]);
        let off = u16::from_be_bytes([table[rec0 + 10], table[rec0 + 11]]);
        assert_eq!(plat, 3, "Windows platform");
        assert_eq!(enc, 1, "Unicode BMP");
        assert_eq!(lang, 0x0409, "en-US");
        assert_eq!(nid, 1, "first record is Family");
        assert_eq!(len, 6, "'foo' is 3 chars * 2 bytes UTF-16 BE");
        assert_eq!(off, 0, "first record's string starts at heap offset 0");

        // String heap starts at `string_offset` and the first 6 bytes
        // should be UTF-16 BE for "foo".
        let heap_start = string_offset as usize;
        assert_eq!(
            &table[heap_start..heap_start + 6],
            &[0x00, b'f', 0x00, b'o', 0x00, b'o'],
        );
    }
}
