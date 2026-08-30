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
use std::fmt;
use std::sync::LazyLock;

use iced::alignment::Vertical;
use iced::widget::{
    Row, button, column, container, operation, pick_list, row, scrollable, text, text_editor,
    text_input, tooltip,
};
use iced::{Background, Border, Color, Element, Font, Length, Padding, Task, Theme};
use widgets::anchor::Align;
use widgets::popover::popover;

/// Find one of the interface's own icons in the index.
///
/// Looked up once each: scanning eighty thousand icons per frame would not do.
/// `None` if the `fluent` feature is off, which every caller falls back for.
fn fluent(name: &str) -> Option<icon::Icon> {
    icon::ALL_ICONS
        .iter()
        .find(|held| held.family == "fluent" && held.name == name)
        .copied()
}

/// Shown beside a selected family.
static CHECKMARK: LazyLock<Option<icon::Icon>> = LazyLock::new(|| fluent("checkmark-12"));

/// On the button that copies the current snippet.
static COPY_ICON: LazyLock<Option<icon::Icon>> = LazyLock::new(|| fluent("copy-20"));

/// Replaces it once the clipboard has taken it.
static COPIED_ICON: LazyLock<Option<icon::Icon>> = LazyLock::new(|| fluent("checkmark-20"));

/// The most icons drawn at once.
///
/// This is a hard ceiling, not a starting point: the window slides through the
/// results as you scroll rather than growing, so a search matching twenty
/// thousand icons costs the same as one matching sixteen hundred.
const WINDOW: usize = 1600;

/// How far the window slides each time you reach one of its ends.
const STEP: usize = 400;

/// How close to an end counts as asking for the next or previous step.
const LOAD_MORE_AT: f32 = 0.85;
const LOAD_BACK_AT: f32 = 0.15;

/// Identifies the icon grid so the window can keep the view steady as it slides.
const GRID: &str = "icon-grid";

/// Keeps the rightmost column of icons clear of the scrollbar.
const SCROLLBAR_CLEARANCE: f32 = 18.0;

/// Fixed so the toolbar does not reflow as the selection changes.
const FAMILY_FIELD_WIDTH: f32 = 210.0;

/// Shared by both text fields so they come out the same height.
const FIELD_PADDING: u16 = 10;
const FIELD_TEXT_SIZE: f32 = 14.0;

/// Fixed for the same reason: the list must not resize as it is filtered.
const FAMILY_LIST_WIDTH: f32 = 262.0;

/// Wide enough for two rows of chips, and fixed like the others.
const COLOUR_LIST_WIDTH: f32 = 168.0;

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

/// A rendering size offered in the size dropdown.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct IconSize(u16);

impl IconSize {
    const ALL: [Self; 7] = [
        Self(12),
        Self(16),
        Self(20),
        Self(24),
        Self(32),
        Self(48),
        Self(64),
    ];

    /// How the dropdown labels an entry. `pick_list` asks for this by
    /// reference rather than relying on `Display`.
    fn label(size: &Self) -> String {
        size.to_string()
    }
}

impl fmt::Display for IconSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} px", self.0)
    }
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
    OpenFamilyList(bool),
    /// Text typed into the family field, narrowing the list of families.
    FamilyFilter(String),
    Scrolled(scrollable::Viewport),
    SizeChanged(IconSize),
    ToggleIcon(icon::Icon),
    Remove(usize),
    ClearSelection,
    UseThemeColor,
    OpenColorList(bool),
    /// Deliberately does nothing.
    ///
    /// The colour chip needs an `on_press` to render as a live button, but
    /// `popover` toggles itself on any press over its trigger *in addition* to
    /// whatever the trigger does. Toggling here as well would open and shut the
    /// list in the same click.
    ColorTriggerPressed,
    ColorChanged(Color),
    ShowSnippet(Snippet),
    SnippetAction(text_editor::Action),
    CopySnippet,
    /// Whether the clipboard actually took it.
    Copied(bool),
}

struct Picker {
    search: String,
    /// Active family filters. Empty means no filtering at all, which reads
    /// better than holding all of them and keeping the set in step.
    families: BTreeSet<&'static str>,
    family_list_open: bool,
    /// Narrows the family list inside the dropdown.
    family_filter: String,
    size: IconSize,
    /// Every icon passing the current filters.
    ///
    /// Cached rather than recomputed in `view`: scanning eighty thousand icons
    /// on every frame is wasted work when the answer only changes when the
    /// search box or the family filter does.
    results: Vec<icon::Icon>,
    /// Index of the first result the grid is drawing.
    ///
    /// The grid always draws at most [`WINDOW`] from here, so the widget count
    /// stays flat however many icons match.
    start: usize,
    selected: Vec<icon::Icon>,
    /// `None` follows the theme's text colour.
    color: Option<Color>,
    color_list_open: bool,
    snippet: Snippet,
    /// Held as editor content so the text can be selected and copied a line at
    /// a time. Edits are filtered out in `update`, leaving it read-only.
    snippet_content: text_editor::Content,
    /// Acknowledgement on the copy button, cleared by the next interaction.
    copied: bool,
}

impl Default for Picker {
    fn default() -> Self {
        let mut picker = Self {
            search: String::new(),
            families: BTreeSet::new(),
            family_list_open: false,
            family_filter: String::new(),
            size: IconSize(24),
            results: Vec::new(),
            start: 0,
            selected: Vec::new(),
            color: None,
            color_list_open: false,
            snippet: Snippet::default(),
            snippet_content: text_editor::Content::new(),
            copied: false,
        };

        picker.refresh_results();
        picker.refresh_snippet();
        picker
    }
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
            Message::Search(search) => {
                self.search = search;
                self.refresh_results();
            }
            Message::ToggleFamily(family) => {
                if !self.families.remove(family) {
                    self.families.insert(family);
                }
                self.refresh_results();
            }
            Message::ShowAllFamilies => {
                self.families.clear();
                self.refresh_results();
            }
            Message::OpenFamilyList(open) => self.family_list_open = open,
            Message::FamilyFilter(filter) => {
                self.family_filter = filter;
                // Typing in the field should reveal what it is narrowing.
                self.family_list_open = true;
            }
            Message::Scrolled(viewport) => {
                let slide =
                    slide_window(self.start, self.results.len(), viewport.relative_offset().y);

                self.start = slide.start;

                return match slide.snap {
                    // Sliding the window moves the icons under the cursor, so
                    // the view is nudged back by as much as it shifted.
                    Some(y) => operation::snap_to(GRID, scrollable::RelativeOffset { x: 0.0, y }),
                    None => Task::none(),
                };
            }
            Message::SizeChanged(size) => self.size = size,
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
            // Picking a colour is a single choice, so the list closes behind it.
            Message::UseThemeColor => {
                self.color = None;
                self.color_list_open = false;
            }
            Message::ColorChanged(color) => {
                self.color = Some(color);
                self.color_list_open = false;
            }
            Message::OpenColorList(open) => self.color_list_open = open,
            Message::ColorTriggerPressed => {}
            Message::ShowSnippet(snippet) => self.snippet = snippet,
            Message::SnippetAction(action) => {
                // Selecting, scrolling, and clicking are allowed through so the
                // text can be highlighted and copied; edits are dropped, which
                // is what makes it read-only.
                if !action.is_edit() {
                    self.snippet_content.perform(action);
                }

                return Task::none();
            }
            Message::CopySnippet => {
                // Acknowledge only once the clipboard has actually taken it.
                return iced::clipboard::write(self.snippet_text())
                    .map(|result| Message::Copied(result.is_ok()));
            }
            Message::Copied(succeeded) => self.copied = succeeded,
        }

        self.refresh_snippet();

        Task::none()
    }

    /// Recompute the icons passing the current filters.
    ///
    /// Also rewinds the window to the start: after changing what is being
    /// looked for, landing in the middle of the new results is disorienting.
    fn refresh_results(&mut self) {
        let needle = self.search.trim().to_lowercase();

        self.results = icon::ALL_ICONS
            .iter()
            .filter(|icon| self.families.is_empty() || self.families.contains(icon.family))
            .filter(|icon| needle.is_empty() || icon.name.contains(&needle))
            .copied()
            .collect();

        self.start = 0;
    }

    /// The families whose names match what has been typed in the family field.
    fn listed_families(&self) -> Vec<&'static icon::Family> {
        let needle = self.family_filter.trim().to_lowercase();

        icon::FAMILIES
            .iter()
            .filter(|family| needle.is_empty() || family.name.to_lowercase().contains(&needle))
            .collect()
    }

    // -----------------------------------------------------------------------
    // Export
    // -----------------------------------------------------------------------

    /// Rebuild the editor's buffer when the snippet it shows has changed.
    ///
    /// Replacing it unconditionally would throw away the caret and any
    /// selection on every keystroke in the search box.
    fn refresh_snippet(&mut self) {
        let wanted = self.snippet_text();

        if self.snippet_content.text().trim_end() != wanted.trim_end() {
            self.snippet_content = text_editor::Content::with_text(&wanted);
        }
    }

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
        column![
            self.toolbar(),
            row![
                // The scrollable itself has to fill, or it shrinks to the
                // wrapped content and leaves the scrollbar stranded well left
                // of the panel beside it.
                scrollable(self.grid())
                    // Reaching either end of the window slides it along.
                    .on_scroll(Message::Scrolled)
                    .id(GRID)
                    .width(Length::Fill)
                    .height(Length::Fill),
                self.side_panel(),
            ]
            .spacing(10),
        ]
        .spacing(12)
        .padding(16)
        .into()
    }

    fn toolbar(&self) -> Element<'_, Message> {
        // No filter means every family is in play, so the count reads full
        // rather than switching to a differently-shaped label.
        let active = if self.families.is_empty() {
            icon::FAMILIES.len()
        } else {
            self.families.len()
        };

        row![
            text_input("Search icons by name…", &self.search)
                .on_input(Message::Search)
                .padding(FIELD_PADDING)
                .size(FIELD_TEXT_SIZE)
                .style(field_style)
                .width(Length::Fill),
            self.family_dropdown(),
            // Fixed width, so the row does not shift as the count changes.
            container(text(format!("{active}/{}", icon::FAMILIES.len())).size(12))
                .width(Length::Fixed(46.0)),
            pick_list(Some(self.size), IconSize::ALL.to_vec(), IconSize::label)
                .on_select(Message::SizeChanged)
                .padding(8)
                .text_size(13),
            self.colour_dropdown(),
        ]
        .spacing(10)
        .align_y(Vertical::Center)
        .into()
    }

    /// One chip showing the current icon colour; pressing it offers the rest.
    fn colour_dropdown(&self) -> Element<'_, Message> {
        // The popover does the toggling; see `ColorTriggerPressed`.
        let trigger = swatch(self.color, false, Message::ColorTriggerPressed);

        let mut choices: Vec<Element<'_, Message>> =
            vec![swatch(None, self.color.is_none(), Message::UseThemeColor)];

        choices.extend(presets().into_iter().map(|colour| {
            swatch(
                Some(colour),
                self.color == Some(colour),
                Message::ColorChanged(colour),
            )
        }));

        let list = column![
            text("Icon colour").size(13),
            Row::with_children(choices).spacing(6).wrap(),
        ]
        .spacing(10);

        popover(trigger, list)
            .open(self.color_list_open)
            .on_toggle(Message::OpenColorList)
            .dismiss_on_outside_press(true)
            .align(Align::End)
            .min_width(COLOUR_LIST_WIDTH)
            .max_width(COLOUR_LIST_WIDTH)
            .padding(12.0)
            .into()
    }

    /// A dropdown that checks off as many families as you like.
    ///
    /// One row per family beats a wall of buttons once there are twenty-odd of
    /// them, and it keeps the toolbar to a single line.
    fn family_dropdown(&self) -> Element<'_, Message> {
        // A text field rather than a button: typing narrows the list below it,
        // which beats scrolling twenty-six families to find one. The row is a
        // fixed width so the toolbar does not reflow as the selection changes.
        let trigger = text_input("Filter families…", &self.family_filter)
            .on_input(Message::FamilyFilter)
            .width(Length::Fixed(FAMILY_FIELD_WIDTH))
            .padding(FIELD_PADDING)
            .size(FIELD_TEXT_SIZE)
            .style(field_style);

        let listed = self.listed_families();

        let rows: Vec<Element<'_, Message>> = listed
            .into_iter()
            .map(|family| {
                let on = self.families.contains(family.id);

                // A fixed-width slot keeps the labels aligned whether or not
                // the row is ticked.
                let tick: Element<'_, Message> = if !on {
                    text("").into()
                } else if let Some(mark) = *CHECKMARK {
                    icon::render(mark).size(13.0).into()
                } else {
                    text("✓").size(13).into()
                };

                button(
                    row![
                        container(tick).width(Length::Fixed(16.0)),
                        text(family.name).size(13),
                    ]
                    .spacing(8)
                    .align_y(Vertical::Center),
                )
                .width(Length::Fill)
                .padding([5, 8])
                .on_press(Message::ToggleFamily(family.id))
                .style(selectable(on))
                .into()
            })
            .collect();

        let body: Element<'_, Message> = if rows.is_empty() {
            container(text("No family matches that.").size(12))
                .padding(8)
                .into()
        } else {
            scrollable(column(rows).spacing(6).padding(Padding {
                right: SCROLLBAR_CLEARANCE,
                ..Padding::ZERO
            }))
            // Fixed, so filtering the list does not resize the popover.
            .height(Length::Fixed(320.0))
            .into()
        };

        let heading = match self.families.len() {
            0 => "All families".to_string(),
            count => format!("{count} selected"),
        };

        let list = column![
            row![
                text(heading).size(13).width(Length::Fill),
                button(text("All").size(12))
                    .on_press(Message::ShowAllFamilies)
                    .style(button::text),
            ]
            .align_y(Vertical::Center),
            container(body).height(Length::Fixed(320.0)),
        ]
        .spacing(10);

        popover(trigger, list)
            .open(self.family_list_open)
            .on_toggle(Message::OpenFamilyList)
            .dismiss_on_outside_press(true)
            // Flush with the trigger's leading edge rather than centred on it.
            .align(Align::Start)
            // One fixed width: min and max together stop the panel resizing as
            // families are ticked or the list is filtered.
            .min_width(FAMILY_LIST_WIDTH)
            .max_width(FAMILY_LIST_WIDTH)
            .padding(12.0)
            .into()
    }

    fn grid(&self) -> Element<'_, Message> {
        let cells: Vec<Element<'_, Message>> = self
            .results
            .iter()
            .skip(self.start)
            .take(WINDOW)
            .map(|found| {
                let held = self.selected.contains(found);

                tooltip(
                    button(
                        icon::render(*found)
                            .size(f32::from(self.size.0))
                            .color_maybe(self.color),
                    )
                    .padding(8)
                    .on_press(Message::ToggleIcon(*found))
                    .style(picked_cell(held)),
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

        Row::with_children(cells)
            .spacing(2)
            // Keeps the rightmost column out from under the scrollbar.
            .padding(Padding {
                right: SCROLLBAR_CLEARANCE,
                ..Padding::ZERO
            })
            .wrap()
            .into()
    }

    /// The picked icons, and the snippets that carry them into a project.
    fn side_panel(&self) -> Element<'_, Message> {
        let total = self.results.len();
        let shown = WINDOW.min(total - self.start);
        let summary = if shown < total {
            format!(
                "Showing {}–{} of {total} matching icons",
                self.start + 1,
                self.start + shown
            )
        } else {
            format!("{total} matching icons")
        };

        container(
            column![
                self.selection_list(),
                container(self.export_panel()).padding([12, 0]),
                text(summary).size(11),
            ]
            .spacing(4),
        )
        .padding(12)
        .width(Length::Fixed(380.0))
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
                        // Capped so a 64 px browsing size does not stretch the list.
                        icon::render(*held)
                            .size(f32::from(self.size.0.min(24)))
                            .color_maybe(self.color),
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

            scrollable(column(rows).spacing(6).padding(Padding {
                right: SCROLLBAR_CLEARANCE,
                ..Padding::ZERO
            }))
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

        // A text editor rather than a label: it can be selected and copied a
        // line at a time, which is what you want when the target file already
        // has entries in it. Edit actions are dropped in `update`.
        let preview = text_editor(&self.snippet_content)
            .on_action(Message::SnippetAction)
            .font(Font::MONOSPACE)
            .size(11)
            .height(Length::Fixed(200.0))
            .padding(8)
            .style(code_editor_style);

        let mark = if self.copied {
            *COPIED_ICON
        } else {
            *COPY_ICON
        };
        let face: Element<'_, Message> = match mark {
            Some(glyph) => icon::render(glyph).size(16.0).into(),
            None => text(if self.copied { "✓" } else { "Copy" }).size(13).into(),
        };

        let copy = tooltip(
            button(face)
                .padding(6)
                .on_press(Message::CopySnippet)
                .style(if self.copied {
                    button::success
                } else {
                    button::primary
                }),
            container(text("Copy all").size(12))
                .style(container::bordered_box)
                .padding(5),
            tooltip::Position::Left,
        );

        column![
            text("Use these in your project").size(14),
            row![container(tabs).width(Length::Fill), copy]
                .spacing(8)
                .align_y(Vertical::Center),
            preview,
        ]
        .spacing(8)
        .into()
    }
}

/// A small toggle button that reads as pressed when it is on.
fn chip(label: &str, active: bool, message: Message) -> Element<'_, Message> {
    button(text(label.to_string()).size(13))
        .padding([4, 10])
        .on_press(message)
        .style(toggled(active))
        .into()
}

/// Style for a control that can be on or off.
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

/// Where the window sits after a scroll, and where the view should land.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Slide {
    /// Index of the first icon to draw.
    start: usize,
    /// Scroll position to snap to, when the window moved under the view.
    snap: Option<f32>,
}

/// Advance or retreat the window after a scroll to `offset` (0 top, 1 bottom).
///
/// Split out from `update` because a `scrollable::Viewport` cannot be built by
/// hand, and the arithmetic is the part worth testing.
///
/// Because the window is a fixed size, moving it by `n` icons shifts everything
/// on screen by `n / WINDOW` of the scrollable's height. Snapping back by that
/// much leaves the icons under the cursor where they were, so a slide reads as
/// ordinary scrolling rather than a jump.
fn slide_window(start: usize, total: usize, offset: f32) -> Slide {
    let unchanged = Slide { start, snap: None };

    if total <= WINDOW {
        return Slide {
            start: 0,
            snap: None,
        };
    }

    let shift = |from: usize, to: usize| {
        let moved = to.abs_diff(from) as f32 / WINDOW as f32;
        let snapped = if to > from {
            offset - moved
        } else {
            offset + moved
        };

        Slide {
            start: to,
            snap: Some(snapped.clamp(0.0, 1.0)),
        }
    };

    let last = total - WINDOW;

    if offset >= LOAD_MORE_AT && start < last {
        return shift(start, (start + STEP).min(last));
    }

    if offset <= LOAD_BACK_AT && start > 0 {
        return shift(start, start.saturating_sub(STEP));
    }

    unchanged
}

/// Style for something that is plain until it has been picked.
fn selectable(selected: bool) -> fn(&Theme, button::Status) -> button::Style {
    if selected {
        button::primary
    } else {
        button::text
    }
}

/// Style for a grid cell: an outline when picked, never a fill.
///
/// A filled cell tints whatever is drawn on it, which fights with the icon
/// colour as soon as one is chosen. The outline uses the same colour as
/// selected text elsewhere, so "picked" reads the same throughout.
fn picked_cell(selected: bool) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |theme, status| {
        let base = button::text(theme, status);

        if !selected {
            return base;
        }

        button::Style {
            border: Border {
                color: theme.palette().background.strong.color,
                width: 2.0,
                radius: 6.0.into(),
            },
            ..base
        }
    }
}

/// Styling for the read-only snippet preview.
///
/// Everything comes from the theme; only the selection colour is overridden.
/// The default is `primary.weak`, which most themes make bright enough that
/// the text sitting on it disappears the moment you highlight anything.
fn code_editor_style(theme: &Theme, status: text_editor::Status) -> text_editor::Style {
    text_editor::Style {
        selection: theme.palette().background.strong.color,
        ..text_editor::default(theme, status)
    }
}

/// The same correction for the text fields, which default to it too.
fn field_style(theme: &Theme, status: text_input::Status) -> text_input::Style {
    text_input::Style {
        selection: theme.palette().background.strong.color,
        ..text_input::default(theme, status)
    }
}

/// A rounded colour chip.
///
/// `fill` of `None` stands for "follow the theme", and is drawn in the colour
/// the icons would take anyway.
fn swatch<'a>(fill: Option<Color>, active: bool, message: Message) -> Element<'a, Message> {
    button(text(" ").size(12))
        .width(Length::Fixed(30.0))
        .height(Length::Fixed(26.0))
        .on_press(message)
        .style(move |theme: &Theme, _status| {
            let palette = theme.palette();

            button::Style {
                background: Some(Background::Color(
                    fill.unwrap_or(palette.background.base.text),
                )),
                text_color: Color::TRANSPARENT,
                border: Border {
                    color: if active {
                        palette.background.base.text
                    } else {
                        palette.background.strong.color
                    },
                    width: if active { 2.0 } else { 1.0 },
                    radius: 6.0.into(),
                },
                ..Default::default()
            }
        })
        .into()
}

/// A handful of colours to try without opening the picker.
///
/// The only fixed colours in the app. Everything else — borders, backgrounds,
/// selection, button states — comes from the theme; these are the icon colour
/// being overridden, so taking them from the palette would defeat the point.
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

    /// The preview buffer has to follow the selection, or the copy button and
    /// the visible text disagree.
    #[test]
    fn the_preview_buffer_tracks_the_selection() {
        let mut picker = Picker::default();
        let lucide = icon::ALL_ICONS
            .iter()
            .find(|held| held.family == "lucide")
            .copied()
            .expect("the lucide family is built in");

        let _ = picker.update(Message::ToggleIcon(lucide));

        assert_eq!(
            picker.snippet_content.text().trim_end(),
            picker.snippet_text().trim_end(),
        );
        assert!(picker.snippet_content.text().contains(lucide.name));
    }

    /// Typing in the family field has to reveal what it is narrowing, and
    /// ticking a family must not shut the list on a multi-select.
    #[test]
    fn the_family_list_stays_up_while_it_is_being_used() {
        let mut picker = Picker::default();

        let _ = picker.update(Message::FamilyFilter("boot".to_string()));
        assert!(picker.family_list_open, "typing did not open the list");

        let _ = picker.update(Message::ToggleFamily(icon::FAMILIES[0].id));
        assert!(picker.family_list_open, "ticking a family closed the list");

        // Only the popover's own message closes it.
        let _ = picker.update(Message::OpenFamilyList(false));
        assert!(!picker.family_list_open);
    }

    /// Same hazard as the family list: `popover` toggles on top of the
    /// trigger's own press, so the chip must not toggle as well.
    #[test]
    fn the_colour_chip_does_not_toggle_its_own_list() {
        let mut picker = Picker::default();

        let _ = picker.update(Message::ColorTriggerPressed);
        assert!(!picker.color_list_open, "the chip toggled on its own");

        let _ = picker.update(Message::OpenColorList(true));
        let _ = picker.update(Message::ColorTriggerPressed);
        assert!(picker.color_list_open, "the chip closed it on its own");
    }

    /// Picking a colour is a single choice, so it applies and closes.
    #[test]
    fn choosing_a_colour_closes_the_list() {
        let mut picker = Picker::default();
        let chosen = presets()[0];

        let _ = picker.update(Message::OpenColorList(true));
        let _ = picker.update(Message::ColorChanged(chosen));

        assert_eq!(picker.color, Some(chosen));
        assert!(!picker.color_list_open);

        let _ = picker.update(Message::OpenColorList(true));
        let _ = picker.update(Message::UseThemeColor);

        assert_eq!(picker.color, None, "the theme option should clear it");
        assert!(!picker.color_list_open);
    }

    /// The family field narrows the list rather than the icons.
    #[test]
    fn the_family_field_filters_family_names() {
        let mut picker = Picker::default();
        let before = picker.results.len();

        let _ = picker.update(Message::FamilyFilter("bootstrap".to_string()));

        let listed = picker.listed_families();
        assert_eq!(listed.len(), 1, "expected one match, got {}", listed.len());
        assert_eq!(listed[0].id, "bootstrap");

        // Narrowing the family *list* must not narrow the icon results; only
        // actually ticking a family does that.
        assert_eq!(picker.results.len(), before);
    }

    #[test]
    fn the_window_holds_still_away_from_its_ends() {
        let total = WINDOW * 10;

        assert_eq!(
            slide_window(WINDOW, total, 0.5),
            Slide {
                start: WINDOW,
                snap: None
            }
        );
    }

    #[test]
    fn the_window_slides_at_its_ends_and_the_view_follows() {
        let total = WINDOW * 10;
        let compensation = STEP as f32 / WINDOW as f32;

        // Forward: the window advances and the view is nudged back by exactly
        // as much as the icons moved, so nothing appears to jump.
        let forward = slide_window(0, total, 0.9);
        assert_eq!(forward.start, STEP);
        assert_eq!(forward.snap, Some(0.9 - compensation));

        // And the landing point is clear of the trigger, so it cannot loop.
        assert!(forward.snap.expect("snapped") < LOAD_MORE_AT);

        let back = slide_window(STEP, total, 0.1);
        assert_eq!(back.start, 0);
        assert_eq!(back.snap, Some(0.1 + compensation));
        assert!(back.snap.expect("snapped") > LOAD_BACK_AT);
    }

    #[test]
    fn the_window_stops_at_both_ends_of_the_results() {
        let total = WINDOW * 10;
        let last = total - WINDOW;

        // Never past the final full window.
        assert_eq!(slide_window(last, total, 1.0).start, last);
        assert_eq!(slide_window(last - 1, total, 1.0).start, last);

        // Never before the first.
        assert_eq!(slide_window(0, total, 0.0).start, 0);
        assert!(slide_window(0, total, 0.0).snap.is_none());

        // A result set that fits entirely needs no window at all.
        assert_eq!(
            slide_window(0, WINDOW - 1, 1.0),
            Slide {
                start: 0,
                snap: None
            }
        );
    }

    #[test]
    fn changing_the_search_rewinds_the_window() {
        // As though the window had already been slid well along.
        let mut picker = Picker {
            start: WINDOW * 2,
            ..Picker::default()
        };

        let _ = picker.update(Message::Search("a".to_string()));

        assert_eq!(picker.start, 0);
    }

    /// The interface draws its own chrome from the index, so a rename upstream
    /// would silently blank a control.
    #[test]
    fn the_interface_icons_resolve() {
        assert!(CHECKMARK.is_some(), "fluent:checkmark-12 is missing");
        assert!(COPY_ICON.is_some(), "fluent:copy-20 is missing");
        assert!(COPIED_ICON.is_some(), "fluent:checkmark-20 is missing");
    }

    /// Ticking a family must not disturb the others: this is a multi-select.
    #[test]
    fn families_accumulate_rather_than_replace() {
        let mut picker = Picker::default();
        let [first, second] = [icon::FAMILIES[0].id, icon::FAMILIES[1].id];

        let _ = picker.update(Message::ToggleFamily(first));
        let _ = picker.update(Message::ToggleFamily(second));

        assert!(picker.families.contains(first));
        assert!(picker.families.contains(second));

        let _ = picker.update(Message::ToggleFamily(first));
        assert!(!picker.families.contains(first));
        assert!(picker.families.contains(second), "unticking took both");

        let _ = picker.update(Message::ShowAllFamilies);
        assert!(picker.families.is_empty());
    }

    /// The tick is looked up by name, so a rename upstream would silently drop it.
    #[test]
    fn the_selected_family_tick_resolves() {
        assert!(
            CHECKMARK.is_some(),
            "fluent:checkmark-12 is no longer in the index"
        );
    }

    /// Edits must not reach the buffer; selection actions must.
    #[test]
    fn the_preview_is_read_only() {
        let mut picker = Picker::default();
        let before = picker.snippet_content.text();

        let _ = picker.update(Message::SnippetAction(text_editor::Action::Edit(
            text_editor::Edit::Insert('x'),
        )));

        assert_eq!(picker.snippet_content.text(), before, "an edit got through");

        let _ = picker.update(Message::SnippetAction(text_editor::Action::SelectAll));

        assert!(picker.snippet_content.selection().is_some());
    }
}
