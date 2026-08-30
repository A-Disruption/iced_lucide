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
    FlatJson,
    /// CSS rules of the form `.prefix-name:before { content: "\ea60" }`.
    Css { prefix: &'static str },
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
    /// Only worth it when a family is carved out of a much larger font: without
    /// this, taking 310 Octicons out of Symbols Nerd Font would vendor a second
    /// 2.5 MB copy of that font.
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
        metadata: Metadata::FlatJson,
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
        metadata: Metadata::Css { prefix: "codicon-" },
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
        metadata: Metadata::Css { prefix: "devicon-" },
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
        metadata: Metadata::Css { prefix: "pi-" },
        license_url: Some("https://raw.githubusercontent.com/gabrielelana/pomicons/master/LICENSE"),
        trim_to_index: false,
        note: None,
    },
];
