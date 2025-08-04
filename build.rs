use std::env;
use std::path::PathBuf;

fn main() {
    let lib_dir = match env::var("ETHERCAT_LIB_DIR") {
        Ok(dir) => PathBuf::from(dir),
        Err(_) => PathBuf::from("/usr/local/lib")
    }.canonicalize().expect("");

    if !lib_dir.join("libethercat.so").exists() {
        panic!("Couldn't find libethercat.so")
    }

    let include_dir = match env::var("ETHERCAT_INCLUDE_DIR") {
        Ok(dir) => PathBuf::from(dir),
        Err(_) => PathBuf::from("/usr/local/include")
    }.canonicalize().expect("");

    let include_header = include_dir.join("ecrt.h");
    if !include_header.exists() {
        panic!("Couldn't find ecrt.h")
    }

    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=dylib=ethercat");

    bindgen::Builder::default()
        .header(include_header.to_string_lossy()) 
        .size_t_is_usize(true)
        .allowlist_function(r"^ecrt_.*")
        .allowlist_type(r"^ec_.*")
        .allowlist_var(r"^ec_.*")
        .allowlist_var(r"^EC_.*") 
        .derive_default(true)
        .prepend_enum_name(false)
        .rustified_enum(".*")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("./src/binding.rs")
        .expect("Couldn't write binding");
}