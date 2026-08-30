//! Maintainer tool: refresh the vendored icon assets.
//!
//! ```text
//! cargo run -p vendor                # every family
//! cargo run -p vendor -- lucide      # just one
//! cargo run -p vendor -- --offline   # rebuild from the download cache
//! ```
//!
//! For each family this downloads the upstream font and its metadata, decodes
//! the font into a plain sfnt if it arrived wrapped, discards any name whose
//! glyph is not actually present in the font, and writes the results into
//! `assets/<family>/`. The library reads nothing else.

// The OTF primitives are shared with the library by including its source
// directly. A dependency would be circular: the library embeds the assets this
// tool produces, so it cannot compile before this tool has run.
#[path = "../../../src/otf.rs"]
mod otf;

// Likewise for the subsetter, so a family carved out of a larger font is cut
// by exactly the same code that cuts a project's icons at build time.
#[path = "../../../src/subset.rs"]
mod subset;

mod parse;
mod registry;
mod sources;
mod woff;

use std::collections::BTreeMap;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

use sources::{Container, SOURCES, Source};

fn main() {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let offline = arguments.iter().any(|argument| argument == "--offline");
    let wanted: Vec<&str> = arguments
        .iter()
        .filter(|argument| !argument.starts_with("--"))
        .map(String::as_str)
        .collect();

    let root = repository_root();
    let cache = root.join("tools/vendor/.cache");
    fs::create_dir_all(&cache).expect("create download cache");

    let selected: Vec<&Source> = SOURCES
        .iter()
        .filter(|source| wanted.is_empty() || wanted.contains(&source.id))
        .collect();

    if selected.is_empty() {
        eprintln!("No family matched {wanted:?}.");
        eprintln!(
            "Known families: {}",
            SOURCES
                .iter()
                .map(|source| source.id)
                .collect::<Vec<_>>()
                .join(", ")
        );
        std::process::exit(1);
    }

    let mut failures = Vec::new();

    for source in selected {
        print!("{:<12} ", source.id);
        match vendor(source, &root, &cache, offline) {
            Ok(report) => println!("{report}"),
            Err(error) => {
                println!("FAILED");
                eprintln!("  {error}");
                failures.push(source.id);
            }
        }
    }

    if !failures.is_empty() {
        eprintln!("\nFailed: {}", failures.join(", "));
        std::process::exit(1);
    }

    // The registry describes every family, not just the ones refreshed on this
    // run, so it is rewritten unconditionally from the source table.
    let registry = root.join("src/families.rs");
    fs::write(&registry, registry::render()).expect("write src/families.rs");
    println!("\nwrote {}", registry.display());
}

fn vendor(source: &Source, root: &Path, cache: &Path, offline: bool) -> Result<String, String> {
    let font_bytes = fetch(source.font_url, cache, offline)?;
    let metadata_bytes = fetch(source.metadata_url, cache, offline)?;

    // Nerd Fonts ships its license inside the archive rather than in the repo,
    // so it has to be lifted out while the archive is still to hand.
    let mut archive_license = None;

    let font = match &source.container {
        Container::Sfnt => font_bytes,
        Container::Woff => woff::to_sfnt(&font_bytes)?,
        Container::Zip { entry } => {
            archive_license = unzip(&font_bytes, "LICENSE").ok();
            unzip(&font_bytes, entry)?
        }
    };

    let face = ttf_parser::Face::parse(&font, 0)
        .map_err(|error| format!("{} is not a usable font: {error}", source.font_url))?;

    let parsed = parse::parse(&source.metadata, &metadata_bytes)?;
    if parsed.is_empty() {
        return Err(format!("no icons parsed from {}", source.metadata_url));
    }

    // Upstream metadata routinely lists names the shipped font does not carry —
    // icons that are SVG-only, or were removed from the font but not the
    // stylesheet. Keeping them would turn a typo-checking build error into a
    // blank glyph at runtime, so they are dropped here instead.
    let mut icons = BTreeMap::new();
    let mut missing = 0usize;

    for (name, codepoint) in parsed {
        let present = char::from_u32(codepoint)
            .and_then(|character| face.glyph_index(character))
            .is_some();

        if present {
            icons.insert(name, codepoint);
        } else {
            missing += 1;
        }
    }

    if icons.is_empty() {
        return Err(format!(
            "none of the {missing} parsed names resolve to a glyph in the font"
        ));
    }

    // A family carved out of a larger font keeps only the glyphs it named, and
    // takes on its own family name so it can be addressed independently of the
    // font it came from.
    let vendored = if source.trim_to_index {
        let codepoints: Vec<u32> = icons.values().copied().collect();
        subset::subset(&font, &codepoints, source.font_family)
    } else {
        font
    };

    let directory = root.join("assets").join(source.id);
    fs::create_dir_all(&directory).map_err(|error| format!("create {directory:?}: {error}"))?;

    let font_path = directory.join(format!("{}.ttf", source.file_stem));
    fs::write(&font_path, &vendored).map_err(|error| format!("write {font_path:?}: {error}"))?;

    fs::write(directory.join("icons.idx"), render_index(source, &icons))
        .map_err(|error| format!("write icons.idx: {error}"))?;

    if let Some(url) = source.license_url {
        match fetch(url, cache, offline) {
            Ok(text) => fs::write(directory.join("LICENSE"), text)
                .map_err(|error| format!("write LICENSE: {error}"))?,
            Err(error) => eprintln!("  warning: could not fetch LICENSE ({error})"),
        }
    }

    if let Some(text) = archive_license.filter(|_| source.license_url.is_none()) {
        fs::write(directory.join("LICENSE"), text)
            .map_err(|error| format!("write LICENSE: {error}"))?;
    }

    Ok(format!(
        "{:>6} icons  {:>8} KB{}",
        icons.len(),
        vendored.len() / 1024,
        if missing > 0 {
            format!("  ({missing} names had no glyph, dropped)")
        } else {
            String::new()
        }
    ))
}

/// Render the normalised index the library reads at build time.
///
/// One `name<TAB>hex` line per icon, sorted, with the provenance recorded in
/// leading comments so a vendored asset can always be traced to its source.
fn render_index(source: &Source, icons: &BTreeMap<String, u32>) -> String {
    let mut out = String::new();

    out.push_str(&format!("# {} ({})\n", source.label, source.id));
    out.push_str(&format!("# font: {}\n", source.font_url));
    out.push_str(&format!("# metadata: {}\n", source.metadata_url));
    out.push_str(&format!("# license: {}\n", source.license));

    if let Some(note) = source.note {
        for line in note.split_inclusive(' ').collect::<Vec<_>>().chunks(12) {
            out.push_str(&format!("# note: {}\n", line.concat().trim()));
        }
    }

    out.push_str("# Regenerate with: cargo run -p vendor -- ");
    out.push_str(source.id);
    out.push('\n');

    for (name, codepoint) in icons {
        out.push_str(&format!("{name}\t{codepoint:x}\n"));
    }

    out
}

/// Download a URL, caching the body so repeated runs stay offline-friendly.
fn fetch(url: &str, cache: &Path, offline: bool) -> Result<Vec<u8>, String> {
    let path = cache.join(cache_key(url));

    if path.exists() {
        return fs::read(&path).map_err(|error| format!("read cache {path:?}: {error}"));
    }

    if offline {
        return Err(format!("{url} is not cached and --offline was given"));
    }

    let mut response = ureq::get(url)
        .call()
        .map_err(|error| format!("GET {url}: {error}"))?;

    let mut body = Vec::new();
    response
        .body_mut()
        .as_reader()
        .read_to_end(&mut body)
        .map_err(|error| format!("read body of {url}: {error}"))?;

    fs::write(&path, &body).map_err(|error| format!("write cache {path:?}: {error}"))?;

    Ok(body)
}

/// A filesystem-safe cache filename that still hints at its origin.
fn cache_key(url: &str) -> String {
    let sanitised: String = url
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect();

    // Keep the tail: the leading scheme and host repeat across every entry,
    // while the filename at the end is what distinguishes them.
    let tail = sanitised.len().saturating_sub(90);
    sanitised[tail..].to_string()
}

fn unzip(archive: &[u8], entry: &str) -> Result<Vec<u8>, String> {
    let mut zip = zip::ZipArchive::new(std::io::Cursor::new(archive))
        .map_err(|error| format!("open archive: {error}"))?;

    let mut file = zip
        .by_name(entry)
        .map_err(|error| format!("{entry} in archive: {error}"))?;

    let mut out = Vec::new();
    file.read_to_end(&mut out)
        .map_err(|error| format!("read {entry}: {error}"))?;

    Ok(out)
}

/// The repository root, resolved from this crate's location rather than the
/// working directory so the tool behaves the same from anywhere.
fn repository_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("tools/vendor sits two levels below the repository root")
        .to_path_buf()
}
