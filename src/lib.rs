//!
//! This crate exposes Rustic bindings to a gem of a C library called jsmn.
//! The raw bindings are available in the raw module, which are wrapped in
//! this module.
//! 
//!
//! The jsmn library provides a very simple, fast JSON parser. I see it as a
//! great example of library design- it has two enums, two structs, and two functions,
//! and that is all. Its trivial to use, very fast, and has very few extra features.
//!
//!
//! This library only exposes a single function, jsmn_parse, because the jsmn_init
//! function is called when you crate a new JsmnParser.
//! 
//! 
//! To use this library, simply create a parser using JsmnParser::new()
//! and pass the parser, a JSON string, and a slice of JsmnToks to jsmn_parse.
//! The result will be that the slice will be filled out with tokens defining the
//! starting and ending offset of each JSON token in the given string.
//!
//!
//! Thats all there is to it! This crate is just intended to make jsmn easy to use
//! in Rust. There are other JSON parsers in Rust, and certainly Serde and HyperJson
//! are great crates, but I though jsmn deserved a place in the Rust ecosystem, so here
//! it is!
//!

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::mem::transmute;

pub mod raw;


/// The JSON object type. These enum values are identical to the jsmn library
/// enum jsmntype_t, but renamed to match Rust's conventions.
#[repr(i32)]
#[derive(Debug, Copy, Clone)]
pub enum JsmnType {
    JsmnUndefined = raw::jsmntype_t::JSMN_UNDEFINED as i32,
    JsmnObject    = raw::jsmntype_t::JSMN_OBJECT as i32,
    JsmnArray     = raw::jsmntype_t::JSMN_ARRAY as i32,
    JsmnString    = raw::jsmntype_t::JSMN_STRING as i32,
    JsmnPrimitive = raw::jsmntype_t::JSMN_PRIMITIVE as i32,
}

/// Error type from jsmn_parse. These enum values are identical to the jsmn library
/// enum jsmnerr_t, but renamed to match Rust's conventions.
#[repr(i32)]
#[derive(Debug, Copy, Clone)]
pub enum JsmnErr {
    JsmErrorNoMem = raw::jsmnerr::JSMN_ERROR_NOMEM as i32,
    JsmErrorInval = raw::jsmnerr::JSMN_ERROR_INVAL as i32,
    JsmErrorPart  = raw::jsmnerr::JSMN_ERROR_PART  as i32,
}

/// A JSON token structure, defining which type of JSON object it is, the starting
/// character, ending character, and size in bytes. All offsets are from the start
/// of the parsed string.
///
/// Note that if the parent-links feature is used, then this struct will
/// have the "parent" field, and otherwise it will not.
#[repr(C)]
#[derive(Debug, Copy)]
pub struct JsmnTok {
    pub typ    : JsmnType,
    pub start  : i32,
    pub end    : i32,
    pub size   : i32,
#[cfg(feature="parent-links")]
    pub parent : i32,
}

impl JsmnTok {
    pub fn new() -> Self {
        JsmnTok {
            typ    : JsmnType::JsmnUndefined,
            start  : 0,
            end    : 0,
            size   : 0,
#[cfg(feature="parent-links")]
            parent : 0
        }
    }
}

impl Clone for JsmnTok {
    fn clone(&self) -> Self { *self }
}

impl Default for JsmnTok {
    fn default() -> Self {
        JsmnTok { typ    : JsmnType::JsmnUndefined,
                  start  : 0,
                  end    : 0,
                  size   : 0,
#[cfg(feature="parent-links")]
                  parent : 0
                }
    }
}

/// A JsmnParser is the parser state for the jsmn library.
#[repr(C)]
#[derive(Debug, Copy)]
pub struct JsmnParser {
    pub pos      : usize,
    pub toknext  : usize,
    pub toksuper : usize,
}

impl JsmnParser {
    pub fn new() -> Self {
        let parser = 
            JsmnParser {
                pos      : 0,
                toknext  : 0,
                toksuper : 0
            };
        unsafe {
            raw::jsmn_init(transmute(&parser));
        }

        parser
    }
}

impl Clone for JsmnParser {
    fn clone(&self) -> Self { *self }
}

impl Default for JsmnParser {
    fn default() -> Self {
        JsmnParser { pos      : 0,
                     toknext  : 0,
                     toksuper : 0,
                   }
    }
}

/// This function is the core parsing function. It wraps the underlying
/// jsmn_parse function in a more Rustic interface by taking a slice
/// of JsmnTokens, and returning a Result instead of using sentinal values.
///
///
/// Simply provide a JsmnParser, a string, and a slice of JsmnTokens,
/// and the tokens will point to the locations of each JSON object within the
/// string.
///
/// If the function succeeds, it will return a usize giving how many
/// tokens were parsed, and on error it will return an JsmnErr describing the
/// problem encountered while parsing.
pub fn jsmn_parse(parser: &mut JsmnParser,
                  js: &str,
                  tokens: &mut [JsmnTok]) -> Result<usize, JsmnErr> {
    let result : i32;

    unsafe {
        result = raw::jsmn_parse(transmute(parser),
                                 transmute(js.as_ptr()),
                                 js.len() as usize,
                                 transmute(tokens.as_ptr()),
                                 tokens.len() as u32);
    }

    match result {
        -1    => Err(JsmnErr::JsmErrorNoMem),
        -2    => Err(JsmnErr::JsmErrorInval),
        -3    => Err(JsmnErr::JsmErrorPart),
        count => Ok(count as usize),
    }
}

#[cfg(test)]
mod test {
    use *;

    #[test]
    fn test_parse() {
        let mut parser = JsmnParser::new();
        let json = "{\"test\":1}";
        let mut tokens : [JsmnTok; 20] = [Default::default(); 20];

        let result = jsmn_parse(&mut parser, json, &mut tokens);

        println!("{:?}", &tokens[..]);
        println!("{:?}", result);
    }
}

