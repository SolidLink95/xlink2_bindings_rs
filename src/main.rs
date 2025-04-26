use std::ffi::{CStr, CString};
use std::fs;
use std::path::Path;
use libloading::{Library, Symbol};

type XlinkBinaryToYaml = unsafe extern "C" fn(data: *const i8, size: usize) -> *const i8;
type XlinkYamlToBinary = unsafe extern "C" fn(data: *const i8, size: usize, out_size: *mut usize) -> *mut i8;
type FreeXlinkBinary = unsafe extern "C" fn(data: *mut i8);
type FreeXlinkString = unsafe extern "C" fn(str_: *mut i8);


fn main() {
    let dll_path = Path::new("W:/coding/xlink2_bindings_rs/lib/xlink_tool.dll");
    unsafe {
        let lib = Library::new(dll_path).expect("Failed to load DLL");

        let xlink_binary_to_yaml: Symbol<XlinkBinaryToYaml> = lib.get(b"xlink_binary_to_yaml\0").unwrap();
        let xlink_yaml_to_binary: Symbol<XlinkYamlToBinary> = lib.get(b"xlink_yaml_to_binary\0").unwrap();
        let free_xlink_binary: Symbol<FreeXlinkBinary> = lib.get(b"free_xlink_binary\0").unwrap();
        let free_xlink_string: Symbol<FreeXlinkString> = lib.get(b"free_xlink_string\0").unwrap();

        // Example usage
        let rawdata = fs::read("tmp/elink2.Product.110.belnk").unwrap();
        // let example_binary = b"FAKE_BINARY_DATA";
        let c_binary = rawdata.as_ptr() as *const i8;

        let yaml_ptr = (xlink_binary_to_yaml)(c_binary, rawdata.len());
        if !yaml_ptr.is_null() {
            let yaml_cstr = CStr::from_ptr(yaml_ptr);
            let yaml_str = yaml_cstr.to_string_lossy().into_owned();
            fs::write("tmp/output.yaml", yaml_str.clone()).unwrap();
            // println!("YAML output: {}", yaml_cstr.to_string_lossy());
            (free_xlink_string)(yaml_ptr as *mut i8);
        }

        // Convert back
        // let yaml_data = "fake_yaml_data";
        // let yaml_cstring = CString::new(yaml_data).unwrap();
        // let mut out_size: usize = 0;
        // let bin_ptr = (xlink_yaml_to_binary)(yaml_cstring.as_ptr(), yaml_data.len(), &mut out_size);

        // if !bin_ptr.is_null() {
        //     let slice = std::slice::from_raw_parts(bin_ptr as *const u8, out_size);
        //     println!("Binary output: {:?}", slice);
        //     (free_xlink_binary)(bin_ptr);
        // }
    }
}
