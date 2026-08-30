//! Where each icon family comes from, and how to read its metadata.
//!
//! Every upstream publishes its name-to-codepoint mapping in a different
//! format — HTML, JSON, CSS, or a bespoke JSON schema — so each family names
//! the parser it needs. Everything downstream of this file works on the
//! normalised `(name, codepoint)` pairs those parsers produce.

/// How the upstream metadata for a family is encoded.
pub enum Metadata {
    /// Lucide's `unicode.html`: `<h4>name</h4> ... &amp;#57347;`
    LucideHtml,
    /// A flat `{"icon-name": 61697}` JSON object.
    ///
    /// Some publishers namespace every key and tack the style on the end —
    /// Fluent writes `ic_fluent_access_time_24_regular` — so both ends can be
    /// trimmed off. Underscores become hyphens either way.
    FlatJson {
        strip_prefix: Option<&'static str>,
        strip_suffix: Option<&'static str>,
    },
    /// Whitespace-separated `name codepoint` lines, as Google publishes
    /// alongside the Material Symbols fonts.
    Codepoints,
    /// CSS rules of the form `.prefix-name:before { content: "\ea60" }`.
    ///
    /// Each entry pairs the class prefix to look for with the prefix to give
    /// the resulting name. Families that ship several styles in one font need
    /// this: Boxicons writes `.bx-`, `.bxs-`, and `.bxl-` for its regular,
    /// solid, and logo sets, which would otherwise collide on the bare name.
    Css {
        prefixes: &'static [(&'static str, &'static str)],
    },
    /// Font Awesome's `metadata/icons.json`, filtered to one style.
    FontAwesome { style: &'static str },
    /// Nerd Fonts' `glyphnames.json`.
    ///
    /// Nerd Fonts namespaces every glyph by the set it came from (`oct-`,
    /// `cod-`, `dev-`, …). With a `prefix`, only that set is kept and the
    /// prefix is stripped, which turns one aggregate font into a single family.
    NerdFonts { prefix: Option<&'static str> },
}

/// How the upstream font binary is packaged.
pub enum Container {
    /// A plain `.ttf` / `.otf`.
    Sfnt,
    /// WOFF 1.0, which wraps sfnt tables in per-table zlib.
    Woff,
    /// A zip archive; the font is the named entry inside it.
    Zip { entry: &'static str },
}

/// One font file, its icon set, and the provenance we record alongside them.
pub struct Source {
    /// Directory under `assets/`, and the identifier used in icon TOML files.
    pub id: &'static str,
    /// Human-readable name, used in error messages and generated docs.
    pub label: &'static str,
    /// Cargo feature that enables this family.
    pub feature: &'static str,
    /// The family name written into the subset's `name` table. This is the
    /// string the generated code passes to `iced::Font::new`.
    pub font_family: &'static str,
    /// Basename of the `.ttf` written into the consuming project.
    pub file_stem: &'static str,
    /// Where a human browses this icon set.
    pub browse_url: &'static str,
    /// SPDX-ish summary. The full text is vendored beside the font.
    pub license: &'static str,
    pub font_url: &'static str,
    pub container: Container,
    pub metadata_url: &'static str,
    pub metadata: Metadata,
    pub license_url: Option<&'static str>,
    /// Cut the vendored font down to the glyphs its index names.
    ///
    /// Two things make this worth doing. A family carved out of a larger font
    /// needs it to exist at all: taking 310 Octicons out of Symbols Nerd Font
    /// would otherwise vendor a second 2.5 MB copy. And it drops the layout
    /// tables nothing here uses — glyphs are always addressed by codepoint, so
    /// ligatures and kerning are dead weight. That is what takes Material
    /// Symbols from a 10.6 MB variable font to 805 KB.
    ///
    /// It is not a free win. Rebuilding the character map as format 12 costs
    /// twelve bytes per glyph, which for a font with a dense format 4 map and
    /// little layout data to shed — Nerd Fonts, Devicon, Boxicons — comes out
    /// larger than it started. Measure before turning it on.
    pub trim_to_index: bool,
    /// Extra caveats recorded in the vendored index and the generated docs.
    pub note: Option<&'static str>,
}

pub const SOURCES: &[Source] = &[
    Source {
        id: "lucide",
        label: "Lucide",
        feature: "lucide",
        font_family: "lucide",
        file_stem: "lucide",
        browse_url: "https://lucide.dev/icons",
        license: "ISC",
        font_url: "https://cdn.jsdelivr.net/npm/lucide-static/font/lucide.ttf",
        container: Container::Sfnt,
        metadata_url: "https://cdn.jsdelivr.net/npm/lucide-static/font/unicode.html",
        metadata: Metadata::LucideHtml,
        license_url: Some("https://raw.githubusercontent.com/lucide-icons/lucide/main/LICENSE"),
        trim_to_index: false,
        note: None,
    },
    Source {
        id: "bootstrap",
        label: "Bootstrap Icons",
        feature: "bootstrap",
        font_family: "bootstrap-icons",
        file_stem: "bootstrap-icons",
        browse_url: "https://icons.getbootstrap.com",
        license: "MIT",
        font_url: "https://cdn.jsdelivr.net/npm/bootstrap-icons/font/fonts/bootstrap-icons.woff",
        container: Container::Woff,
        metadata_url: "https://cdn.jsdelivr.net/npm/bootstrap-icons/font/bootstrap-icons.json",
        metadata: Metadata::FlatJson {
            strip_prefix: None,
            strip_suffix: None,
        },
        license_url: Some("https://raw.githubusercontent.com/twbs/icons/main/LICENSE"),
        trim_to_index: false,
        note: Some(
            "Bootstrap publishes WOFF and WOFF2 only. The vendored .ttf is decoded \
             from the WOFF build at vendoring time; the glyph outlines are unchanged.",
        ),
    },
    Source {
        id: "codicon",
        label: "VS Code Codicons",
        feature: "codicon",
        font_family: "codicon",
        file_stem: "codicon",
        browse_url: "https://microsoft.github.io/vscode-codicons/dist/codicon.html",
        license: "CC-BY-4.0",
        font_url: "https://cdn.jsdelivr.net/npm/@vscode/codicons/dist/codicon.ttf",
        container: Container::Sfnt,
        metadata_url: "https://cdn.jsdelivr.net/npm/@vscode/codicons/dist/codicon.css",
        metadata: Metadata::Css {
            prefixes: &[("codicon-", "")],
        },
        license_url: Some(
            "https://raw.githubusercontent.com/microsoft/vscode-codicons/main/LICENSE",
        ),
        trim_to_index: false,
        note: Some("Several names are aliases that share a codepoint; all are kept."),
    },
    Source {
        id: "devicon",
        label: "Devicon",
        feature: "devicon",
        font_family: "devicon",
        file_stem: "devicon",
        browse_url: "https://devicon.dev",
        license: "MIT",
        font_url: "https://cdn.jsdelivr.net/npm/devicon/fonts/devicon.ttf",
        container: Container::Sfnt,
        // The package root also ships a `devicon.css`, but it carries only the
        // most recent additions — the minified build is the complete set.
        metadata_url: "https://cdn.jsdelivr.net/npm/devicon/devicon.min.css",
        metadata: Metadata::Css {
            prefixes: &[("devicon-", "")],
        },
        license_url: Some("https://raw.githubusercontent.com/devicons/devicon/master/LICENSE"),
        trim_to_index: false,
        note: Some(
            "Devicon's multi-colour 'original' variants exist only as SVG. \
             The font carries the single-colour plain/line variants.",
        ),
    },
    Source {
        id: "fa-solid",
        label: "Font Awesome Free (Solid)",
        feature: "fontawesome",
        font_family: "fa-solid",
        file_stem: "fa-solid-900",
        browse_url: "https://fontawesome.com/icons/packs/classic",
        license: "CC-BY-4.0 (icons), OFL-1.1 (fonts)",
        font_url: "https://cdn.jsdelivr.net/npm/@fortawesome/fontawesome-free/webfonts/fa-solid-900.ttf",
        container: Container::Sfnt,
        metadata_url: "https://raw.githubusercontent.com/FortAwesome/Font-Awesome/7.x/metadata/icons.json",
        metadata: Metadata::FontAwesome { style: "solid" },
        license_url: Some(
            "https://raw.githubusercontent.com/FortAwesome/Font-Awesome/7.x/LICENSE.txt",
        ),
        trim_to_index: false,
        note: Some(
            "Upstream names both the solid and regular faces \"Font Awesome 7 Free\", \
             separating them only by weight. Each subset is renamed so it can be \
             addressed unambiguously by family.",
        ),
    },
    Source {
        id: "fa-regular",
        label: "Font Awesome Free (Regular)",
        feature: "fontawesome",
        font_family: "fa-regular",
        file_stem: "fa-regular-400",
        browse_url: "https://fontawesome.com/icons/packs/classic",
        license: "CC-BY-4.0 (icons), OFL-1.1 (fonts)",
        font_url: "https://cdn.jsdelivr.net/npm/@fortawesome/fontawesome-free/webfonts/fa-regular-400.ttf",
        container: Container::Sfnt,
        metadata_url: "https://raw.githubusercontent.com/FortAwesome/Font-Awesome/7.x/metadata/icons.json",
        metadata: Metadata::FontAwesome { style: "regular" },
        license_url: Some(
            "https://raw.githubusercontent.com/FortAwesome/Font-Awesome/7.x/LICENSE.txt",
        ),
        trim_to_index: false,
        note: None,
    },
    Source {
        id: "fa-brands",
        label: "Font Awesome Free (Brands)",
        feature: "fontawesome",
        font_family: "fa-brands",
        file_stem: "fa-brands-400",
        browse_url: "https://fontawesome.com/icons/packs/classic",
        license: "CC-BY-4.0 (icons), OFL-1.1 (fonts)",
        font_url: "https://cdn.jsdelivr.net/npm/@fortawesome/fontawesome-free/webfonts/fa-brands-400.ttf",
        container: Container::Sfnt,
        metadata_url: "https://raw.githubusercontent.com/FortAwesome/Font-Awesome/7.x/metadata/icons.json",
        metadata: Metadata::FontAwesome { style: "brands" },
        license_url: Some(
            "https://raw.githubusercontent.com/FortAwesome/Font-Awesome/7.x/LICENSE.txt",
        ),
        trim_to_index: false,
        note: Some(
            "Brand marks are trademarks of their owners. Font Awesome's license \
             does not grant any right to use them.",
        ),
    },
    Source {
        id: "nerdfonts",
        label: "Nerd Fonts (Symbols Only)",
        feature: "nerdfonts",
        font_family: "nerd-symbols",
        file_stem: "symbols-nerd-font",
        browse_url: "https://www.nerdfonts.com/cheat-sheet",
        license: "MIT",
        font_url: "https://github.com/ryanoasis/nerd-fonts/releases/latest/download/NerdFontsSymbolsOnly.zip",
        container: Container::Zip {
            entry: "SymbolsNerdFont-Regular.ttf",
        },
        metadata_url: "https://raw.githubusercontent.com/ryanoasis/nerd-fonts/master/glyphnames.json",
        metadata: Metadata::NerdFonts { prefix: None },
        license_url: None,
        trim_to_index: false,
        note: Some(
            "Symbols Nerd Font aggregates glyphs from many upstream sets, each under \
             its own license. Names keep their upstream prefix (cod-, dev-, fa-, \
             oct-, pom-, md-, weather-, and so on).",
        ),
    },
    Source {
        id: "octicons",
        label: "Octicons",
        feature: "octicons",
        font_family: "octicons",
        file_stem: "octicons",
        browse_url: "https://primer.style/octicons",
        license: "MIT",
        // Primer removed the icon font at Octicons v9, so there is no current
        // upstream build to vendor. Nerd Fonts still carries the set in its
        // `oct-` range and tracks a much later version than the last standalone
        // font did (310 icons against 172), so the glyphs are taken from there
        // and cut back out into a font of their own.
        font_url: "https://github.com/ryanoasis/nerd-fonts/releases/latest/download/NerdFontsSymbolsOnly.zip",
        container: Container::Zip {
            entry: "SymbolsNerdFont-Regular.ttf",
        },
        metadata_url: "https://raw.githubusercontent.com/ryanoasis/nerd-fonts/master/glyphnames.json",
        metadata: Metadata::NerdFonts {
            prefix: Some("oct-"),
        },
        license_url: Some("https://raw.githubusercontent.com/primer/octicons/main/LICENSE"),
        trim_to_index: true,
        note: Some(
            "Taken from the Nerd Fonts `oct-` range rather than a Primer release: \
             Primer removed the icon font at Octicons v9 and publishes SVG only. \
             Names are normalised to Primer's spelling, so `arrow-down` rather than \
             the `oct-arrow_down` Nerd Fonts uses.",
        ),
    },
    Source {
        id: "pomicons",
        label: "Pomicons",
        feature: "pomicons",
        font_family: "pomicons",
        file_stem: "pomicons",
        browse_url: "https://github.com/gabrielelana/pomicons",
        license: "See vendored LICENSE",
        font_url: "https://raw.githubusercontent.com/gabrielelana/pomicons/master/fonts/Pomicons.ttf",
        container: Container::Sfnt,
        metadata_url: "https://raw.githubusercontent.com/gabrielelana/pomicons/master/css/pomicons.css",
        metadata: Metadata::Css {
            prefixes: &[("pi-", "")],
        },
        license_url: Some("https://raw.githubusercontent.com/gabrielelana/pomicons/master/LICENSE"),
        trim_to_index: false,
        note: None,
    },
    Source {
        id: "material-symbols",
        label: "Material Symbols",
        feature: "material_symbols",
        font_family: "material-symbols",
        file_stem: "material-symbols-outlined",
        browse_url: "https://fonts.google.com/icons",
        license: "Apache-2.0",
        // Google publishes Material Symbols only as a four-axis variable font.
        // Trimming resolves it to its default instance — weight 400, unfilled —
        // which is the shape the static builds carry anyway.
        font_url: "https://raw.githubusercontent.com/google/material-design-icons/master/variablefont/MaterialSymbolsOutlined%5BFILL%2CGRAD%2Copsz%2Cwght%5D.ttf",
        container: Container::Sfnt,
        metadata_url: "https://raw.githubusercontent.com/google/material-design-icons/master/variablefont/MaterialSymbolsOutlined%5BFILL%2CGRAD%2Copsz%2Cwght%5D.codepoints",
        metadata: Metadata::Codepoints,
        license_url: Some(
            "https://raw.githubusercontent.com/google/material-design-icons/master/LICENSE",
        ),
        trim_to_index: true,
        note: Some(
            "The Outlined style at weight 400. Upstream is a variable font whose other \
             axes — fill, grade, optical size — do not survive vendoring.",
        ),
    },
    Source {
        id: "material-design-icons",
        label: "Material Design Icons",
        feature: "material_design_icons",
        font_family: "material-design-icons",
        file_stem: "materialdesignicons",
        browse_url: "https://pictogrammers.com/library/mdi/",
        license: "Apache-2.0",
        font_url: "https://cdn.jsdelivr.net/npm/@mdi/font/fonts/materialdesignicons-webfont.ttf",
        container: Container::Sfnt,
        metadata_url: "https://cdn.jsdelivr.net/npm/@mdi/font/css/materialdesignicons.css",
        metadata: Metadata::Css {
            prefixes: &[("mdi-", "")],
        },
        license_url: Some(
            "https://raw.githubusercontent.com/Templarian/MaterialDesign-Webfont/master/LICENSE",
        ),
        trim_to_index: true,
        note: Some("The Pictogrammers set, distinct from Google's Material Symbols."),
    },
    Source {
        id: "phosphor",
        label: "Phosphor",
        feature: "phosphor",
        font_family: "phosphor",
        file_stem: "phosphor",
        browse_url: "https://phosphoricons.com",
        license: "MIT",
        font_url: "https://cdn.jsdelivr.net/npm/@phosphor-icons/web/src/regular/Phosphor.ttf",
        container: Container::Sfnt,
        metadata_url: "https://cdn.jsdelivr.net/npm/@phosphor-icons/web/src/regular/style.css",
        metadata: Metadata::Css {
            prefixes: &[("ph-", "")],
        },
        license_url: Some("https://raw.githubusercontent.com/phosphor-icons/web/master/LICENSE"),
        trim_to_index: true,
        note: Some(
            "The Regular weight. Phosphor's other weights are separate fonts; adding one \
             is a matter of another entry here.",
        ),
    },
    Source {
        id: "tabler",
        label: "Tabler Icons",
        feature: "tabler",
        font_family: "tabler-icons",
        file_stem: "tabler-icons",
        browse_url: "https://tabler.io/icons",
        license: "MIT",
        font_url: "https://cdn.jsdelivr.net/npm/@tabler/icons-webfont/dist/fonts/tabler-icons.ttf",
        container: Container::Sfnt,
        metadata_url: "https://cdn.jsdelivr.net/npm/@tabler/icons-webfont/dist/tabler-icons.css",
        metadata: Metadata::Css {
            prefixes: &[("ti-", "")],
        },
        license_url: Some("https://raw.githubusercontent.com/tabler/tabler-icons/main/LICENSE"),
        trim_to_index: true,
        note: None,
    },
    Source {
        id: "fluent",
        label: "Fluent System Icons",
        feature: "fluent",
        font_family: "fluent-system-icons",
        file_stem: "fluent-system-icons",
        browse_url: "https://github.com/microsoft/fluentui-system-icons",
        license: "MIT",
        font_url: "https://raw.githubusercontent.com/microsoft/fluentui-system-icons/main/fonts/FluentSystemIcons-Regular.ttf",
        container: Container::Sfnt,
        metadata_url: "https://raw.githubusercontent.com/microsoft/fluentui-system-icons/main/fonts/FluentSystemIcons-Regular.json",
        metadata: Metadata::FlatJson {
            strip_prefix: Some("ic_fluent_"),
            strip_suffix: Some("_regular"),
        },
        license_url: Some(
            "https://raw.githubusercontent.com/microsoft/fluentui-system-icons/main/LICENSE",
        ),
        trim_to_index: true,
        note: Some(
            "The Regular style. Fluent draws each icon separately per size, so names keep \
             their pixel size: `access-time-24` rather than `access-time`.",
        ),
    },
    Source {
        id: "simple-icons",
        label: "Simple Icons",
        feature: "simple_icons",
        font_family: "simple-icons",
        file_stem: "simple-icons",
        browse_url: "https://simpleicons.org",
        license: "CC0-1.0",
        font_url: "https://cdn.jsdelivr.net/npm/simple-icons-font/font/SimpleIcons.ttf",
        container: Container::Sfnt,
        metadata_url: "https://cdn.jsdelivr.net/npm/simple-icons-font/font/simple-icons.css",
        metadata: Metadata::Css {
            prefixes: &[("si-", "")],
        },
        license_url: Some(
            "https://raw.githubusercontent.com/simple-icons/simple-icons-font/develop/LICENSE.md",
        ),
        trim_to_index: true,
        note: Some(
            "Brand marks. The font is CC0, but the logos are trademarks of their owners \
             and the license grants no right to use them.",
        ),
    },
    Source {
        id: "boxicons",
        label: "Boxicons",
        feature: "boxicons",
        font_family: "boxicons",
        file_stem: "boxicons",
        browse_url: "https://boxicons.com",
        license: "MIT",
        font_url: "https://cdn.jsdelivr.net/npm/boxicons/fonts/boxicons.ttf",
        container: Container::Sfnt,
        metadata_url: "https://cdn.jsdelivr.net/npm/boxicons/css/boxicons.css",
        // Three styles share one font, separated only by class prefix.
        metadata: Metadata::Css {
            prefixes: &[("bx-", ""), ("bxs-", "solid-"), ("bxl-", "logo-")],
        },
        license_url: Some("https://raw.githubusercontent.com/atisawd/boxicons/master/LICENSE"),
        trim_to_index: false,
        note: Some(
            "The regular, solid, and logo styles share one font; solid and logo names are \
             prefixed `solid-` and `logo-` so they do not collide.",
        ),
    },
];
