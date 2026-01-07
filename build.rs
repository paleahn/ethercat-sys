use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-env-changed=ETHERCAT_LIB_DIR");
    println!("cargo:rerun-if-env-changed=ETHERCAT_INCLUDE_DIR");

    let include_header = match pkg_config::Config::new().probe("ethercat") {
        Ok(lib) => lib.include_paths.get(0).expect("No include path found").clone(),
        Err(_) => {
            let lib_dir = PathBuf::from(env::var("ETHERCAT_LIB_DIR").expect("Env var ETHERCAT_LIB_DIR not set"));
            if !lib_dir.join("libethercat.so").exists() {
                panic!("Couldn't find libethercat.so")
            }

            let include_dir = PathBuf::from(env::var("ETHERCAT_INCLUDE_DIR").expect("Env var ETHERCAT_INCLUDE_DIR not set"));
            if !include_dir.join("ecrt.h").exists() {
                panic!("Couldn't find ecrt.h")
            }

            println!("cargo:rustc-link-search=native={}", lib_dir.display());
            println!("cargo:rustc-link-lib=dylib=ethercat");

            include_dir
        }
    };

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