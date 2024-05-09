use core::str;
use std::fs;
use std::collections::HashMap;

use crate::rcliparser::objects::bnf_commands::{BnfGrammar, Command, CommandType, InvocationCommandSyntax};

const BNF_GRAMMAR_PATH: &str = "src\\rcliparser\\utils\\bnf_grammar.json";
const COMMAND_SYNTAX_PATH: &str = "src\\rcliparser\\utils\\command_syntax.json";


pub fn load_command_syntax() -> InvocationCommandSyntax{
    let data = fs::read_to_string(COMMAND_SYNTAX_PATH).unwrap();
    let json = serde_json::from_str::<InvocationCommandSyntax>(&data).unwrap();

    return json
}


pub fn load_grammar() -> HashMap<CommandType, Command>{
    let data = fs::read_to_string(BNF_GRAMMAR_PATH).unwrap();
    let json = serde_json::from_str::<BnfGrammar>(&data).unwrap();

    return json.get_hashmap().clone();
}