#![allow(clippy::needless_doctest_main)]
//! Compile-time, type-safe icon fonts for [`iced`].
//!
//! Pick the icons you want in a TOML file; this crate cuts a font containing
//! exactly those glyphs and generates a typed function for each one. A project
//! using a dozen icons ships a font holding a dozen glyphs, not the several
//! thousand its upstream publishes.
//!
//! No network calls: every supported font and its icon index are vendored into
//! the crate.
//!
//! [`iced`]: https://github.com/iced-rs/iced
//!
//! # Icon sets
//!
//! Each family lives behind its own Cargo feature, so a build embeds only the
//! fonts it asked for. `lucide` is the default.
//!
//! | Feature | Family | Identifier | License |
//! |---|---|---|---|
//! | `lucide` | [Lucide](https://lucide.dev/icons) | `lucide` | ISC |
//! | `bootstrap` | [Bootstrap Icons](https://icons.getbootstrap.com) | `bootstrap` | MIT |
//! | `codicon` | [VS Code Codicons](https://microsoft.github.io/vscode-codicons/dist/codicon.html) | `codicon` | CC-BY-4.0 |
//! | `devicon` | [Devicon](https://devicon.dev) | `devicon` | MIT |
//! | `fontawesome` | [Font Awesome Free](https://fontawesome.com/icons/packs/classic) | `fa-solid`, `fa-regular`, `fa-brands` | CC-BY-4.0 / OFL-1.1 |
//! | `nerdfonts` | [Nerd Fonts](https://www.nerdfonts.com) | `nerdfonts` | MIT |
//! | `octicons` | [Octicons](https://primer.style/octicons) (via the Nerd Fonts `oct-` range) | `octicons` | MIT |
//! | `pomicons` | [Pomicons](https://github.com/gabrielelana/pomicons) | `pomicons` | see vendored LICENSE |
//! | `material_symbols` | [Material Symbols](https://fonts.google.com/icons) | `material-symbols` | Apache-2.0 |
//! | `material_design_icons` | [Material Design Icons](https://pictogrammers.com/library/mdi/) | `material-design-icons` | Apache-2.0 |
//! | `phosphor` | [Phosphor](https://phosphoricons.com) | `phosphor` | MIT |
//! | `tabler` | [Tabler Icons](https://tabler.io/icons) | `tabler` | MIT |
//! | `fluent` | [Fluent System Icons](https://github.com/microsoft/fluentui-system-icons) | `fluent` | MIT |
//! | `simple_icons` | [Simple Icons](https://simpleicons.org) | `simple-icons` | CC0-1.0 |
//! | `boxicons` | [Boxicons](https://boxicons.com) | `boxicons` | MIT |
//!
//! Seventeen families, 52,977 icons. Several carry caveats — Material Symbols
//! is vendored from its variable font's default instance, Fluent names keep
//! their pixel size, Boxicons prefixes its solid and logo styles — which the
//! README sets out in full.
//!
//! Enable what you need:
//!
//! ```toml
//! [build-dependencies]
//! iced_lucide = { version = "0.2", features = ["bootstrap", "fontawesome"] }
//! ```
//!
//! # Usage
//!
//! Create a `.toml` file describing the icons you want:
//!
//! ```toml
//! # fonts/my-icons.toml
//! module = "icon"
//!
//! [icons]
//! edit   = "pencil"
//! save   = "save"
//! github = "fa-brands:github"
//!
//! [icons.bootstrap]
//! bluetooth = "bluetooth"
//! ```
//!
//! Each key is the Rust function name. Each value is the upstream icon name,
//! optionally prefixed with `family:`. Names under `[icons.<family>]` all come
//! from that family. Unprefixed names come from the `family` key if you set one,
//! and otherwise from Lucide.
//!
//! Call [`build`] from your build script:
//!
//! ```rust,no_run
//! pub fn main() {
//!     println!("cargo::rerun-if-changed=fonts/my-icons.toml");
//!     iced_lucide::build("fonts/my-icons.toml").expect("Build icon module");
//! }
//! ```
//!
//! This writes one subset `.ttf` per family used, next to the TOML, and
//! generates `src/icon.rs`.
//!
//! Register the fonts and use the generated functions:
//!
//! ```rust,ignore
//! mod icon;
//!
//! fn main() -> iced::Result {
//!     let mut app = iced::application(App::default, App::update, App::view);
//!
//!     for font in icon::FONTS {
//!         app = app.font(*font);
//!     }
//!
//!     app.run()
//! }
//!
//! fn view(&self) -> iced::Element<'_, ()> {
//!     iced::widget::row![icon::edit(), icon::save(), icon::github()].into()
//! }
//! ```
//!
//! # Icon pickers
//!
//! [`build_all`] generates a module holding an entire family, for picker
//! widgets and UI builders:
//!
//! ```rust,no_run
//! pub fn main() {
//!     iced_lucide::build_all("lucide", "icon").expect("Build all icons");
//! }
//! ```
//!
//! The module exports `ALL_ICONS: &[Icon]` alongside a `render` function:
//!
//! ```rust,ignore
//! for icon in icon::ALL_ICONS {
//!     button(icon::render(*icon)).on_press(Message::Pick(icon.name));
//! }
//! ```
//!
//! Note that this generates one function per icon, so pointing it at a very
//! large family — Nerd Fonts has almost eleven thousand glyphs — produces a
//! correspondingly large module and a slow compile.
//!
//! For a picker spanning several families, [`build_index`] generates the same
//! module without the per-icon functions:
//!
//! ```rust,no_run
//! pub fn main() {
//!     // An empty slice means every family the enabled features provide.
//!     iced_lucide::build_index(&[], "icon").expect("Build icon index");
//! }
//! ```
//!
//! Each `Icon` records the `family` it came from and the module exports
//! `FAMILIES`, which is what a filter UI wants.
//!
//! # Runtime enumeration
//!
//! Added as a regular dependency, the crate can enumerate what it carries
//! without generating anything:
//!
//! ```rust,ignore
//! for family in iced_lucide::families() {
//!     println!("{} ({})", family.label(), family.icons().len());
//! }
//! ```

mod codegen;
mod definition;
mod families;
mod otf;
mod subset;

use std::collections::BTreeMap;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use definition::{Definition, Resolved};

// ---------------------------------------------------------------------------
// Families
// ---------------------------------------------------------------------------

/// An icon set vendored into this crate.
///
/// Obtain one from [`families`] or [`family`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Family {
    pub(crate) id: &'static str,
    pub(crate) label: &'static str,
    pub(crate) feature: &'static str,
    pub(crate) font_family: &'static str,
    pub(crate) file_stem: &'static str,
    pub(crate) browse_url: &'static str,
    pub(crate) license: &'static str,
    pub(crate) font: &'static [u8],
    pub(crate) index: &'static str,
}

impl Family {
    /// The identifier used in icon definition files, e.g. `fa-brands`.
    pub fn id(&self) -> &'static str {
        self.id
    }

    /// The human-readable name, e.g. `Font Awesome Free (Brands)`.
    pub fn label(&self) -> &'static str {
        self.label
    }

    /// The Cargo feature that provides this set.
    ///
    /// Several families can share one: Font Awesome's three faces all arrive
    /// with `fontawesome`.
    pub fn feature(&self) -> &'static str {
        self.feature
    }

    /// The font family name a subset of this set is addressable by.
    ///
    /// This is what the generated code passes to `iced::Font::new`.
    pub fn font_family(&self) -> &'static str {
        self.font_family
    }

    /// Where to browse this icon set.
    pub fn browse_url(&self) -> &'static str {
        self.browse_url
    }

    /// The license the icons are published under.
    ///
    /// The full text is vendored beside the font, in `assets/<id>/LICENSE`.
    pub fn license(&self) -> &'static str {
        self.license
    }

    /// The complete, un-subsetted font.
    pub fn font_bytes(&self) -> &'static [u8] {
        self.font
    }

    /// Cut a font carrying only `codepoints`, addressable as [`font_family`].
    ///
    /// This is what [`build`] writes to disk. It is exposed for callers that
    /// assemble fonts themselves — an editor building a subset from a document's
    /// live icon usage, say — rather than from a definition file.
    ///
    /// [`font_family`]: Family::font_family
    pub fn subset(&self, codepoints: &[u32]) -> Vec<u8> {
        subset::subset(self.font, codepoints, self.font_family)
    }

    /// Every icon in the set as `(name, codepoint)`, sorted by name.
    pub fn icons(&self) -> Vec<(String, u32)> {
        self.entries()
            .map(|(name, codepoint)| (name.to_string(), codepoint))
            .collect()
    }

    /// The codepoint for an icon name, if the set has one.
    pub fn codepoint(&self, name: &str) -> Option<u32> {
        self.entries()
            .find(|(candidate, _)| *candidate == name)
            .map(|(_, codepoint)| codepoint)
    }

    /// Names that look like `name`, to help with a typo.
    fn suggest(&self, name: &str) -> Vec<String> {
        let mut scored: Vec<(usize, &str)> = self
            .entries()
            .filter_map(|(candidate, _)| {
                let score = similarity(name, candidate);
                (score > 0).then_some((score, candidate))
            })
            .collect();

        // Longest overlap first, then alphabetically so the output is stable.
        scored.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.cmp(b.1)));
        scored
            .into_iter()
            .take(5)
            .map(|(_, candidate)| candidate.to_string())
            .collect()
    }

    /// Walk the vendored index.
    ///
    /// The index is `name<TAB>hex` lines, sorted by name, with `#` comments
    /// recording where the assets came from.
    fn entries(&self) -> impl Iterator<Item = (&'static str, u32)> {
        self.index
            .lines()
            .filter(|line| !line.starts_with('#') && !line.trim().is_empty())
            .filter_map(|line| line.split_once('\t'))
            .filter_map(|(name, code)| {
                u32::from_str_radix(code.trim(), 16)
                    .ok()
                    .map(|codepoint| (name, codepoint))
            })
    }
}

/// Every icon family enabled by the current feature set.
pub fn families() -> Vec<&'static Family> {
    families::enabled()
}

/// Look up an enabled family by its identifier.
pub fn family(id: &str) -> Option<&'static Family> {
    families().into_iter().find(|family| family.id == id)
}

/// How much of `wanted` a candidate name shares, as a crude similarity score.
fn similarity(wanted: &str, candidate: &str) -> usize {
    if candidate.contains(wanted) || wanted.contains(candidate) {
        return wanted.len().min(candidate.len()) + 100;
    }

    // Fall back to the longest shared word.
    wanted
        .split(['-', '_'])
        .filter(|part| part.len() > 2)
        .filter(|part| candidate.contains(*part))
        .map(str::len)
        .max()
        .unwrap_or(0)
}

// ---------------------------------------------------------------------------
// Build-time API
// ---------------------------------------------------------------------------

/// Build a type-safe icon module from a TOML definition file.
///
/// Validates every requested icon against its family, writes one subset `.ttf`
/// per family used next to the TOML, and generates `src/{module}.rs`.
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

    let contents = fs::read_to_string(path).map_err(|source| Error::ReadDefinition {
        path: path.to_path_buf(),
        source,
    })?;

    let definition: Definition =
        toml::from_str(&contents).map_err(|source| Error::ParseDefinition {
            path: path.to_path_buf(),
            source: Box::new(source),
        })?;

    let icons = definition.resolve()?;

    // Fonts live beside the definition that asked for them.
    let directory = path.parent().unwrap_or(Path::new(".")).to_path_buf();

    generate(
        &definition.module,
        &icons,
        &directory,
        codegen::Functions::PerIcon,
    )
}

/// Generate a module containing **every** icon in a family.
///
/// Writes the family's font into `fonts/` and generates `src/{module}.rs`.
///
/// ```rust,no_run
/// pub fn main() {
///     iced_lucide::build_all("lucide", "icon").expect("Build all Lucide icons");
/// }
/// ```
///
/// The generated module exports `FONT`, `ALL_ICONS`, a `render` function for
/// picker widgets, and one typed function per icon. That last part makes this a
/// poor fit for the largest families: Nerd Fonts would generate almost eleven
/// thousand functions.
pub fn build_all(family: &str, module: &str) -> Result<(), Error> {
    let family = crate::family(family).ok_or_else(|| Error::UnknownFamily {
        id: family.to_string(),
        available: families().into_iter().map(Family::id).collect(),
    })?;

    let icons = every_icon_in(&[family]);

    generate(
        module,
        &icons,
        Path::new("fonts"),
        codegen::Functions::PerIcon,
    )
}

/// Generate a browsable index of whole families, without per-icon functions.
///
/// This is [`build_all`] for the case where naming each icon in Rust would be
/// absurd: an icon picker over everything this crate carries covers close to
/// twenty thousand glyphs, and wants to walk `ALL_ICONS` rather than call
/// `icon::house()`.
///
/// Pass an empty slice for every enabled family.
///
/// ```rust,no_run
/// pub fn main() {
///     iced_lucide::build_index(&[], "icon").expect("Build icon index");
/// }
/// ```
///
/// The generated module exports `FONTS`, `FAMILIES`, `LICENSES`, `ALL_ICONS`,
/// `render`, and `find` — everything a picker needs, and nothing else.
pub fn build_index(families: &[&str], module: &str) -> Result<(), Error> {
    let selected: Vec<&'static Family> = if families.is_empty() {
        crate::families()
    } else {
        families
            .iter()
            .map(|id| {
                crate::family(id).ok_or_else(|| Error::UnknownFamily {
                    id: (*id).to_string(),
                    available: crate::families().into_iter().map(Family::id).collect(),
                })
            })
            .collect::<Result<_, _>>()?
    };

    let icons = every_icon_in(&selected);

    generate(module, &icons, Path::new("fonts"), codegen::Functions::Omit)
}

/// Every icon in the given families, grouped by family and sorted by name.
fn every_icon_in(families: &[&'static Family]) -> Vec<Resolved> {
    families
        .iter()
        .flat_map(|family| {
            family
                .icons()
                .into_iter()
                .map(move |(name, codepoint)| Resolved {
                    function: sanitize_fn_name(&name),
                    icon: name,
                    family,
                    codepoint,
                })
        })
        .collect()
}

/// Write the subset fonts and the generated module.
fn generate(
    module: &str,
    icons: &[Resolved],
    font_directory: &Path,
    functions: codegen::Functions,
) -> Result<(), Error> {
    let hash = compute_hash(icons);

    let module_target = PathBuf::from("src")
        .join(module.replace("::", "/"))
        .with_extension("rs");

    // Re-run if the generated module is deleted or edited.
    println!("cargo::rerun-if-changed={}", module_target.display());

    // Relative path from the generated module back to the project root.
    let depth = module.split("::").count();
    let to_root: PathBuf = "../".repeat(depth).into();

    let used = codegen::used_families(icons);
    let bundled: Vec<codegen::Bundled> = used
        .iter()
        .map(|family| {
            let target = font_directory.join(format!("{}.ttf", family.file_stem));

            codegen::Bundled {
                family,
                path: to_root.join(&target).to_string_lossy().replace('\\', "/"),
            }
        })
        .collect();

    let fonts_present = used.iter().all(|family| {
        font_directory
            .join(format!("{}.ttf", family.file_stem))
            .exists()
    });

    let existing = fs::read_to_string(&module_target).unwrap_or_default();
    let existing_hash = existing
        .lines()
        .nth(2)
        .unwrap_or_default()
        .trim_start_matches("// ");

    if hash == existing_hash && fonts_present {
        return Ok(());
    }

    fs::create_dir_all(font_directory).map_err(|source| Error::Write {
        path: font_directory.to_path_buf(),
        source,
    })?;

    for family in &used {
        let codepoints: Vec<u32> = icons
            .iter()
            .filter(|icon| icon.family.id == family.id)
            .map(|icon| icon.codepoint)
            .collect();

        let font = subset::subset(family.font, &codepoints, family.font_family);
        let target = font_directory.join(format!("{}.ttf", family.file_stem));

        fs::write(&target, &font).map_err(|source| Error::Write {
            path: target,
            source,
        })?;
    }

    if let Some(parent) = module_target.parent() {
        fs::create_dir_all(parent).map_err(|source| Error::Write {
            path: parent.to_path_buf(),
            source,
        })?;
    }

    fs::write(
        &module_target,
        codegen::module(icons, &bundled, &hash, functions),
    )
    .map_err(|source| Error::Write {
        path: module_target,
        source,
    })?;

    Ok(())
}

// ---------------------------------------------------------------------------
// Runtime API
// ---------------------------------------------------------------------------

/// Every Lucide icon as `(name, codepoint)` pairs, sorted by name.
#[cfg(feature = "lucide")]
#[deprecated(since = "0.2.0", note = "use `family(\"lucide\").unwrap().icons()`")]
pub fn icons() -> Vec<(String, u32)> {
    families::LUCIDE.icons()
}

/// The raw bytes of the bundled Lucide font.
#[cfg(feature = "lucide")]
#[deprecated(
    since = "0.2.0",
    note = "use `family(\"lucide\").unwrap().font_bytes()`"
)]
pub const FONT_BYTES: &[u8] = families::LUCIDE.font;

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

/// Everything that can go wrong generating an icon module.
pub enum Error {
    ReadDefinition {
        path: PathBuf,
        source: std::io::Error,
    },
    ParseDefinition {
        path: PathBuf,
        source: Box<toml::de::Error>,
    },
    /// A family was named that no enabled feature provides.
    UnknownFamily {
        id: String,
        available: Vec<&'static str>,
    },
    /// An icon name is not in its family.
    UnknownIcon {
        family: &'static str,
        browse_url: &'static str,
        name: String,
        suggestions: Vec<String>,
    },
    /// An unqualified icon name with no way to tell which family it means.
    AmbiguousFamily {
        icon: String,
        available: Vec<&'static str>,
    },
    /// Two entries would generate the same function.
    DuplicateFunction(String),
    Write {
        path: PathBuf,
        source: std::io::Error,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ReadDefinition { path, source } => {
                write!(
                    f,
                    "Icon definition {} could not be read: {source}",
                    path.display()
                )
            }
            Error::ParseDefinition { path, source } => {
                write!(f, "Icon definition {} is invalid: {source}", path.display())
            }
            Error::UnknownFamily { id, available } => write!(
                f,
                "Unknown icon family \"{id}\".\n\
                 Enabled families: {}\n\
                 Others need their Cargo feature turned on.",
                available.join(", ")
            ),
            Error::UnknownIcon {
                family,
                browse_url,
                name,
                suggestions,
            } => {
                write!(f, "{family} has no icon \"{name}\".")?;

                if !suggestions.is_empty() {
                    write!(f, "\nDid you mean: {}?", suggestions.join(", "))?;
                }

                write!(f, "\nBrowse all icons at {browse_url}")
            }
            Error::AmbiguousFamily { icon, available } => write!(
                f,
                "\"{icon}\" does not say which family it comes from, and several \
                 are enabled: {}.\n\
                 Prefix the name (\"lucide:{icon}\"), group it under \
                 [icons.<family>], or set a top-level `family` key.",
                available.join(", ")
            ),
            Error::DuplicateFunction(name) => write!(
                f,
                "Two icons would both generate a function called \"{name}\". \
                 Rename one of them."
            ),
            Error::Write { path, source } => {
                write!(f, "Could not write {}: {source}", path.display())
            }
        }
    }
}

// A build script reports failure by unwrapping, which prints Debug. Showing the
// Display text there is the difference between a usable message and a wall of
// struct fields.
impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::ReadDefinition { source, .. } | Error::Write { source, .. } => Some(source),
            Error::ParseDefinition { source, .. } => Some(source),
            _ => None,
        }
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// The function name [`build`] will generate for an icon.
///
/// Definition files name their own functions, but a tool writing one — the
/// icon picker example does exactly this — needs to know what an icon will end
/// up being called before the build script runs.
///
/// ```rust
/// assert_eq!(iced_lucide::function_name("trash-2"), "trash_2");
/// assert_eq!(iced_lucide::function_name("move"), "move_icon");
/// ```
pub fn function_name(icon: &str) -> String {
    sanitize_fn_name(icon)
}

/// Convert an icon name to a valid Rust identifier.
///
/// - `pencil` → `pencil`
/// - `trash-2` → `trash_2`
/// - `3d-rotation` → `icon_3d_rotation`
/// - `move` → `move_icon` (Rust keyword)
/// - `render` → `render_icon` (the generated module defines its own)
fn sanitize_fn_name(name: &str) -> String {
    // Strict and reserved keywords across all Rust editions.
    const KEYWORDS: &[&str] = &[
        "abstract", "as", "async", "await", "become", "box", "break", "const", "continue", "crate",
        "do", "dyn", "else", "enum", "extern", "false", "final", "fn", "for", "if", "impl", "in",
        "let", "loop", "macro", "match", "mod", "move", "mut", "override", "priv", "pub", "ref",
        "return", "self", "static", "struct", "super", "trait", "true", "try", "type", "typeof",
        "unsafe", "unsized", "use", "virtual", "where", "while", "yield",
    ];

    // Functions the generated module defines for itself. Lucide really does
    // ship an icon called `text`, so this is not hypothetical.
    const RESERVED: &[&str] = &["render", "find", "text"];

    let mut out: String = name
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect();

    if out.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        out = format!("icon_{out}");
    }

    if KEYWORDS.contains(&out.as_str()) || RESERVED.contains(&out.as_str()) {
        out = format!("{out}_icon");
    }

    out
}

/// A hash of the resolved icon list, used to skip regeneration.
fn compute_hash(icons: &[Resolved]) -> String {
    use sha2::Digest as _;

    // Ordered so the hash does not depend on how the definition was written.
    let ordered: BTreeMap<(&str, &str, &str), &Resolved> = icons
        .iter()
        .map(|icon| {
            (
                (icon.family.id, icon.icon.as_str(), icon.function.as_str()),
                icon,
            )
        })
        .collect();

    let mut hasher = sha2::Sha256::new();

    // The hash decides whether an existing module can be left alone, so it has
    // to cover the generator as well as the icons: upgrading iced_lucide can
    // change the shape of the emitted module even when the selection has not
    // moved, and a stale module then fails to compile against the new one.
    hasher.update(env!("CARGO_PKG_VERSION").as_bytes());
    hasher.update(b"|");

    for ((_, _, function), icon) in ordered {
        hasher.update(function.as_bytes());
        hasher.update(b":");
        hasher.update(icon.family.id.as_bytes());
        hasher.update(b":");
        hasher.update(icon.icon.as_bytes());
        hasher.update(b":");
        hasher.update(icon.codepoint.to_le_bytes());
        hasher.update(b"|");
    }

    format!("{:x}", hasher.finalize())
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// Building with no family features at all is legal, if not much use.
    ///
    /// It is worth keeping working: `default-features = false` plus a single
    /// family is the normal way to ask for one set and not Lucide, and getting
    /// the feature name wrong should produce an empty registry rather than a
    /// compile error somewhere confusing.
    #[test]
    fn an_empty_feature_set_still_produces_a_registry() {
        assert_eq!(families().len(), families::enabled().len());

        for family in families() {
            assert!(!family.id().is_empty());
        }
    }

    #[test]
    fn every_family_has_a_usable_font_and_index() {
        for family in families() {
            assert!(
                ttf_parser::Face::parse(family.font_bytes(), 0).is_ok(),
                "{} does not carry a parseable font",
                family.id()
            );

            assert!(
                !family.icons().is_empty(),
                "{} has an empty icon index",
                family.id()
            );
        }
    }

    #[test]
    fn every_indexed_icon_resolves_to_a_glyph() {
        for family in families() {
            let face = ttf_parser::Face::parse(family.font_bytes(), 0).expect("parse font");

            for (name, codepoint) in family.icons() {
                let character = char::from_u32(codepoint)
                    .unwrap_or_else(|| panic!("{}:{name} has an invalid codepoint", family.id()));

                assert!(
                    face.glyph_index(character).is_some(),
                    "{}:{name} (U+{codepoint:04X}) is indexed but absent from the font",
                    family.id(),
                );
            }
        }
    }

    #[test]
    fn family_identifiers_are_unique() {
        let mut ids: Vec<&str> = families().into_iter().map(Family::id).collect();
        let count = ids.len();

        ids.sort_unstable();
        ids.dedup();

        assert_eq!(ids.len(), count, "family identifiers must be unique");
    }

    #[test]
    fn font_family_names_are_unique() {
        // Two families sharing a name would make Font::new ambiguous, which is
        // exactly the Font Awesome problem the name rewrite exists to solve.
        let mut names: Vec<&str> = families().into_iter().map(Family::font_family).collect();
        let count = names.len();

        names.sort_unstable();
        names.dedup();

        assert_eq!(names.len(), count, "font family names must be unique");
    }

    #[test]
    fn icons_are_sorted_by_name() {
        for family in families() {
            let names: Vec<String> = family.icons().into_iter().map(|(name, _)| name).collect();

            let mut sorted = names.clone();
            sorted.sort();

            assert_eq!(names, sorted, "{} index is not sorted", family.id());
        }
    }

    #[test]
    fn generated_function_names_are_valid_identifiers() {
        for family in families() {
            for (name, _) in family.icons() {
                let function = sanitize_fn_name(&name);

                assert!(
                    !function.is_empty()
                        && !function.chars().next().is_some_and(|c| c.is_ascii_digit())
                        && function
                            .chars()
                            .all(|c| c.is_ascii_alphanumeric() || c == '_'),
                    "{}:{name} sanitises to the invalid identifier {function:?}",
                    family.id(),
                );
            }
        }
    }

    #[test]
    fn sanitize_fn_name_works() {
        assert_eq!(sanitize_fn_name("pencil"), "pencil");
        assert_eq!(sanitize_fn_name("trash-2"), "trash_2");
        assert_eq!(sanitize_fn_name("a-arrow-down"), "a_arrow_down");
        assert_eq!(sanitize_fn_name("3d-rotation"), "icon_3d_rotation");
        assert_eq!(sanitize_fn_name("move"), "move_icon");
        assert_eq!(sanitize_fn_name("type"), "type_icon");
        assert_eq!(sanitize_fn_name("1password"), "icon_1password");
        assert_eq!(sanitize_fn_name("text"), "text_icon");
        assert_eq!(sanitize_fn_name("render"), "render_icon");
    }

    #[test]
    fn no_icon_generates_a_function_the_module_already_defines() {
        // Lucide's `text` icon once shadowed `iced::widget::text` and broke
        // every other generated function in the module.
        const OWN_ITEMS: &[&str] = &["render", "find", "text", "Icon", "Text", "Font"];

        for family in families() {
            for (name, _) in family.icons() {
                let function = sanitize_fn_name(&name);

                assert!(
                    !OWN_ITEMS.contains(&function.as_str()),
                    "{}:{name} generates `{function}`, which the module defines itself",
                    family.id(),
                );
            }
        }
    }

    #[test]
    fn suggestions_find_a_near_miss() {
        let Some(lucide) = family("lucide") else {
            return;
        };

        let suggestions = lucide.suggest("penci");

        assert!(
            suggestions.iter().any(|name| name == "pencil"),
            "expected 'pencil' among {suggestions:?}"
        );
    }

    #[test]
    fn hash_is_stable_and_order_independent() {
        let Some(family) = families().first().copied() else {
            return;
        };

        let icons = family.icons();
        let first = Resolved {
            function: "a".to_string(),
            icon: icons[0].0.clone(),
            family,
            codepoint: icons[0].1,
        };
        let second = Resolved {
            function: "b".to_string(),
            icon: icons[1].0.clone(),
            family,
            codepoint: icons[1].1,
        };

        let forwards = compute_hash(&[first.clone(), second.clone()]);
        let backwards = compute_hash(&[second, first]);

        assert_eq!(forwards, backwards);
    }
}
