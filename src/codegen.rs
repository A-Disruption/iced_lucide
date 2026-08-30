//! Rendering the generated Rust module.

use std::collections::BTreeMap;

use crate::Family;
use crate::definition::Resolved;

/// Everything the generated module needs to know about one font it uses.
pub struct Bundled {
    pub family: &'static Family,
    /// Path to the subset `.ttf`, relative to the generated module.
    pub path: String,
}

/// Whether the module gets a typed function for every icon.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Functions {
    /// One `pub fn` per icon, the point of a hand-written definition.
    PerIcon,
    /// None. A picker over every family would otherwise generate twenty
    /// thousand functions to call `render` in a loop instead.
    Omit,
}

/// Render the module source.
pub fn module(icons: &[Resolved], bundled: &[Bundled], hash: &str, functions: Functions) -> String {
    let mut out = String::new();

    out.push_str(&header(bundled, hash));
    out.push_str(&fonts(bundled));
    out.push_str(&icon_type());
    out.push_str(&all_icons(icons));

    if functions == Functions::PerIcon {
        out.push_str(&per_icon_functions(icons));
    }

    out.push_str(HELPERS);

    out
}

/// The provenance banner.
///
/// The hash on the third line is what [`crate::build`] reads back to decide
/// whether anything needs regenerating, so its position is load-bearing.
fn header(bundled: &[Bundled], hash: &str) -> String {
    let mut out = String::new();

    out.push_str(
        "// Generated automatically by iced_lucide at build time.\n\
         // Do not edit manually.\n",
    );
    out.push_str(&format!("// {hash}\n\n"));

    out.push_str("//! Icon fonts for this crate.\n//!\n");
    out.push_str("//! Each font here is a subset containing only the icons this\n");
    out.push_str("//! project asks for, cut from these upstream sets:\n//!\n");

    for Bundled { family, .. } in bundled {
        out.push_str(&format!(
            "//! - **{label}** — {license} — <{url}>\n",
            label = family.label(),
            license = family.license(),
            url = family.browse_url(),
        ));
    }

    out.push_str("//!\n//! Redistributing these fonts means carrying their licenses with you.\n\n");

    // A generated module is rarely exercised in full by the crate that owns it.
    out.push_str("#![allow(dead_code)]\n\n");
    // `text` is deliberately not imported: Lucide ships an icon by that name,
    // and a generated `fn text()` would shadow the widget constructor.
    out.push_str("use iced::Font;\nuse iced::widget::Text;\n\n");

    out
}

/// Font byte constants, the `Font` handles, and the registration list.
fn fonts(bundled: &[Bundled]) -> String {
    let mut out = String::new();

    for Bundled { family, path } in bundled {
        out.push_str(&format!(
            "/// {label} subset — {license}.\n\
             pub const {constant}: &[u8] = include_bytes!({path:?});\n\n",
            label = family.label(),
            license = family.license(),
            constant = bytes_constant(family),
            path = path,
        ));
    }

    out.push_str(
        "/// Every font this module needs.\n\
         ///\n\
         /// Register all of them before rendering any icon:\n\
         ///\n\
         /// ```ignore\n\
         /// let mut app = iced::application(App::new, App::update, App::view);\n\
         ///\n\
         /// for font in icon::FONTS {\n\
         ///     app = app.font(*font);\n\
         /// }\n\
         ///\n\
         /// app.run()\n\
         /// ```\n\
         // One entry per line stays readable as the list grows, and keeps a\n\
         // `cargo fmt` in the consuming crate from rewriting generated output.\n\
         #[rustfmt::skip]\n\
         pub const FONTS: &[&[u8]] = &[\n",
    );

    for Bundled { family, .. } in bundled {
        out.push_str(&format!("    {},\n", bytes_constant(family)));
    }

    out.push_str("];\n\n");

    // Modules that use exactly one family keep the singular spelling, both
    // because it reads better and because it is what earlier versions emitted.
    if let [only] = bundled {
        out.push_str(&format!(
            "/// The single font this module uses.\n\
             pub const FONT: &[u8] = {};\n\n",
            bytes_constant(only.family),
        ));
    }

    out.push_str(FAMILY_TYPE);

    out.push_str(
        "/// The icon sets this module draws from.\n\
         ///\n\
         /// [`Icon::family`] holds an `id` from this list, so it is both what a\n\
         /// filter UI wants and where an \"about\" screen finds its attribution.\n\
         #[rustfmt::skip]\n\
         pub const FAMILIES: &[Family] = &[\n",
    );
    for Bundled { family, .. } in bundled {
        out.push_str(&format!(
            "    Family {{ id: {id:?}, name: {name:?}, feature: {feature:?}, \
             license: {license:?}, url: {url:?} }},\n",
            id = family.id(),
            name = family.label(),
            feature = family.feature(),
            license = family.license(),
            url = family.browse_url(),
        ));
    }
    out.push_str("];\n\n");

    for Bundled { family, .. } in bundled {
        out.push_str(&format!(
            "const {handle}: Font = Font::new({name:?});\n",
            handle = font_handle(family),
            name = family.font_family(),
        ));
    }
    out.push('\n');

    out
}

const FAMILY_TYPE: &str = "\
/// One of the icon sets this module draws from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Family {
    /// Identifier, matching [`Icon::family`].
    pub id: &'static str,
    /// Human-readable name.
    pub name: &'static str,
    /// The `iced_lucide` Cargo feature that provides this set.
    pub feature: &'static str,
    /// License the icons are published under.
    pub license: &'static str,
    /// Where to browse the set.
    pub url: &'static str,
}

";

fn icon_type() -> String {
    "\
/// An icon: its name, the character that draws it, and the font that carries it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Icon {
    /// The upstream icon name.
    pub name: &'static str,
    /// The character to render.
    pub codepoint: &'static str,
    /// The font the character belongs to.
    pub font: Font,
    /// Identifier of the set this icon came from, matching [`FAMILIES`].
    pub family: &'static str,
}

"
    .to_string()
}

fn all_icons(icons: &[Resolved]) -> String {
    let mut out = String::new();

    out.push_str(
        "/// Every icon in this module.\n\
         ///\n\
         /// Use this with [`render`] to build a picker:\n\
         ///\n\
         /// ```ignore\n\
         /// for icon in ALL_ICONS {\n\
         ///     button(render(*icon)).on_press(Message::Pick(icon.name))\n\
         /// }\n\
         /// ```\n\
         // One icon per line; rustfmt would otherwise explode each struct\n\
         // literal across five lines and make this unreadable.\n\
         #[rustfmt::skip]\n\
         pub const ALL_ICONS: &[Icon] = &[\n",
    );

    for icon in icons {
        out.push_str(&format!(
            "    Icon {{ name: {name:?}, codepoint: \"\\u{{{code:X}}}\", \
             font: {handle}, family: {family:?} }},\n",
            name = icon.icon,
            code = icon.codepoint,
            handle = font_handle(icon.family),
            family = icon.family.id(),
        ));
    }

    out.push_str("];\n\n");

    out
}

fn per_icon_functions(icons: &[Resolved]) -> String {
    let mut out = String::new();

    for icon in icons {
        out.push_str(&format!(
            "\
/// `{name}` from {label}.
pub fn {function}<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{{
    iced::widget::text(\"\\u{{{code:X}}}\").font({handle})
}}\n\n",
            name = icon.icon,
            label = icon.family.label(),
            function = icon.function,
            code = icon.codepoint,
            handle = font_handle(icon.family),
        ));
    }

    out
}

const HELPERS: &str = "\
/// Render any icon from [`ALL_ICONS`].
pub fn render<'a, Theme, Renderer>(icon: Icon) -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text(icon.codepoint).font(icon.font)
}

/// Look an icon up by its upstream name.
pub fn find(name: &str) -> Option<Icon> {
    ALL_ICONS.iter().find(|icon| icon.name == name).copied()
}
";

/// `fa-solid` becomes `FONT_FA_SOLID`.
fn bytes_constant(family: &Family) -> String {
    format!("FONT_{}", family.id().replace('-', "_").to_uppercase())
}

/// `fa-solid` becomes `FA_SOLID`.
fn font_handle(family: &Family) -> String {
    family.id().replace('-', "_").to_uppercase()
}

/// The fonts an icon list needs, in a stable order.
pub fn used_families(icons: &[Resolved]) -> Vec<&'static Family> {
    let unique: BTreeMap<&str, &'static Family> = icons
        .iter()
        .map(|icon| (icon.family.id(), icon.family))
        .collect();

    unique.into_values().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// One icon from the first enabled family, ready to render.
    fn one_icon() -> Option<(Resolved, Bundled)> {
        let family = crate::families().first().copied()?;
        let (name, codepoint) = family.icons().into_iter().next()?;

        Some((
            Resolved {
                function: crate::sanitize_fn_name(&name),
                icon: name,
                family,
                codepoint,
            },
            Bundled {
                family,
                path: "../fonts/test.ttf".to_string(),
            },
        ))
    }

    #[test]
    fn per_icon_mode_emits_a_function_for_each_icon() {
        let Some((icon, bundled)) = one_icon() else {
            return;
        };

        let source = module(
            std::slice::from_ref(&icon),
            std::slice::from_ref(&bundled),
            "hash",
            Functions::PerIcon,
        );

        assert!(
            source.contains(&format!("pub fn {}<", icon.function)),
            "expected a function for {}",
            icon.function
        );
    }

    #[test]
    fn index_mode_emits_the_index_but_no_per_icon_functions() {
        let Some((icon, bundled)) = one_icon() else {
            return;
        };

        let source = module(
            std::slice::from_ref(&icon),
            std::slice::from_ref(&bundled),
            "hash",
            Functions::Omit,
        );

        assert!(
            !source.contains(&format!("pub fn {}<", icon.function)),
            "index mode should not name individual icons"
        );

        // The parts a picker actually uses still have to be there.
        assert!(source.contains("pub const ALL_ICONS"));
        assert!(source.contains("pub const FAMILIES"));
        assert!(source.contains("pub fn render<"));
        assert!(source.contains(&format!("family: {:?}", icon.family.id())));
    }
}
