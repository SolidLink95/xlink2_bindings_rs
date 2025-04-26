use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_uchar};
use std::ptr;

//DOESNT WORK, manually load dll file

unsafe extern "C" {
    pub unsafe fn xlink_binary_to_yaml(data: *const u8, size: usize) -> *const c_char;
    pub unsafe fn xlink_yaml_to_binary(data: *const u8, size: usize, out_size: *mut usize) -> *mut u8;
    pub unsafe fn free_xlink_string(s: *const c_char);
    pub unsafe fn free_xlink_binary(ptr: *mut u8);
}

pub struct Xlink_rs;

impl Xlink_rs {
    pub fn binary_to_text(input: &[u8]) -> Option<String> {
        unsafe {
            let ptr = xlink_binary_to_yaml(input.as_ptr(), input.len());
            if ptr.is_null() {
                return None;
            }

            let c_str = CStr::from_ptr(ptr);
            let result = c_str.to_string_lossy().into_owned();
            free_xlink_string(ptr);
            Some(result)
        }
    }

    pub fn text_to_binary(data: &str) -> Option<Vec<u8>> {
        let input_bytes = data.as_bytes();
        let mut out_size = 0;

        unsafe {
            let ptr = xlink_yaml_to_binary(input_bytes.as_ptr(), input_bytes.len(), &mut out_size);
            if ptr.is_null() {
                return None;
            }

            let result = Vec::from_raw_parts(ptr, out_size, out_size);
            // Free manually after copying to Rust-owned Vec
            let vec = result.clone();
            std::mem::forget(result); // prevent double free
            free_xlink_binary(ptr);
            Some(vec)
        }
    }
}
