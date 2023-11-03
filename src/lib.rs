#![allow(clippy::missing_safety_doc)]

use std::ptr;

mod features;

pub mod console {
    mod console_js {
        #[link(wasm_import_module = "console")]
        extern "C" {
            pub fn console_log(ptr: *const u8, len: usize);
        }
    }

    pub fn console_log(ptr: *const u8, len: usize) {
        unsafe { console_js::console_log(ptr, len) }
    }
}

pub mod resulting_js {
    #[link(wasm_import_module = "resulting")]
    extern "C" {
        pub fn to_ron(ptr: *const u8, len: usize);
        pub fn from_ron(ptr: *const u8, len: usize);
        pub fn compress(ptr: *const u8, len: usize);
        pub fn decompress(ptr: *const u8, len: usize);
        pub fn error(ptr: *const u8, len: usize);
    }
}

pub fn log(s: &str) {
    console::console_log(s.as_ptr(), s.len());
}

pub fn error(s: &str) {
    unsafe {
        resulting_js::error(s.as_ptr(), s.len());
    }
}

#[no_mangle]
pub unsafe extern "C" fn to_ron(input: *mut u8, input_size: usize) {
    let cb = features::to_ron(input, input_size);

    if let Err(e) = cb {
        error(&e.to_string());
    }
}

#[no_mangle]
pub unsafe extern "C" fn from_ron(input: *const u8, input_size: usize) {
    let cb = features::from_ron(input, input_size);
    if let Err(e) = cb {
        error(&e.to_string());
    }
}

#[no_mangle]
pub unsafe extern "C" fn compress(
    input: *const u8,
    input_size: usize,
    level: u8,
) {
    let cb = features::compress(input, input_size, level);
    if let Err(e) = cb {
        error(&e.to_string());
    }
}

#[no_mangle]
pub unsafe extern "C" fn decompress(input: *const u8, input_size: usize) {
    let cb = features::decompress(input, input_size);
    if let Err(e) = cb {
        error(&e.to_string());
    }
}

#[no_mangle]
pub unsafe extern "C" fn __alloc(length: usize) -> *const u8 {
    let l = std::alloc::Layout::array::<u8>(length).unwrap();
    std::alloc::alloc(l)
}

#[no_mangle]
pub unsafe extern "C" fn __alloc_mut(length: usize) -> *mut u8 {
    let l = std::alloc::Layout::array::<u8>(length).unwrap();
    std::alloc::alloc(l)
}

#[no_mangle]
pub unsafe extern "C" fn __dealloc(ptr: *mut u8, length: usize) {
    let l = std::alloc::Layout::array::<u8>(length).unwrap();
    std::alloc::dealloc(ptr, l);
}
