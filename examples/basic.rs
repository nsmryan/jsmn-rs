extern crate jsmn_rs;

use jsmn_rs::*;


const JSON : &str =
  "{\"test\":1}";

fn main() {
    let mut parser = JsmnParser::new();

    let mut tokens = [JsmnTok::new(); 20];
    
    let count = jsmn_parse(&mut parser, JSON, &mut tokens).unwrap();

    assert!(count == 3);

    println!("{:?}", tokens);
}

