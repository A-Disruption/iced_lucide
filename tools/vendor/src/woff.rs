//! WOFF 1.0 to sfnt decoding.
//!
//! Bootstrap Icons publishes WOFF and WOFF2 only, but a font database wants a
//! plain sfnt. WOFF 1.0 is a thin container — a header, a table directory, and
//! each table individually zlib-compressed — so unwrapping it here lets the
//! repository vendor an ordinary `.ttf` with untouched outlines.
//!
//! WOFF2 is deliberately not supported: it applies lossy-looking glyph
//! transforms that must be reversed, and no upstream here requires it.

use std::io::Read;

use flate2::read::ZlibDecoder;

use crate::otf;

const HEADER_LEN: usize = 44;
const DIRECTORY_ENTRY_LEN: usize = 20;

pub fn to_sfnt(woff: &[u8]) -> Result<Vec<u8>, String> {
    if woff.len() < HEADER_LEN || &woff[0..4] != b"wOFF" {
        return Err("not a WOFF 1.0 file".to_string());
    }

    let flavor = u32::from_be_bytes(woff[4..8].try_into().expect("4 bytes"));
    let num_tables = u16::from_be_bytes([woff[12], woff[13]]) as usize;

    let mut tables: Vec<otf::Table> = Vec::with_capacity(num_tables);

    for index in 0..num_tables {
        let base = HEADER_LEN + index * DIRECTORY_ENTRY_LEN;
        let entry = woff
            .get(base..base + DIRECTORY_ENTRY_LEN)
            .ok_or_else(|| format!("table directory entry {index} is truncated"))?;

        let tag: [u8; 4] = entry[0..4].try_into().expect("4 bytes");
        let offset = u32::from_be_bytes(entry[4..8].try_into().expect("4 bytes")) as usize;
        let compressed_len = u32::from_be_bytes(entry[8..12].try_into().expect("4 bytes")) as usize;
        let original_len = u32::from_be_bytes(entry[12..16].try_into().expect("4 bytes")) as usize;

        let stored = woff
            .get(offset..offset + compressed_len)
            .ok_or_else(|| format!("table {} is truncated", String::from_utf8_lossy(&tag)))?;

        // A table is stored uncompressed when compression would not have
        // shrunk it, which the spec signals by equal lengths.
        let data = if compressed_len == original_len {
            stored.to_vec()
        } else {
            let mut decoded = Vec::with_capacity(original_len);
            ZlibDecoder::new(stored)
                .read_to_end(&mut decoded)
                .map_err(|error| format!("inflate {}: {error}", String::from_utf8_lossy(&tag)))?;
            decoded
        };

        if data.len() != original_len {
            return Err(format!(
                "table {} decoded to {} bytes, expected {original_len}",
                String::from_utf8_lossy(&tag),
                data.len()
            ));
        }

        tables.push((tag, data));
    }

    Ok(otf::reconstruct(flavor, &mut tables))
}
