use core::str;
use std::fs;

use crate::rcliparser::objects::grammar_objects::Grammar;

const GRAMMAR: &str = "src\\rcliparser\\utils\\grammar.json";


pub fn load_grammar() -> Grammar{
    let data = fs::read_to_string(GRAMMAR).unwrap();
    let json = serde_json::from_str::<Grammar>(&data).unwrap();

    return json;
}