#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::mem::transmute;

mod raw;


#[repr(i32)]
#[derive(Debug, Copy, Clone)]
pub enum JsmnType {
    JsmnUndefined = raw::jsmntype_t::JSMN_UNDEFINED as i32,
    JsmnObject    = raw::jsmntype_t::JSMN_OBJECT as i32,
    JsmnArray     = raw::jsmntype_t::JSMN_ARRAY as i32,
    JsmnString    = raw::jsmntype_t::JSMN_STRING as i32,
    JsmnPrimitive = raw::jsmntype_t::JSMN_PRIMITIVE as i32,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone)]
pub enum JsmnErr {
    JsmErrorNoMem = raw::jsmnerr::JSMN_ERROR_NOMEM as i32,
    JsmErrorInval = raw::jsmnerr::JSMN_ERROR_INVAL as i32,
    JsmErrorPart  = raw::jsmnerr::JSMN_ERROR_PART  as i32,
}

#[repr(C)]
#[derive(Debug, Copy)]
pub struct JsmnTok {
    pub typ   : JsmnType,
    pub start : i32,
    pub end   : i32,
    pub size  : i32,
}

impl JsmnTok {
    pub fn new(typ   : JsmnType,
               start : i32,
               end   : i32,
               size  : i32) -> Self {
        JsmnTok {
            typ   : typ,
            start : start,
            end   : end,
            size  : size,
        }
    }
}

impl Clone for JsmnTok {
    fn clone(&self) -> Self { *self }
}

impl Default for JsmnTok {
    fn default() -> Self {
        JsmnTok { typ   : JsmnType::JsmnUndefined,
                  start : 0,
                  end   : 0,
                  size  : 0,
                }
    }
}

#[repr(C)]
#[derive(Debug, Copy)]
pub struct JsmnParser {
    pub pos      : usize,
    pub toknext  : usize,
    pub toksuper : usize,
}

impl JsmnParser {
    pub fn new(pos      : usize,
               toknext  : usize,
               toksuper : usize) -> Self {
        JsmnParser {
            pos      : pos,
            toknext  : toknext,
            toksuper : toksuper
        }
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

pub fn jsmn_init(parser: &mut JsmnParser) {
    unsafe {
        raw::jsmn_init(transmute(parser));
    }
}

pub fn jsmn_parse(parser: &mut JsmnParser,
                  js: &str,
                  tokens: &mut [JsmnTok]) -> Result<usize, JsmnErr> {
    let mut result :i32 = 0;

    println!("json len = {}", js.len());
    println!("tokens len = {}", tokens.len());
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
        let mut parser : JsmnParser = Default::default();
        let json = "{\"test\":1}";
        let mut tokens : [JsmnTok; 20] = [Default::default(); 20];

        jsmn_init(&mut parser);
        let result = jsmn_parse(&mut parser, json, &mut tokens);

        println!("{:?}", &tokens[..]);
        println!("{:?}", result);
    }
}
