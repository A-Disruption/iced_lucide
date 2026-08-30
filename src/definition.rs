//! The icon definition file: its schema, and how entries resolve to glyphs.

use std::collections::BTreeMap;
use std::collections::BTreeSet;

use serde::Deserialize;

use crate::{Error, Family};

/// A parsed icon definition TOML file.
#[derive(Debug, Clone, Deserialize)]
pub struct Definition {
    /// Path of the module to generate, e.g. `icon` or `ui::icon`.
    pub module: String,
    /// Family used for icon names that are not explicitly qualified.
    #[serde(default)]
    pub family: Option<String>,
    #[serde(default)]
    pub icons: BTreeMap<String, Entry>,
}

/// A value under `[icons]`.
///
/// A string names a single icon; a table groups several icons under one family,
/// which saves repeating the family prefix on every line.
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Entry {
    Icon(String),
    Group(BTreeMap<String, String>),
}

/// An icon request that has been matched to a real glyph.
#[derive(Debug, Clone)]
pub struct Resolved {
    /// Name of the generated Rust function.
    pub function: String,
    /// The upstream icon name.
    pub icon: String,
    pub family: &'static Family,
    pub codepoint: u32,
}

impl Definition {
    /// Match every entry against the enabled families.
    ///
    /// Results are ordered by function name so that generated output — and the
    /// hash derived from it — does not depend on TOML ordering.
    pub fn resolve(&self) -> Result<Vec<Resolved>, Error> {
        let default = self.default_family()?;
        let mut resolved: Vec<Resolved> = Vec::new();

        for (key, entry) in &self.icons {
            match entry {
                Entry::Icon(icon) => resolved.push(resolve_one(key, icon, default)?),
                Entry::Group(group) => {
                    let family = crate::family(key).ok_or_else(|| Error::UnknownFamily {
                        id: key.clone(),
                        available: enabled_ids(),
                    })?;

                    for (function, icon) in group {
                        resolved.push(resolve_one(function, icon, Some(family))?);
                    }
                }
            }
        }

        // Collisions are checked on the generated name rather than the key,
        // because sanitising can bring two distinct keys together.
        let mut seen: BTreeSet<&str> = BTreeSet::new();
        for icon in &resolved {
            if !seen.insert(&icon.function) {
                return Err(Error::DuplicateFunction(icon.function.clone()));
            }
        }

        resolved.sort_by(|a, b| a.function.cmp(&b.function));

        Ok(resolved)
    }

    /// The family that unqualified icon names belong to.
    ///
    /// `None` means there is no sensible default and every name must say which
    /// family it comes from; that only becomes an error if an unqualified name
    /// actually turns up.
    fn default_family(&self) -> Result<Option<&'static Family>, Error> {
        if let Some(id) = &self.family {
            return crate::family(id)
                .map(Some)
                .ok_or_else(|| Error::UnknownFamily {
                    id: id.clone(),
                    available: enabled_ids(),
                });
        }

        let enabled = crate::families();

        Ok(match enabled.len() {
            // With a single family enabled there is nothing to be ambiguous about.
            1 => Some(enabled[0]),
            // Otherwise fall back to this crate's original family, if present.
            _ => enabled.into_iter().find(|family| family.id() == "lucide"),
        })
    }
}

/// Resolve one `function = "icon"` pair.
///
/// The icon may carry a `family:` prefix, which overrides the default.
fn resolve_one(
    function: &str,
    icon: &str,
    default: Option<&'static Family>,
) -> Result<Resolved, Error> {
    let (family, name) = match icon.split_once(':') {
        Some((id, name)) => {
            let family = crate::family(id).ok_or_else(|| Error::UnknownFamily {
                id: id.to_string(),
                available: enabled_ids(),
            })?;
            (family, name)
        }
        None => {
            let family = default.ok_or_else(|| Error::AmbiguousFamily {
                icon: icon.to_string(),
                available: enabled_ids(),
            })?;
            (family, icon)
        }
    };

    let codepoint = family.codepoint(name).ok_or_else(|| Error::UnknownIcon {
        family: family.label(),
        browse_url: family.browse_url(),
        name: name.to_string(),
        suggestions: family.suggest(name),
    })?;

    Ok(Resolved {
        // The key is the author's choice of name, but it still has to be a
        // legal identifier that does not collide with the module's own items.
        function: crate::sanitize_fn_name(function),
        icon: name.to_string(),
        family,
        codepoint,
    })
}

fn enabled_ids() -> Vec<&'static str> {
    crate::families().into_iter().map(Family::id).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(toml: &str) -> Definition {
        ::toml::from_str(toml).expect("valid definition")
    }

    #[test]
    fn reads_flat_icon_entries() {
        let definition = parse("module = \"icon\"\n[icons]\nedit = \"pencil\"\n");

        assert_eq!(definition.module, "icon");
        assert!(matches!(
            definition.icons.get("edit"),
            Some(Entry::Icon(name)) if name == "pencil"
        ));
    }

    #[test]
    fn reads_family_grouped_entries() {
        let definition = parse(
            "module = \"icon\"\n\
             [icons.lucide]\n\
             edit = \"pencil\"\n",
        );

        let Some(Entry::Group(group)) = definition.icons.get("lucide") else {
            panic!("expected a family group, got {:?}", definition.icons);
        };

        assert_eq!(group.get("edit").map(String::as_str), Some("pencil"));
    }

    #[test]
    fn reads_flat_and_grouped_entries_side_by_side() {
        let definition = parse(
            "module = \"icon\"\n\
             [icons]\n\
             edit = \"pencil\"\n\
             [icons.lucide]\n\
             save = \"save\"\n",
        );

        assert!(matches!(definition.icons.get("edit"), Some(Entry::Icon(_))));
        assert!(matches!(
            definition.icons.get("lucide"),
            Some(Entry::Group(_))
        ));
    }

    #[test]
    fn rejects_two_functions_with_the_same_name() {
        let families = crate::families();
        if families.len() < 2 {
            return;
        }

        let definition = Definition {
            module: "icon".to_string(),
            family: None,
            icons: BTreeMap::from([
                (
                    families[0].id().to_string(),
                    Entry::Group(BTreeMap::from([(
                        "duplicate".to_string(),
                        families[0].icons()[0].0.clone(),
                    )])),
                ),
                (
                    families[1].id().to_string(),
                    Entry::Group(BTreeMap::from([(
                        "duplicate".to_string(),
                        families[1].icons()[0].0.clone(),
                    )])),
                ),
            ]),
        };

        assert!(matches!(
            definition.resolve(),
            Err(Error::DuplicateFunction(name)) if name == "duplicate"
        ));
    }

    #[test]
    fn reports_an_unknown_icon_with_suggestions() {
        let Some(family) = crate::families().first().copied() else {
            return;
        };

        let definition = Definition {
            module: "icon".to_string(),
            family: Some(family.id().to_string()),
            icons: BTreeMap::from([(
                "missing".to_string(),
                Entry::Icon("definitely-not-an-icon".to_string()),
            )]),
        };

        assert!(matches!(
            definition.resolve(),
            Err(Error::UnknownIcon { .. })
        ));
    }

    #[test]
    fn a_family_prefix_overrides_the_default() {
        let families = crate::families();
        if families.len() < 2 {
            return;
        }

        let (first, second) = (families[0], families[1]);
        let icon = second.icons()[0].0.clone();

        let definition = Definition {
            module: "icon".to_string(),
            family: Some(first.id().to_string()),
            icons: BTreeMap::from([(
                "qualified".to_string(),
                Entry::Icon(format!("{}:{icon}", second.id())),
            )]),
        };

        let resolved = definition.resolve().expect("resolves");

        assert_eq!(resolved[0].family.id(), second.id());
    }
}
