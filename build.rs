extern crate cc;

extern crate bindgen;

use std::env;
use std::path::PathBuf;


fn main() {

    let parent_links : &str;
    let strict : &str;

    if cfg!(feature = "parent-links") {
        parent_links = "JSMN_PARENT_LINKS";
    }
    else {
        parent_links = "";
    }

    if cfg!(feature = "strict") {
        strict = "JSMN_STRICT";
    }
    else {
        strict = "";
    }

    cc::Build::new()
        .file("src/jsmn/jsmn.c")
        .include("src/jsmn")
        .define(parent_links, None)
        .define(strict, None)
        .compile("jsmn");

    let bindings = bindgen::Builder::default()
        .header("src/jsmn/jsmn.h")
        .generate()
        .expect("Unable to generate bindings!");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings.write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write bindings.rs!");
    println!("cargo:rerun-if-changed=src/jsmn");
}
