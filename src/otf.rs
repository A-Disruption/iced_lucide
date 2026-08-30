//! Low-level OpenType/TrueType container manipulation.
//!
//! This module knows how to take an sfnt binary apart into its table records,
//! put it back together, and synthesise the two tables a subsetter typically
//! discards: `cmap` (so glyphs can be found by codepoint) and `name` (so the
//! font can be found by family name).
//!
//! It deliberately depends on nothing but `std` — the `tools/vendor` maintainer
//! binary includes this same file directly via `#[path]` rather than depending
//! on the library, which would be circular: the library cannot compile until
//! the assets that `vendor` produces exist.

// The vendor tool uses one half of these helpers, the library the other.
// Neither should warn about the half it does not touch.
#![allow(dead_code)]

/// A parsed sfnt table: its 4-byte tag and raw contents.
pub type Table = ([u8; 4], Vec<u8>);

/// Split an sfnt binary into its tables, in table-directory order.
pub fn tables(font: &[u8]) -> Vec<Table> {
    if font.len() < 12 {
        return Vec::new();
    }

    let num_tables = u16::from_be_bytes([font[4], font[5]]) as usize;
    let mut out = Vec::with_capacity(num_tables);

    for i in 0..num_tables {
        let base = 12 + i * 16;
        if base + 16 > font.len() {
            break;
        }

        let Ok(tag) = <[u8; 4]>::try_from(&font[base..base + 4]) else {
            break;
        };
        let offset =
            u32::from_be_bytes(font[base + 8..base + 12].try_into().expect("4 bytes")) as usize;
        let length =
            u32::from_be_bytes(font[base + 12..base + 16].try_into().expect("4 bytes")) as usize;

        out.push((
            tag,
            font.get(offset..offset + length).unwrap_or(&[]).to_vec(),
        ));
    }

    out
}

/// The sfnt version tag at the head of the font (`0x00010000` or `OTTO`).
pub fn flavor(font: &[u8]) -> u32 {
    font.get(0..4)
        .and_then(|bytes| <[u8; 4]>::try_from(bytes).ok())
        .map(u32::from_be_bytes)
        .unwrap_or(0x0001_0000)
}

/// Extract a named table's raw bytes from an sfnt binary.
pub fn extract_table(font: &[u8], tag: &[u8; 4]) -> Option<Vec<u8>> {
    tables(font)
        .into_iter()
        .find(|(candidate, _)| candidate == tag)
        .map(|(_, data)| data)
}

/// Inject (or replace) a named table in an sfnt binary.
pub fn inject_table(font: &[u8], tag: &[u8; 4], table_data: &[u8]) -> Vec<u8> {
    if font.len() < 12 {
        return font.to_vec();
    }

    let mut tables = tables(font);

    tables.retain(|(candidate, _)| candidate != tag);
    tables.push((*tag, table_data.to_vec()));

    reconstruct(flavor(font), &mut tables)
}

/// Copy a table from `source` into `font`, but only if `font` lacks it.
///
/// Subsetters drop tables they consider irrelevant to their output format.
/// Some of those — `OS/2` and `post` in particular — are what a system font
/// database consults when deciding whether a face is usable at all.
pub fn carry_over(font: &[u8], source: &[u8], tag: &[u8; 4]) -> Vec<u8> {
    if extract_table(font, tag).is_some() {
        return font.to_vec();
    }

    match extract_table(source, tag) {
        Some(data) => inject_table(font, tag, &data),
        None => font.to_vec(),
    }
}

/// Rebuild a complete sfnt binary from a table list.
///
/// Tables are sorted by tag (required by the spec), padded to 4-byte
/// boundaries, and checksummed — including the whole-font checksum adjustment
/// written back into `head`.
pub fn reconstruct(flavor: u32, tables: &mut [Table]) -> Vec<u8> {
    // The OpenType spec requires table records sorted by tag.
    tables.sort_by_key(|(tag, _)| *tag);

    let count = tables.len() as u16;
    let entry_selector = if count > 0 {
        f64::from(count).log2().floor() as u16
    } else {
        0
    };
    let search_range = 2u16.pow(u32::from(entry_selector)) * 16;
    let range_shift = count * 16 - search_range;

    // Pre-compute each table's offset in the final binary.
    let directory_size = 12 + tables.len() * 16;
    let mut offsets = Vec::with_capacity(tables.len());
    let mut cursor = directory_size;
    for (_, data) in tables.iter() {
        offsets.push(cursor as u32);
        cursor += data.len();
        while !cursor.is_multiple_of(4) {
            cursor += 1;
        }
    }

    let mut font = Vec::with_capacity(cursor);

    // Offset table
    font.extend_from_slice(&flavor.to_be_bytes());
    font.extend_from_slice(&count.to_be_bytes());
    font.extend_from_slice(&search_range.to_be_bytes());
    font.extend_from_slice(&entry_selector.to_be_bytes());
    font.extend_from_slice(&range_shift.to_be_bytes());

    // Table directory. The checksum adjustment field inside `head` must be
    // zeroed before that table is checksummed.
    let mut head_adjustment_offset: Option<usize> = None;
    for ((tag, data), &offset) in tables.iter().zip(offsets.iter()) {
        let sum = if tag == b"head" && data.len() >= 12 {
            let mut zeroed = data.clone();
            zeroed[8..12].fill(0);
            checksum(&zeroed)
        } else {
            checksum(data)
        };

        font.extend_from_slice(tag);
        font.extend_from_slice(&sum.to_be_bytes());
        font.extend_from_slice(&offset.to_be_bytes());
        font.extend_from_slice(&(data.len() as u32).to_be_bytes());

        if tag == b"head" {
            head_adjustment_offset = Some(offset as usize + 8);
        }
    }

    // Table data
    for (tag, data) in tables.iter() {
        if tag == b"head" && data.len() >= 12 {
            font.extend_from_slice(&data[..8]);
            font.extend_from_slice(&[0u8; 4]); // zeroed for the whole-font checksum
            font.extend_from_slice(&data[12..]);
        } else {
            font.extend_from_slice(data);
        }

        while !font.len().is_multiple_of(4) {
            font.push(0);
        }
    }

    // head.checkSumAdjustment = 0xB1B0AFBA − (whole-font checksum).
    if let Some(index) = head_adjustment_offset {
        let value = 0xB1B0_AFBA_u32.wrapping_sub(checksum(&font));
        if index + 4 <= font.len() {
            font[index..index + 4].copy_from_slice(&value.to_be_bytes());
        }
    }

    font
}

/// OpenType table checksum: sum of big-endian `u32` words, zero-padding the tail.
pub fn checksum(data: &[u8]) -> u32 {
    let mut sum = 0u32;
    for chunk in data.chunks(4) {
        let mut bytes = [0u8; 4];
        bytes[..chunk.len()].copy_from_slice(chunk);
        sum = sum.wrapping_add(u32::from_be_bytes(bytes));
    }
    sum
}

/// Build a `cmap` table (format 12) mapping codepoints to glyph IDs.
///
/// Format 12 is used unconditionally rather than format 4 because several of
/// the supported families place glyphs above the Basic Multilingual Plane,
/// which format 4 cannot address.
pub fn build_cmap(entries: &mut Vec<(u32, u16)>) -> Vec<u8> {
    entries.sort_by_key(|&(codepoint, _)| codepoint);
    entries.dedup_by_key(|(codepoint, _)| *codepoint);

    let groups = entries.len() as u32;
    // format(2) + reserved(2) + length(4) + language(4) + numGroups(4) + n*12
    let subtable_len: u32 = 16 + groups * 12;

    let mut cmap = Vec::with_capacity(12 + subtable_len as usize);

    // cmap header
    cmap.extend_from_slice(&0u16.to_be_bytes()); // version
    cmap.extend_from_slice(&1u16.to_be_bytes()); // numTables

    // Encoding record: Windows / Unicode full repertoire
    cmap.extend_from_slice(&3u16.to_be_bytes()); // platformID
    cmap.extend_from_slice(&10u16.to_be_bytes()); // encodingID
    cmap.extend_from_slice(&12u32.to_be_bytes()); // offset: header(4) + record(8)

    // Format 12 subtable
    cmap.extend_from_slice(&12u16.to_be_bytes()); // format
    cmap.extend_from_slice(&0u16.to_be_bytes()); // reserved
    cmap.extend_from_slice(&subtable_len.to_be_bytes()); // length
    cmap.extend_from_slice(&0u32.to_be_bytes()); // language
    cmap.extend_from_slice(&groups.to_be_bytes()); // numGroups

    // One SequentialMapGroup per codepoint (startCharCode == endCharCode)
    for &(codepoint, glyph) in entries.iter() {
        cmap.extend_from_slice(&codepoint.to_be_bytes());
        cmap.extend_from_slice(&codepoint.to_be_bytes());
        cmap.extend_from_slice(&u32::from(glyph).to_be_bytes());
    }

    cmap
}

// ---------------------------------------------------------------------------
// name table
// ---------------------------------------------------------------------------

/// Name IDs carried over verbatim from the upstream font.
///
/// These are the records that carry legal meaning — copyright, trademark, and
/// the license text and URL — plus the credits for whoever drew the glyphs.
/// Rewriting a font's identity must never strip its attribution.
const PRESERVED_NAME_IDS: &[u16] = &[
    0,  // copyright notice
    7,  // trademark
    8,  // manufacturer
    9,  // designer
    10, // description
    11, // vendor URL
    12, // designer URL
    13, // license description
    14, // license info URL
];

/// Rewrite a font's `name` table so it is addressable by a family name we choose.
///
/// Two families in this crate ship as several faces that upstream gives a
/// *single* family name, distinguished only by weight — Font Awesome's solid
/// (900) and regular (400) both call themselves "Font Awesome 7 Free". A font
/// database asked for that family by name cannot be relied upon to return the
/// face we meant, so each subset is given its own unambiguous family instead.
///
/// Identity records (family, subfamily, unique ID, full name, PostScript name)
/// are replaced; the legal and credit records listed in [`PRESERVED_NAME_IDS`]
/// are copied through unchanged.
pub fn rewrite_name_table(font: &[u8], family: &str) -> Vec<u8> {
    let mut records: Vec<(u16, String)> = Vec::new();

    // Preserve the upstream legal and credit records.
    if let Some(existing) = extract_table(font, b"name") {
        for id in PRESERVED_NAME_IDS {
            if let Some(value) = read_name_record(&existing, *id) {
                records.push((*id, value));
            }
        }
    }

    // Our identity records.
    records.push((1, family.to_string()));
    records.push((2, "Regular".to_string()));
    records.push((3, format!("{family};iced_lucide")));
    records.push((4, family.to_string()));
    records.push((6, postscript_name(family)));

    records.sort_by_key(|(id, _)| *id);

    inject_table(font, b"name", &build_name_table(&records))
}

/// Read the first usable string for a name ID out of a `name` table.
///
/// Prefers the Windows/UTF-16BE encoding, falling back to Macintosh/ASCII.
fn read_name_record(name_table: &[u8], name_id: u16) -> Option<String> {
    if name_table.len() < 6 {
        return None;
    }

    let count = u16::from_be_bytes([name_table[2], name_table[3]]) as usize;
    let storage = u16::from_be_bytes([name_table[4], name_table[5]]) as usize;

    let mut fallback = None;

    for i in 0..count {
        let base = 6 + i * 12;
        if base + 12 > name_table.len() {
            break;
        }

        let platform = u16::from_be_bytes([name_table[base], name_table[base + 1]]);
        let id = u16::from_be_bytes([name_table[base + 6], name_table[base + 7]]);
        if id != name_id {
            continue;
        }

        let length = u16::from_be_bytes([name_table[base + 8], name_table[base + 9]]) as usize;
        let offset = u16::from_be_bytes([name_table[base + 10], name_table[base + 11]]) as usize;
        let Some(bytes) = name_table.get(storage + offset..storage + offset + length) else {
            continue;
        };

        match platform {
            // Windows: UTF-16BE
            3 => {
                let units: Vec<u16> = bytes
                    .chunks_exact(2)
                    .map(|pair| u16::from_be_bytes([pair[0], pair[1]]))
                    .collect();
                if let Ok(text) = String::from_utf16(&units) {
                    return Some(text);
                }
            }
            // Macintosh: treat as ASCII, which MacRoman agrees with for the
            // characters that actually appear in these records.
            _ => {
                if fallback.is_none() {
                    fallback = Some(bytes.iter().map(|&b| b as char).collect::<String>());
                }
            }
        }
    }

    fallback
}

/// Encode name records as a format 0 `name` table.
///
/// Every string is written twice — once for Macintosh/Roman and once for
/// Windows/Unicode BMP — because font databases differ in which they consult.
fn build_name_table(records: &[(u16, String)]) -> Vec<u8> {
    struct Encoded {
        platform: u16,
        encoding: u16,
        language: u16,
        name_id: u16,
        bytes: Vec<u8>,
    }

    let mut encoded = Vec::with_capacity(records.len() * 2);

    for (name_id, value) in records {
        // Macintosh / Roman / English
        encoded.push(Encoded {
            platform: 1,
            encoding: 0,
            language: 0,
            name_id: *name_id,
            bytes: value
                .chars()
                .map(|c| if c.is_ascii() { c as u8 } else { b'?' })
                .collect(),
        });

        // Windows / Unicode BMP / en-US
        encoded.push(Encoded {
            platform: 3,
            encoding: 1,
            language: 0x0409,
            name_id: *name_id,
            bytes: value
                .encode_utf16()
                .flat_map(|unit| unit.to_be_bytes())
                .collect(),
        });
    }

    // Records must be sorted by platform, encoding, language, then name ID.
    encoded.sort_by_key(|record| {
        (
            record.platform,
            record.encoding,
            record.language,
            record.name_id,
        )
    });

    let count = encoded.len();
    let storage_offset = 6 + count * 12;

    let mut table = Vec::with_capacity(storage_offset);
    table.extend_from_slice(&0u16.to_be_bytes()); // format 0
    table.extend_from_slice(&(count as u16).to_be_bytes());
    table.extend_from_slice(&(storage_offset as u16).to_be_bytes());

    let mut storage = Vec::new();
    for record in &encoded {
        table.extend_from_slice(&record.platform.to_be_bytes());
        table.extend_from_slice(&record.encoding.to_be_bytes());
        table.extend_from_slice(&record.language.to_be_bytes());
        table.extend_from_slice(&record.name_id.to_be_bytes());
        table.extend_from_slice(&(record.bytes.len() as u16).to_be_bytes());
        table.extend_from_slice(&(storage.len() as u16).to_be_bytes());
        storage.extend_from_slice(&record.bytes);
    }

    table.extend_from_slice(&storage);
    table
}

/// Derive a valid PostScript name from a family name.
///
/// PostScript names admit neither spaces nor the ten characters the spec
/// reserves for delimiters, and are capped at 63 bytes.
fn postscript_name(family: &str) -> String {
    const FORBIDDEN: &[char] = &['(', ')', '{', '}', '[', ']', '<', '>', '/', '%', ' '];

    family
        .chars()
        .filter(|c| c.is_ascii_graphic() && !FORBIDDEN.contains(c))
        .take(63)
        .collect()
}
