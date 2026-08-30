//! Parsers turning each upstream's metadata into `(name, codepoint)` pairs.

use std::collections::BTreeMap;

use serde::Deserialize;

use crate::sources::Metadata;

pub type Icons = BTreeMap<String, u32>;

pub fn parse(metadata: &Metadata, raw: &[u8]) -> Result<Icons, String> {
    let text = String::from_utf8_lossy(raw);

    match metadata {
        Metadata::LucideHtml => Ok(lucide_html(&text)),
        Metadata::FlatJson {
            strip_prefix,
            strip_suffix,
        } => flat_json(&text, *strip_prefix, *strip_suffix),
        Metadata::Codepoints => Ok(codepoints(&text)),
        Metadata::Css { prefixes } => Ok(css(&text, prefixes)),
        Metadata::FontAwesome { style } => font_awesome(&text, style),
        Metadata::NerdFonts { prefix } => nerd_fonts(&text, *prefix),
    }
}

/// Trim a publisher's namespacing off a name and settle on hyphens.
///
/// Upstreams that namespace their keys do it consistently, so a name that does
/// not carry the prefix is not one of ours and is dropped.
fn rename(name: &str, strip_prefix: Option<&str>, strip_suffix: Option<&str>) -> Option<String> {
    let mut trimmed = match strip_prefix {
        Some(prefix) => name.strip_prefix(prefix)?,
        None => name,
    };

    if let Some(suffix) = strip_suffix {
        trimmed = trimmed.strip_suffix(suffix)?;
    }

    let renamed = trimmed.replace('_', "-");

    (!renamed.is_empty()).then_some(renamed)
}

/// Lucide's `unicode.html`, which pairs each name with an HTML entity:
///
/// ```html
/// <h4>pencil</h4><span class="unicode">&amp;#57347;</span>
/// ```
fn lucide_html(html: &str) -> Icons {
    let mut icons = Icons::new();
    let mut remaining = html;

    while let Some(start) = remaining.find("<h4>") {
        remaining = &remaining[start + 4..];

        let Some(end) = remaining.find("</h4>") else {
            break;
        };

        let name = remaining[..end].trim().to_string();
        remaining = &remaining[end + 5..];

        // The codepoint span follows the </h4> within the same <li>.
        let item_end = remaining.find("</li>").unwrap_or(remaining.len());
        let item = &remaining[..item_end];

        if let Some(position) = item.find("&amp;#") {
            let after = &item[position + 6..];

            if let Some(semicolon) = after.find(';')
                && let Ok(codepoint) = after[..semicolon].parse::<u32>()
            {
                icons.insert(name, codepoint);
            }
        }
    }

    icons
}

/// A flat `{"icon-name": 61697}` object, as Bootstrap Icons publishes.
fn flat_json(
    json: &str,
    strip_prefix: Option<&str>,
    strip_suffix: Option<&str>,
) -> Result<Icons, String> {
    let raw: BTreeMap<String, u32> =
        serde_json::from_str(json).map_err(|error| format!("flat JSON: {error}"))?;

    Ok(raw
        .into_iter()
        .filter_map(|(name, codepoint)| {
            Some((rename(&name, strip_prefix, strip_suffix)?, codepoint))
        })
        .collect())
}

/// Whitespace-separated `name hexcodepoint` lines.
fn codepoints(text: &str) -> Icons {
    text.lines()
        .filter_map(|line| {
            let (name, code) = line.split_once(char::is_whitespace)?;
            let codepoint = u32::from_str_radix(code.trim(), 16).ok()?;

            Some((rename(name, None, None)?, codepoint))
        })
        .collect()
}

/// CSS rules of the form `.prefix-name:before { content: "\ea60" }`.
///
/// Selectors may be grouped, in which case every class in the group takes the
/// block's codepoint — Devicon relies on this, Codicon does not.
fn css(stylesheet: &str, prefixes: &[(&str, &str)]) -> Icons {
    let mut icons = Icons::new();

    for rule in stylesheet.split('}') {
        let Some((selector, declarations)) = rule.split_once('{') else {
            continue;
        };

        let Some(codepoint) = css_content_codepoint(declarations) else {
            continue;
        };

        for (class_prefix, name_prefix) in prefixes {
            for name in css_class_names(selector, class_prefix) {
                icons.insert(format!("{name_prefix}{name}"), codepoint);
            }
        }
    }

    icons
}

/// Pull every `.prefix-<name>` class out of a (possibly grouped) selector.
fn css_class_names(selector: &str, prefix: &str) -> Vec<String> {
    let needle = format!(".{prefix}");
    let mut names = Vec::new();
    let mut remaining = selector;

    while let Some(position) = remaining.find(&needle) {
        remaining = &remaining[position + needle.len()..];

        let name: String = remaining
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_')
            .collect();

        if !name.is_empty() {
            names.push(name);
        }
    }

    names
}

/// Read the codepoint out of a `content:` declaration.
///
/// Both spellings appear in the wild: an escape (`content: "\ea60"`), and the
/// literal character itself (`content:""`), which is what minifiers emit
/// once they decide the escape is redundant.
fn css_content_codepoint(declarations: &str) -> Option<u32> {
    let position = declarations.find("content:")?;
    let after = &declarations[position + "content:".len()..];

    // The value is quoted with either flavour of quote.
    let open = after.find(['"', '\''])?;
    let quote = after.as_bytes()[open] as char;
    let rest = &after[open + 1..];
    let close = rest.find(quote)?;
    let value = &rest[..close];

    match value.strip_prefix('\\') {
        // CSS terminates an escape with an optional trailing space, which must
        // not be read as part of the value.
        Some(escaped) => {
            let hex: String = escaped
                .chars()
                .take_while(char::is_ascii_hexdigit)
                .collect();
            u32::from_str_radix(&hex, 16).ok()
        }
        None => value.chars().next().map(u32::from),
    }
}

/// Font Awesome's `metadata/icons.json`, filtered to a single style.
///
/// The file is several megabytes because it embeds every icon's SVG source;
/// only the style list and codepoint are of interest here.
fn font_awesome(json: &str, style: &str) -> Result<Icons, String> {
    #[derive(Deserialize)]
    struct Icon {
        #[serde(default)]
        styles: Vec<String>,
        unicode: String,
    }

    let all: BTreeMap<String, Icon> =
        serde_json::from_str(json).map_err(|error| format!("Font Awesome metadata: {error}"))?;

    Ok(all
        .into_iter()
        .filter(|(_, icon)| icon.styles.iter().any(|candidate| candidate == style))
        .filter_map(|(name, icon)| {
            u32::from_str_radix(&icon.unicode, 16)
                .ok()
                .map(|codepoint| (name, codepoint))
        })
        .collect())
}

/// Nerd Fonts' `glyphnames.json`: `{"cod-account": {"char": "", "code": "eb99"}}`.
///
/// With a `prefix`, only that upstream set is kept and the prefix is dropped,
/// carving a single family out of the aggregate font. Nerd Fonts also spells
/// names with underscores where the original projects use hyphens, so those are
/// put back — `oct-arrow_down` becomes `arrow-down`, which is what Primer calls
/// it and what someone reading their site would type.
fn nerd_fonts(json: &str, prefix: Option<&str>) -> Result<Icons, String> {
    #[derive(Deserialize)]
    struct Glyph {
        code: String,
    }

    let all: BTreeMap<String, serde_json::Value> =
        serde_json::from_str(json).map_err(|error| format!("Nerd Fonts glyphnames: {error}"))?;

    Ok(all
        .into_iter()
        // The file carries a METADATA entry with a different shape.
        .filter(|(name, _)| name != "METADATA")
        .filter_map(|(name, value)| {
            let name = match prefix {
                Some(prefix) => name.strip_prefix(prefix)?.replace('_', "-"),
                None => name,
            };

            let glyph: Glyph = serde_json::from_value(value).ok()?;
            let codepoint = u32::from_str_radix(&glyph.code, 16).ok()?;

            Some((name, codepoint))
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_grouped_css_selectors() {
        let stylesheet = "
            .devicon-nixos-plain-wordmark:before,
            .devicon-nixos-plain:before {
              content: \"\\e992\";
            }
        ";

        let icons = css(stylesheet, &[("devicon-", "")]);

        assert_eq!(icons.get("nixos-plain"), Some(&0xe992));
        assert_eq!(icons.get("nixos-plain-wordmark"), Some(&0xe992));
    }

    #[test]
    fn reads_single_line_css_rules() {
        let icons = css(
            ".codicon-add:before { content: \"\\ea60\" }",
            &[("codicon-", "")],
        );

        assert_eq!(icons.get("add"), Some(&0xea60));
    }

    /// Boxicons puts three styles in one font, told apart only by class prefix.
    #[test]
    fn keeps_styles_apart_when_they_share_a_font() {
        let stylesheet = "
            .bx-alarm:before { content: \"\\e900\" }
            .bxs-alarm:before { content: \"\\e901\" }
            .bxl-github:before { content: \"\\e902\" }
        ";

        let icons = css(
            stylesheet,
            &[("bx-", ""), ("bxs-", "solid-"), ("bxl-", "logo-")],
        );

        assert_eq!(icons.get("alarm"), Some(&0xe900));
        assert_eq!(icons.get("solid-alarm"), Some(&0xe901));
        assert_eq!(icons.get("logo-github"), Some(&0xe902));

        // `.bxs-alarm` must not also register as the bare `alarm`, which would
        // silently overwrite the regular style.
        assert_eq!(icons.len(), 3, "{icons:?}");
    }

    #[test]
    fn trims_publisher_namespacing_off_flat_json_keys() {
        let json = r#"{"ic_fluent_access_time_24_regular": 61697}"#;

        let icons = flat_json(json, Some("ic_fluent_"), Some("_regular")).expect("parses");

        assert_eq!(icons.get("access-time-24"), Some(&61697));
    }

    #[test]
    fn reads_codepoint_listings() {
        let icons = codepoints("10k e951\nairplanemode_active e195\n");

        assert_eq!(icons.get("10k"), Some(&0xe951));
        assert_eq!(icons.get("airplanemode-active"), Some(&0xe195));
    }

    #[test]
    fn reads_a_literal_character_as_content() {
        // Minified stylesheets drop the escape and embed the glyph directly.
        assert_eq!(css_content_codepoint("content:\"\u{e9a6}\""), Some(0xe9a6));
    }

    #[test]
    fn stops_at_the_space_terminating_a_css_escape() {
        // Font Awesome writes low codepoints as "\30 " — the trailing space
        // terminates the escape and is not part of the value.
        assert_eq!(css_content_codepoint("content: \"\\30 \";"), Some(0x30));
    }

    #[test]
    fn reads_lucide_html() {
        let html = "<li><h4>pencil</h4><span class=\"unicode\">&amp;#57347;</span></li>";

        assert_eq!(lucide_html(html).get("pencil"), Some(&57347));
    }
}
