# iced_lucide

Compile-time, type-safe icon fonts for [`iced`](https://github.com/iced-rs/iced).

Pick the icons you want in a TOML file. This crate cuts a font containing
exactly those glyphs and generates a typed function for each one. A project
using a dozen icons ships a font holding a dozen glyphs — not the several
thousand its upstream publishes.

No network calls. Every font and its icon index are vendored into the crate.

> **On the name:** this started as a Lucide-only crate and kept its name through
> the move to twenty-six icon sets, so existing users are not stranded. Lucide is still
> the default family.

## Icon sets

Each family sits behind its own Cargo feature, so a build embeds only the fonts
it asks for.

| Feature | Set | Identifier(s) | Icons | License |
|---|---|---|---:|---|
| `lucide` *(default)* | [Lucide](https://lucide.dev/icons) | `lucide` | 2048 | ISC |
| `bootstrap` | [Bootstrap Icons](https://icons.getbootstrap.com) | `bootstrap` | 2078 | MIT |
| `codicon` | [VS Code Codicons](https://microsoft.github.io/vscode-codicons/dist/codicon.html) | `codicon` | 746 | CC-BY-4.0 |
| `devicon` | [Devicon](https://devicon.dev) | `devicon` | 1491 | MIT |
| `fontawesome` | [Font Awesome Free](https://fontawesome.com/icons/packs/classic) | `fa-solid`, `fa-regular`, `fa-brands` | 2054 | CC-BY-4.0 / OFL-1.1 |
| `nerdfonts` | [Nerd Fonts](https://www.nerdfonts.com) | `nerdfonts` | 10995 | MIT |
| `octicons` | [Octicons](https://primer.style/octicons) | `octicons` | 310 | MIT |
| `pomicons` | [Pomicons](https://github.com/gabrielelana/pomicons) | `pomicons` | 11 | see vendored `LICENSE` |
| `material_symbols` | [Material Symbols](https://fonts.google.com/icons) | `material-symbols` | 4275 | Apache-2.0 |
| `material_design_icons` | [Material Design Icons](https://pictogrammers.com/library/mdi/) | `material-design-icons` | 7447 | Apache-2.0 |
| `phosphor` | [Phosphor](https://phosphoricons.com) | `phosphor` | 1530 | MIT |
| `tabler` | [Tabler Icons](https://tabler.io/icons) | `tabler` | 5193 | MIT |
| `fluent` | [Fluent System Icons](https://github.com/microsoft/fluentui-system-icons) | `fluent` | 9708 | MIT |
| `simple_icons` | [Simple Icons](https://simpleicons.org) | `simple-icons` | 3457 | CC0-1.0 |
| `boxicons` | [Boxicons](https://boxicons.com) | `boxicons` | 1634 | MIT |

### Extra weights and styles

Several sets publish more than one face. Each is a separate font, so each is a
separate family behind its own feature — the base feature above stays a single
face, and you download only the weights you actually use.

| Feature | Adds | Icons |
|---|---|---:|
| `material_symbols_styles` | `material-symbols-rounded`, `material-symbols-sharp` | 8550 |
| `phosphor_weights` | `phosphor-thin`, `-light`, `-bold`, `-fill`, `-duotone` | 7632 |
| `tabler_filled` | `tabler-filled` | 1057 |
| `fluent_filled` | `fluent-filled` | 9833 |

These are separate *families*, not weights on an existing one:

```toml
[icons.phosphor-bold]
save = "floppy-disk"
```

That is deliberate. Naming the face means asking for one that is not enabled is
a build error, where `Font::new("phosphor").weight(Bold)` against a family that
only has Regular loaded would quietly render Regular instead.

That is **80,049 icons** across 26 families. Nothing is enabled but Lucide
unless you ask for it.

Enable what you need:

```toml
[build-dependencies]
iced_lucide = { version = "0.2", features = ["bootstrap", "fontawesome"] }
```

`features = ["all"]` turns on everything.

### Caveats worth knowing

- **Octicons** — GitHub removed the icon font at Octicons v9, and
  `primer.style/octicons` is SVG-only today. The glyphs are taken from the Nerd
  Fonts `oct-` range instead, which tracks a far later version than the last
  standalone font did (310 icons against 172), and cut back out into a font of
  their own. Names use Primer's spelling — `arrow-down`, not the
  `oct-arrow_down` Nerd Fonts writes.
- **Font Awesome** — brand marks are trademarks of their owners; the license
  does not grant any right to use them.
- **Devicon** — the multi-colour `original` variants exist only as SVG. The font
  carries the single-colour `plain` and `line` variants.
- **Nerd Fonts** — aggregates glyphs from many upstream sets, each under its own
  license. Names keep their upstream prefix (`cod-`, `dev-`, `fa-`, `oct-`, …).
- **Material Symbols** — Google publishes this only as a four-axis variable
  font. The vendored copy is its default instance: the Outlined style at weight
  400. The fill, grade, and optical-size axes do not survive vendoring.
- **Fluent** — Microsoft draws each icon separately per pixel size, so names
  keep theirs: `access-time-24`, not `access-time`. This is the Regular style.
- **Simple Icons** — the font is CC0, but the brand marks in it are trademarks
  of their owners and the license grants no right to use them.
- **Boxicons** — regular, solid, and logo styles share one font, so solid and
  logo names are prefixed: `alarm`, `solid-alarm`, `logo-github`.
- **Phosphor** — the base feature is the Regular weight; `phosphor_weights` adds
  the other five. Duotone draws two tones from one glyph, so it renders as a
  single flat colour here rather than the layered look the web font achieves.
- **Iconoir is not supported.** It has no icon font — its stylesheet draws every
  icon from an inline SVG data URI, so there is nothing to subset.

## Usage

Create a `.toml` file describing the icons you want:

```toml
# fonts/my-icons.toml
module = "icon"

[icons]
edit   = "pencil"
save   = "save"
trash  = "trash-2"

# A "family:" prefix pulls one icon from another set.
github = "fa-brands:github"

# Or group several under one family.
[icons.bootstrap]
bluetooth = "bluetooth"
wifi      = "wifi"
```

Each key is the Rust function name; each value is the upstream icon name.
Unprefixed names come from the `family` key if you set one, and otherwise from
Lucide:

```toml
module = "icon"
family = "bootstrap"   # now bare names mean Bootstrap Icons

[icons]
save = "floppy"
```

Call `build` in your `build.rs`:

```rust
pub fn main() {
    println!("cargo::rerun-if-changed=fonts/my-icons.toml");
    iced_lucide::build("fonts/my-icons.toml").expect("Build icon module");
}
```

This generates `src/icon.rs` and writes one subset `.ttf` per family used, next
to the TOML.

Register the fonts and use the generated functions:

```rust
mod icon;

fn main() -> iced::Result {
    let mut app = iced::application(App::default, App::update, App::view);

    for font in icon::FONTS {
        app = app.font(*font);
    }

    app.run()
}

fn view(&self) -> iced::Element<'_, ()> {
    iced::widget::row![icon::edit(), icon::save(), icon::github()]
        .spacing(10)
        .into()
}
```

A misspelled icon fails the build with suggestions rather than rendering a blank
box at runtime:

```
Lucide has no icon "penci".
Did you mean: pencil, pencil-off, pencil-line, pencil-ruler, pen?
Browse all icons at https://lucide.dev/icons
```

## What the generated module exports

| Item | Purpose |
|---|---|
| `FONTS: &[&[u8]]` | Every subset font, ready to register |
| `FONT: &[u8]` | The single font, when only one family is used |
| `Family` | `{ id, name, feature, license, url }` |
| `FAMILIES: &[Family]` | The sets used — for a filter UI, or an about screen |
| `Icon` | `{ name, codepoint, font, family }` |
| `ALL_ICONS: &[Icon]` | Every icon in the module |
| `render(icon: Icon) -> Text` | Draw any `Icon` |
| `find(name: &str) -> Option<Icon>` | Look one up by upstream name |
| one `fn` per icon | e.g. `icon::edit()` |

## All icons (icon picker)

To generate an entire family at once — useful for picker widgets and UI
builders:

```rust
pub fn main() {
    iced_lucide::build_all("lucide", "icon").expect("Build all icons");
}
```

```rust
use crate::icon::ALL_ICONS;

let buttons: Vec<_> = ALL_ICONS
    .iter()
    .map(|icon| button(icon::render(*icon)).on_press(Msg::Pick(icon.name)))
    .collect();
```

This emits one function per icon, so pointing it at a very large family produces
a correspondingly large module — Nerd Fonts would generate almost eleven
thousand of them.

## Browsing several families

For a picker spanning more than one set, `build_index` generates the same
module *without* the per-icon functions — naming fifty thousand icons in Rust
helps nobody:

```rust
pub fn main() {
    // An empty slice means every family the enabled features provide.
    iced_lucide::build_index(&[], "icon").expect("Build icon index");
}
```

Each `Icon` carries the `family` it came from, and `FAMILIES` lists them, so
filtering is straightforward:

```rust
let shown = icon::ALL_ICONS
    .iter()
    .filter(|i| active.is_empty() || active.contains(i.family))
    .filter(|i| i.name.contains(&search));
```

`Family` also carries the Cargo `feature` that provides it, which is what lets
the picker below write out a manifest entry for whatever you selected.

## Runtime enumeration

Add `iced_lucide` as a regular `[dependency]` (not just a build one) to inspect
what it carries:

```rust
for family in iced_lucide::families() {
    println!("{} — {} icons", family.label(), family.icons().len());
}

let lucide = iced_lucide::family("lucide").unwrap();

for (name, codepoint) in lucide.icons() {
    println!("{name}  U+{codepoint:04X}");
}
```

## Attribution

Subsetting rewrites a font's identity so it can be addressed by a name we
choose, but it carries the upstream copyright, trademark, and license records
through untouched. The full license text for every set is vendored in
`assets/<family>/LICENSE`, and the generated module exposes `LICENSES` so an
about screen can show it. **Shipping these fonts means shipping their licenses.**

## Updating a family

The vendoring tool fetches each upstream, normalises its metadata, and rewrites
`assets/`:

```bash
cargo run -p vendor
```

Pass a family to refresh just one:

```bash
cargo run -p vendor -- lucide
```

Downloads are cached in `tools/vendor/.cache/`, so `--offline` rebuilds from
what is already there. Adding a new icon set means appending an entry to
`tools/vendor/src/sources.rs` and re-running the tool — it regenerates
`src/families.rs` as well as the assets.

## Examples

```bash
cargo run -p subset_example        # several families in one module
cargo run -p all_icons_example     # a grid of every Lucide icon
cargo run -p icon_picker_example   # browse all 26 families
```

`icon_picker_example` is the one to reach for when deciding what to use: search
across all 80,049 icons, filter by family, click to collect them in a side
panel, and recolour the lot. The family and colour dropdowns are built on the
[`popover`](https://github.com/A-Disruption/widgets) widget.

The toolbar carries a search box, a **family filter** that narrows a
multi-select list as you type, a **size dropdown** from 12 to 64 px so you can
see how an icon reads at the size you will actually use it, and a **colour
chip** offering the theme's own colour or a handful of overrides.

It will also write your selection out for you. The export panel offers three
snippets. Each is shown in a read-only editor you can select and copy a line at
a time — useful when the target file already has entries — or take whole with
**Copy all**:

| Snippet | What you get |
|---|---|
| `my-icons.toml` | Your picks as a definition, grouped by family, with the licenses noted |
| `build.rs` | The build script that consumes it |
| `Cargo.toml` | A `build-dependencies` entry with exactly the features your picks need |

So updating the icons in a project is: open the picker, select, copy, paste.
Function names are generated with `iced_lucide::function_name`, the same rule
the build script applies, and names that collide across families — `search`
turns up in most of them — get a numeric suffix so the definition stays valid.

Every crate in this workspace builds against one pinned iced revision, declared
once in the root `[workspace.dependencies]` — the colour picker and the examples
have to agree on it or their `Element` types will not match.

## Migrating from 0.1

- `build_all("icon")` is now `build_all("lucide", "icon")`.
- `ALL_ICONS` is `&[Icon]` rather than `&[(&str, &str)]`. Each `Icon` carries
  its own font, which is what makes mixing families possible.
- `render` takes an `Icon` instead of a codepoint string: `render(*icon)`.
- Register with `for font in icon::FONTS { app = app.font(*font); }`. `FONT`
  still exists for single-family modules.
- `iced_lucide::icons()` and `FONT_BYTES` still work but are deprecated in
  favour of `family("lucide")`.

## License

MIT for this crate. The vendored icon fonts keep their own licenses — see the
table above and `assets/<family>/LICENSE`.
