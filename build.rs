use std::{fs, io};
use std::path::{Path, PathBuf};

//DOESNT WORK, manually load the resulting dll file

fn _copy(source_path: &PathBuf, target_path: &PathBuf) -> io::Result<()> {
    if let Err(e) = fs::copy(source_path, target_path) {
        println!(
            "cargo:warning=Failed to copy DLL: {} -> {} ({})",
            source_path.display(),
            target_path.display(),
            e
        );
    } else {
        println!("cargo:warning=DLL copied to: {}", target_path.display());
    }
    Ok(())
}

fn main() {
    println!("cargo:rustc-link-lib=dylib=xlink_tool");
    println!("cargo:rustc-link-search=native=lib");
    println!("cargo:rerun-if-changed=lib/xlink_tool.dll");
    println!("cargo:rerun-if-changed=lib/xlink_tool.lib");

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let cwd = std::env::current_dir().unwrap();
    let mut source_path = cwd.join("lib/xlink_tool.dll");

    // Check if we are building Debug or Release
    let profile = std::env::var("PROFILE").unwrap(); // "debug" or "release"

    // Target folder
    let target_dir: PathBuf = Path::new(&out_dir)
        .ancestors()
        .nth(4) // go one more up
        .unwrap()
        .join(&profile);

    let mut target_path: PathBuf = target_dir.join("xlink_tool.dll");
    _copy(&source_path, &target_path).unwrap();
    
    //lib/xlink_tool.lib
    source_path = cwd.join("lib/xlink_tool.lib");
    target_path = target_dir.join("xlink_tool.lib");

    _copy(&source_path, &target_path).unwrap();
}
