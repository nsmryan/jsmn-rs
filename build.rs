extern crate cc;

extern crate bindgen;

use std::env;
use std::path::PathBuf;


fn main() {
    cc::Build::new()
        .file("src/jsmn/jsmn.c")
        .include("src/jsmn")
        .compile("jsmn");

    let bindings = bindgen::Builder::default()
        .header("src/jsmn/jsmn.h")
        .generate()
        .expect("Unable to generate bindings!");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings.write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write bindings.rs!");
}
