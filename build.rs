extern crate cc;

extern crate bindgen;

use std::env;
use std::path::PathBuf;


fn main() {

    let parent_links : &str;
    let strict : &str;

    println!("parent-links {}", cfg!(feature = "parent-links"));
    println!("strict {}", cfg!(feature = "strict"));

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

    let mut build = cc::Build::new();
    let mut build = build.file("src/jsmn/jsmn.c");
    let mut build = build.include("src/jsmn");

    #[cfg(feature = "parent-links")]
    let mut build = build.define(parent_links, None);

    #[cfg(feature = "strict")]
    let mut build = build.define(strict, None);

    build.compile("jsmn");

    let bindings = bindgen::Builder::default()
        .header("src/jsmn/jsmn.h")
        .generate()
        .expect("Unable to generate bindings!");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings.write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write bindings.rs!");
    //println!("cargo:rerun-if-changed=src/jsmn");
}
