//! Browse every icon family this crate carries, and export the picks.
//!
//! The generated `icon` module is an index rather than a set of named
//! functions — see `build.rs` — so everything here works off `ALL_ICONS`,
//! filtering it by family and by name and handing the survivors to `render`.
//!
//! Whatever you collect can be copied straight out as the three pieces a
//! project needs: the icon definition, the build script, and the manifest
//! entry with the right features turned on.

mod icon;

use std::collections::BTreeSet;

use iced::alignment::Vertical;
use iced::widget::{Row, button, column, container, row, scrollable, text, text_input, tooltip};
use iced::{Background, Border, Color, Element, Font, Length, Task, Theme};
use widgets::color_picker_two::color_picker_two;

/// Drawing twenty thousand buttons a frame helps nobody. Past this, the count
/// nudges you to narrow the search instead.
const MAX_RESULTS: usize = 800;

/// The build script a project needs. It does not vary with the selection —
/// the icons live in the definition file it points at.
const BUILD_RS: &str = r#"pub fn main() {
    println!("cargo::rerun-if-changed=fonts/my-icons.toml");
    iced_lucide::build("fonts/my-icons.toml").expect("Build icon module");
}
"#;

fn main() -> iced::Result {
    let mut app = iced::application(Picker::default, Picker::update, Picker::view)
        .title("iced_lucide — icon picker")
        .theme(Picker::theme);

    // One subset font per family; every one has to be registered.
    for font in icon::FONTS {
        app = app.font(*font);
    }

    app.run()
}

/// Which of the three snippets the export panel is showing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Snippet {
    #[default]
    Icons,
    Build,
    Manifest,
}

impl Snippet {
    const ALL: [Self; 3] = [Self::Icons, Self::Build, Self::Manifest];

    fn label(self) -> &'static str {
        match self {
            Self::Icons => "my-icons.toml",
            Self::Build => "build.rs",
            Self::Manifest => "Cargo.toml",
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    Search(String),
    ToggleFamily(&'static str),
    ShowAllFamilies,
    ToggleIcon(icon::Icon),
    Remove(usize),
    ClearSelection,
    UseThemeColor,
    OpenColorPicker,
    CloseColorPicker,
    ColorChanged(Color),
    ShowSnippet(Snippet),
    CopySnippet,
    /// Whether the clipboard actually took it.
    Copied(bool),
}

#[derive(Default)]
struct Picker {
    search: String,
    /// Active family filters. Empty means no filtering at all, which reads
    /// better than holding all ten and having to keep the set in step.
    families: BTreeSet<&'static str>,
    selected: Vec<icon::Icon>,
    /// `None` follows the theme's text colour.
    color: Option<Color>,
    color_picker_open: bool,
    snippet: Snippet,
    /// Acknowledgement on the copy button, cleared by the next interaction.
    copied: bool,
}

impl Picker {
    fn theme(&self) -> Theme {
        Theme::TokyoNight
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        // Doing anything else retires the "Copied" acknowledgement.
        if !matches!(message, Message::CopySnippet | Message::Copied(_)) {
            self.copied = false;
        }

        match message {
            Message::Search(search) => self.search = search,
            Message::ToggleFamily(family) => {
                if !self.families.remove(family) {
                    self.families.insert(family);
                }
            }
            Message::ShowAllFamilies => self.families.clear(),
            Message::ToggleIcon(icon) => {
                if let Some(index) = self.selected.iter().position(|held| *held == icon) {
                    self.selected.remove(index);
                } else {
                    self.selected.push(icon);
                }
            }
            Message::Remove(index) => {
                if index < self.selected.len() {
                    self.selected.remove(index);
                }
            }
            Message::ClearSelection => self.selected.clear(),
            Message::UseThemeColor => self.color = None,
            Message::OpenColorPicker => self.color_picker_open = true,
            Message::CloseColorPicker => self.color_picker_open = false,
            Message::ColorChanged(color) => self.color = Some(color),
            Message::ShowSnippet(snippet) => self.snippet = snippet,
            Message::CopySnippet => {
                // Acknowledge only once the clipboard has actually taken it.
                return iced::clipboard::write(self.snippet_text())
                    .map(|result| Message::Copied(result.is_ok()));
            }
            Message::Copied(succeeded) => self.copied = succeeded,
        }

        Task::none()
    }

    /// Icons passing the current family filter and search term.
    fn matches(&self) -> Vec<icon::Icon> {
        let needle = self.search.trim().to_lowercase();

        icon::ALL_ICONS
            .iter()
            .filter(|icon| self.families.is_empty() || self.families.contains(icon.family))
            .filter(|icon| needle.is_empty() || icon.name.contains(&needle))
            .copied()
            .collect()
    }

    // -----------------------------------------------------------------------
    // Export
    // -----------------------------------------------------------------------

    /// The Rust name each selected icon will be generated under.
    ///
    /// Two families can offer the same name — `search` turns up in most of
    /// them — so anything already taken gets a numeric suffix. `iced_lucide`
    /// would otherwise refuse the definition as a duplicate.
    fn function_names(&self) -> Vec<String> {
        let mut taken: BTreeSet<String> = BTreeSet::new();

        self.selected
            .iter()
            .map(|held| {
                let base = iced_lucide::function_name(held.name);
                let mut candidate = base.clone();
                let mut suffix = 2;

                while taken.contains(&candidate) {
                    candidate = format!("{base}_{suffix}");
                    suffix += 1;
                }

                taken.insert(candidate.clone());
                candidate
            })
            .collect()
    }

    /// The families the selection draws on, in the order `FAMILIES` lists them.
    fn families_used(&self) -> Vec<&'static icon::Family> {
        icon::FAMILIES
            .iter()
            .filter(|family| self.selected.iter().any(|held| held.family == family.id))
            .collect()
    }

    fn snippet_text(&self) -> String {
        match self.snippet {
            Snippet::Icons => self.icons_toml(),
            Snippet::Build => BUILD_RS.to_string(),
            Snippet::Manifest => self.manifest_toml(),
        }
    }

    /// `fonts/my-icons.toml` for the current selection.
    fn icons_toml(&self) -> String {
        if self.selected.is_empty() {
            return "# Pick some icons and the definition will appear here.\n".to_string();
        }

        let names = self.function_names();
        let mut out = String::new();

        out.push_str(&format!(
            "# {} icon{} chosen in the iced_lucide picker.\n",
            self.selected.len(),
            if self.selected.len() == 1 { "" } else { "s" },
        ));

        for family in self.families_used() {
            out.push_str(&format!("# {} — {}\n", family.name, family.license));
        }

        out.push_str("\nmodule = \"icon\"\n");

        for family in self.families_used() {
            out.push_str(&format!("\n[icons.{}]\n", family.id));

            for (held, name) in self.selected.iter().zip(&names) {
                if held.family == family.id {
                    out.push_str(&format!("{name} = \"{}\"\n", held.name));
                }
            }
        }

        out
    }

    /// The `build-dependencies` entry, with just the features the picks need.
    fn manifest_toml(&self) -> String {
        let features: BTreeSet<&str> = self
            .families_used()
            .iter()
            .map(|family| family.feature)
            .collect();

        let mut out = String::from("[build-dependencies]\n");

        // `lucide` is on by default, so asking for it explicitly is noise.
        if features.is_empty() || features.iter().eq(["lucide"].iter()) {
            out.push_str("iced_lucide = \"0.2\"\n");
        } else {
            let list: Vec<String> = features
                .iter()
                .map(|feature| format!("\"{feature}\""))
                .collect();

            out.push_str(&format!(
                "iced_lucide = {{ version = \"0.2\", features = [{}] }}\n",
                list.join(", "),
            ));
        }

        if !self.selected.is_empty() {
            out.push_str("\n# Shipping these fonts means shipping their licenses:\n");

            for family in self.families_used() {
                out.push_str(&format!(
                    "#   {} — {} — {}\n",
                    family.name, family.license, family.url
                ));
            }
        }

        out
    }

    // -----------------------------------------------------------------------
    // View
    // -----------------------------------------------------------------------

    fn view(&self) -> Element<'_, Message> {
        let matches = self.matches();

        let content = column![
            self.search_bar(matches.len()),
            self.family_filters(),
            self.colour_bar(),
            row![
                container(scrollable(self.grid(&matches)))
                    .width(Length::Fill)
                    .height(Length::Fill),
                self.side_panel(),
            ]
            .spacing(16),
        ]
        .spacing(12)
        .padding(16);

        // The picker draws itself as an overlay when open, so it sits beside
        // the content rather than inside the layout.
        let colour_picker: Element<'_, Message> =
            color_picker_two(self.color_picker_open, self.color.unwrap_or(Color::WHITE))
                .on_change(Message::ColorChanged)
                .on_close(|| Message::CloseColorPicker)
                .into();

        column![content, colour_picker].into()
    }

    fn search_bar(&self, total: usize) -> Element<'_, Message> {
        let summary = if total > MAX_RESULTS {
            format!("showing {MAX_RESULTS} of {total} — keep typing to narrow it down")
        } else {
            format!("{total} icons")
        };

        row![
            text_input("Search icons by name…", &self.search)
                .on_input(Message::Search)
                .padding(10)
                .width(Length::Fill),
            text(summary).size(13),
        ]
        .spacing(12)
        .align_y(Vertical::Center)
        .into()
    }

    fn family_filters(&self) -> Element<'_, Message> {
        let mut chips: Vec<Element<'_, Message>> = vec![chip(
            "All",
            self.families.is_empty(),
            Message::ShowAllFamilies,
        )];

        chips.extend(icon::FAMILIES.iter().map(|family| {
            chip(
                family.name,
                self.families.contains(family.id),
                Message::ToggleFamily(family.id),
            )
        }));

        Row::with_children(chips).spacing(6).wrap().into()
    }

    fn colour_bar(&self) -> Element<'_, Message> {
        let mut swatches: Vec<Element<'_, Message>> =
            vec![chip("Theme", self.color.is_none(), Message::UseThemeColor)];

        swatches.extend(presets().into_iter().map(|colour| {
            let active = self.color == Some(colour);

            button(text(" ").size(12))
                .width(Length::Fixed(30.0))
                .height(Length::Fixed(26.0))
                .on_press(Message::ColorChanged(colour))
                .style(move |_theme, _status| button::Style {
                    background: Some(Background::Color(colour)),
                    text_color: Color::TRANSPARENT,
                    border: Border {
                        color: if active {
                            Color::WHITE
                        } else {
                            Color::from_rgba(0.0, 0.0, 0.0, 0.35)
                        },
                        width: if active { 2.0 } else { 1.0 },
                        radius: 6.0.into(),
                    },
                    ..Default::default()
                })
                .into()
        }));

        swatches.push(
            button(text("Custom…").size(13))
                .on_press(Message::OpenColorPicker)
                .style(button::secondary)
                .into(),
        );

        row![
            text("Colour").size(13),
            Row::with_children(swatches).spacing(6).wrap(),
        ]
        .spacing(12)
        .align_y(Vertical::Center)
        .into()
    }

    fn grid<'a>(&'a self, matches: &[icon::Icon]) -> Element<'a, Message> {
        let cells: Vec<Element<'a, Message>> = matches
            .iter()
            .take(MAX_RESULTS)
            .map(|found| {
                let held = self.selected.contains(found);

                tooltip(
                    button(icon::render(*found).size(22).color_maybe(self.color))
                        .padding(8)
                        .on_press(Message::ToggleIcon(*found))
                        .style(cell(held)),
                    container(text(format!("{}  ·  {}", found.name, found.family)).size(12))
                        .style(container::bordered_box)
                        .padding(5),
                    tooltip::Position::Top,
                )
                .into()
            })
            .collect();

        if cells.is_empty() {
            return container(text("Nothing matches that search.").size(14))
                .padding(24)
                .into();
        }

        Row::with_children(cells).spacing(2).wrap().into()
    }

    /// The picked icons, and the snippets that carry them into a project.
    fn side_panel(&self) -> Element<'_, Message> {
        container(
            column![
                self.selection_list(),
                container(self.export_panel()).padding([12, 0]),
            ]
            .spacing(4),
        )
        .padding(12)
        .width(Length::Fixed(360.0))
        .height(Length::Fill)
        .style(container::bordered_box)
        .into()
    }

    fn selection_list(&self) -> Element<'_, Message> {
        let header = row![
            text(format!("Selected ({})", self.selected.len()))
                .size(14)
                .width(Length::Fill),
            button(text("Clear").size(12))
                .on_press(Message::ClearSelection)
                .style(button::text),
        ]
        .align_y(Vertical::Center);

        let body: Element<'_, Message> = if self.selected.is_empty() {
            text("Click an icon to keep track of it here.")
                .size(12)
                .into()
        } else {
            let names = self.function_names();

            let rows: Vec<Element<'_, Message>> = self
                .selected
                .iter()
                .zip(&names)
                .enumerate()
                .map(|(index, (held, name))| {
                    row![
                        icon::render(*held).size(18).color_maybe(self.color),
                        column![
                            text(name.clone()).size(13),
                            text(format!("{}  ·  {}", held.name, held.family)).size(11),
                        ]
                        .spacing(1)
                        .width(Length::Fill),
                        button(text("×").size(16))
                            .on_press(Message::Remove(index))
                            .style(button::text),
                    ]
                    .spacing(10)
                    .align_y(Vertical::Center)
                    .into()
                })
                .collect();

            scrollable(column(rows).spacing(6))
                .height(Length::Fill)
                .into()
        };

        column![header, body]
            .spacing(10)
            .height(Length::Fill)
            .into()
    }

    /// Copyable snippets for wiring the selection into a project.
    fn export_panel(&self) -> Element<'_, Message> {
        let tabs = Row::with_children(Snippet::ALL.map(|snippet| {
            chip(
                snippet.label(),
                self.snippet == snippet,
                Message::ShowSnippet(snippet),
            )
        }))
        .spacing(6)
        .wrap();

        let preview = container(
            scrollable(text(self.snippet_text()).size(11).font(Font::MONOSPACE))
                .height(Length::Fixed(180.0)),
        )
        .padding(8)
        .width(Length::Fill)
        .style(container::bordered_box);

        let copy = button(
            text(if self.copied {
                "Copied ✓"
            } else {
                "Copy to clipboard"
            })
            .size(13),
        )
        .on_press(Message::CopySnippet)
        .style(if self.copied {
            button::success
        } else {
            button::primary
        });

        column![
            text("Use these in your project").size(14),
            tabs,
            preview,
            row![copy].spacing(8),
        ]
        .spacing(8)
        .into()
    }
}

/// A filter button that reads as pressed when it is on.
fn chip(label: &str, active: bool, message: Message) -> Element<'_, Message> {
    button(text(label.to_string()).size(13))
        .padding([4, 10])
        .on_press(message)
        .style(toggled(active))
        .into()
}

/// Style for a chip that can be on or off.
///
/// The branches are returned as a function pointer because `button::primary`
/// and `button::secondary` are distinct types; an `if` over them directly does
/// not typecheck.
fn toggled(active: bool) -> fn(&Theme, button::Status) -> button::Style {
    if active {
        button::primary
    } else {
        button::secondary
    }
}

/// Style for a grid cell, which is plain until it has been picked.
fn cell(selected: bool) -> fn(&Theme, button::Status) -> button::Style {
    if selected {
        button::primary
    } else {
        button::text
    }
}

/// A handful of colours to try without opening the picker.
fn presets() -> Vec<Color> {
    vec![
        Color::from_rgb8(0xF7, 0x76, 0x8E),
        Color::from_rgb8(0xFF, 0x9E, 0x64),
        Color::from_rgb8(0xE0, 0xAF, 0x68),
        Color::from_rgb8(0x9E, 0xCE, 0x6A),
        Color::from_rgb8(0x2A, 0xC3, 0xDE),
        Color::from_rgb8(0x7A, 0xA2, 0xF7),
        Color::from_rgb8(0xBB, 0x9A, 0xF7),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A picker holding the first icon of each of `count` families.
    fn with_one_icon_from_each(count: usize) -> Picker {
        let selected = icon::FAMILIES
            .iter()
            .take(count)
            .filter_map(|family| {
                icon::ALL_ICONS
                    .iter()
                    .find(|held| held.family == family.id)
                    .copied()
            })
            .collect();

        Picker {
            selected,
            ..Picker::default()
        }
    }

    /// Every icon in the index, so nothing can generate an unusable name.
    #[test]
    fn exported_names_are_unique_even_when_families_collide() {
        // `search` and friends appear in most families; the exporter has to
        // keep the TOML keys apart or iced_lucide rejects the definition.
        let selected: Vec<icon::Icon> = icon::ALL_ICONS
            .iter()
            .filter(|held| held.name == "search" || held.name == "home")
            .copied()
            .collect();

        assert!(
            selected.len() > 1,
            "expected the same name in several families"
        );

        let picker = Picker {
            selected,
            ..Picker::default()
        };
        let names = picker.function_names();

        let mut unique = names.clone();
        unique.sort();
        unique.dedup();

        assert_eq!(unique.len(), names.len(), "duplicate names in {names:?}");
    }

    #[test]
    fn exported_definition_is_valid_toml() {
        let picker = with_one_icon_from_each(icon::FAMILIES.len());
        let rendered = picker.icons_toml();

        // Parsing also proves no key was emitted twice: TOML rejects that.
        let parsed: toml::Table =
            toml::from_str(&rendered).unwrap_or_else(|error| panic!("{error}\n---\n{rendered}"));

        assert_eq!(
            parsed.get("module").and_then(toml::Value::as_str),
            Some("icon")
        );

        let icons = parsed
            .get("icons")
            .and_then(toml::Value::as_table)
            .expect("an [icons] table");

        assert_eq!(
            icons.len(),
            icon::FAMILIES.len(),
            "expected one group per family"
        );
    }

    #[test]
    fn exported_manifest_lists_only_the_features_the_picks_need() {
        let codicon = icon::ALL_ICONS
            .iter()
            .find(|held| held.family == "codicon")
            .copied()
            .expect("the codicon family is built in");

        let picker = Picker {
            selected: vec![codicon],
            ..Picker::default()
        };
        let rendered = picker.manifest_toml();

        assert!(rendered.contains("\"codicon\""), "{rendered}");
        assert!(!rendered.contains("\"nerdfonts\""), "{rendered}");
    }

    /// Lucide is a default feature, so asking for it explicitly is noise.
    #[test]
    fn exported_manifest_stays_bare_for_lucide_alone() {
        let lucide = icon::ALL_ICONS
            .iter()
            .find(|held| held.family == "lucide")
            .copied()
            .expect("the lucide family is built in");

        let picker = Picker {
            selected: vec![lucide],
            ..Picker::default()
        };

        assert!(
            picker.manifest_toml().contains("iced_lucide = \"0.2\""),
            "{}",
            picker.manifest_toml()
        );
    }

    #[test]
    fn an_empty_selection_still_renders_every_snippet() {
        let picker = Picker::default();

        for snippet in Snippet::ALL {
            let rendered = Picker {
                snippet,
                ..Picker::default()
            }
            .snippet_text();

            assert!(!rendered.is_empty(), "{snippet:?} rendered nothing");
        }

        assert!(picker.icons_toml().starts_with('#'));
    }
}
