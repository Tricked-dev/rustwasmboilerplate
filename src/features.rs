use crate::{__alloc_mut, __dealloc, log, resulting_js};

use std::ptr;

use miniz_oxide::{deflate::compress_to_vec, inflate::decompress_to_vec};
use serde_json::Value;

#[inline]
pub unsafe fn to_ron(
    input: *const u8,
    input_size: usize,
) -> anyhow::Result<()> {
    let input_data = std::slice::from_raw_parts(input, input_size);

    let file: Value = serde_json::from_slice(&input_data)?;

    let binding =
        ron::ser::to_string_pretty(&file, ron::ser::PrettyConfig::default())?;
    let ron_data = binding.as_bytes();
    let len = ron_data.len();

    let alloc_ptr = __alloc_mut(len);

    ptr::copy_nonoverlapping(ron_data.as_ptr(), alloc_ptr, len);

    resulting_js::to_ron(alloc_ptr, len);

    __dealloc(input as *mut u8, input_size);

    Ok(())
}

#[inline]
pub unsafe fn from_ron(
    input: *const u8,
    input_size: usize,
) -> anyhow::Result<()> {
    let input_data = std::slice::from_raw_parts(input, input_size);

    let file: Value = ron::from_str(&String::from_utf8_lossy(input_data))?;

    let json_data = serde_json::to_vec(&file)?;
    let len = json_data.len();

    let alloc_ptr = __alloc_mut(len);

    ptr::copy_nonoverlapping(json_data.as_ptr(), alloc_ptr, len);

    resulting_js::from_ron(alloc_ptr, len);

    __dealloc(input as *mut u8, input_size);

    Ok(())
}

pub unsafe fn compress(
    input: *const u8,
    input_size: usize,
    level: u8,
) -> anyhow::Result<()> {
    let input_data = std::slice::from_raw_parts(input, input_size);

    let compressed = compress_to_vec(input_data, level);
    let len = compressed.len();

    let alloc_ptr = __alloc_mut(len);

    ptr::copy_nonoverlapping(compressed.as_ptr(), alloc_ptr, len);

    resulting_js::compress(alloc_ptr, len);

    __dealloc(input as *mut u8, input_size);

    Ok(())
}

pub unsafe fn decompress(
    input: *const u8,
    input_size: usize,
) -> anyhow::Result<()> {
    let input_data = std::slice::from_raw_parts(input, input_size);

    let decompressed = decompress_to_vec(input_data)?;
    let len = decompressed.len();

    let alloc_ptr = __alloc_mut(len);

    ptr::copy_nonoverlapping(decompressed.as_ptr(), alloc_ptr, len);

    resulting_js::decompress(alloc_ptr, len);

    __dealloc(input as *mut u8, input_size);

    Ok(())
}
