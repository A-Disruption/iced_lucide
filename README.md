# iced_lucide

A compile-time, type-safe [Lucide](https://lucide.dev) icon font library for [`iced`](https://github.com/iced-rs/iced).

No network calls. The Lucide TTF and icon map are bundled directly into the library.

## Usage

Create a `.toml` file in your crate with the icons you need:

```toml
# fonts/my-icons.toml
module = "icon"

[icons]
edit   = "pencil"
save   = "save"
trash  = "trash-2"
search = "search"
```

Each key is the Rust function name; each value is the [Lucide icon name](https://lucide.dev/icons).

Add `iced_lucide` to your `build-dependencies`:

```toml
[build-dependencies]
iced_lucide = { git = "https://github.com/A-Disruption/iced_lucide" }
```

Call `build` in your `build.rs`:

```rust
pub fn main() {
    println!("cargo::rerun-if-changed=fonts/my-icons.toml");
    iced_lucide::build("fonts/my-icons.toml").expect("Build icon module");
}
```

This generates `src/icon.rs` and copies `lucide.ttf` next to your TOML.

Register the font and use the generated functions:

```rust
mod icon;

fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .font(icon::FONT)
        .run()
}

fn view(&self) -> iced::Element<'_, ()> {
    iced::widget::row![icon::edit(), icon::save(), icon::trash()]
        .spacing(10)
        .into()
}
```

## All Icons (icon picker)

To generate every Lucide icon at once — useful for picker widgets or UI builders:

```rust
pub fn main() {
    iced_lucide::build_all("icon").expect("Build all icons");
}
```

The generated module exports `ALL_ICONS: &[(&str, &str)]`, a static list of
`(icon_name, codepoint_str)` pairs you can iterate at runtime:

```rust
use crate::icon::ALL_ICONS;

// in your view:
let buttons: Vec<_> = ALL_ICONS
    .iter()
    .map(|(name, cp)| button(text(*cp)).on_press(Msg::Pick(name.to_string())))
    .collect();
```

## Runtime enumeration

Add `iced_lucide` as a regular `[dependency]` (not just build) to call
`iced_lucide::icons()` at runtime:

```rust
for (name, codepoint) in iced_lucide::icons() {
    println!("{name}  U+{codepoint:04X}");
}
```

## Updating Lucide

Both the TTF and the icon map are bundled in `assets/`. To update to a newer
Lucide release, replace both files together and rebuild:

```bash
curl -L -o assets/lucide.ttf   https://cdn.jsdelivr.net/npm/lucide-static/font/lucide.ttf
curl -L -o assets/unicode.html https://cdn.jsdelivr.net/npm/lucide-static/font/unicode.html
```

The hash-based cache in the build script detects the change and regenerates
your icon module automatically.

## Example

See the [`example`](example) directory for a full working application.

```
cargo run --manifest-path example/Cargo.toml
```
