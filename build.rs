extern crate cc;

extern crate bindgen;

use std::env;
use std::path::PathBuf;


fn main() {
    // Build jsmn library, with optional compiler directives
    let mut build = cc::Build::new();
    let build = build.file("src/jsmn/jsmn.c");
    let build = build.include("src/jsmn");

    #[cfg(feature = "parent-links")]
    let mut build = build.define(parent_links, None);

    #[cfg(feature = "strict")]
    let mut build = build.define(strict, None);

    build.compile("jsmn");


    // Generate bindings for jsmn
    let bindings = bindgen::Builder::default()
        .header("src/jsmn/jsmn.h")
        .whitelisted_type("jsmntype_t")
        .whitelisted_type("jsmnerr")
        .whitelisted_type("jsmntok_t")
        .whitelisted_type("jsmn_parser")
        .whitelisted_function("jsmn_init")
        .whitelisted_function("jsmn_parse")
        .generate()
        .expect("Unable to generate bindings!");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings.write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write bindings.rs!");

    // Only regenerate if jsmn changed
    println!("cargo:rerun-if-changed=src/jsmn");
}
