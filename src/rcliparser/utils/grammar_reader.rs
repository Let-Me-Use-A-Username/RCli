use core::str;
use std::fs;
use std::collections::HashMap;

use crate::rcliparser::objects::bnf_commands::{BnfGrammar, Command, CommandType, InvocationCommand, InvocationCommandSyntax};

const BNF_GRAMMAR_PATH: &str = "src\\rcliparser\\utils\\bnf_grammar.json";
const COMMAND_SYNTAX_PATH: &str = "src\\rcliparser\\utils\\command_syntax.json";


pub fn load_command_syntax() -> InvocationCommandSyntax{
    let data = fs::read_to_string(COMMAND_SYNTAX_PATH).unwrap();
    let json = serde_json::from_str::<InvocationCommandSyntax>(&data).unwrap();

    let mut syntax: Vec<InvocationCommand> = Vec::new();

    return json
}


pub fn load_grammar() -> HashMap<CommandType, Command>{
    let data = fs::read_to_string(BNF_GRAMMAR_PATH).unwrap();
    let json = serde_json::from_str::<BnfGrammar>(&data).unwrap();

    let mut grammar: HashMap<CommandType, Command> = HashMap::new();

    for (key, value) in json.get_hashmap(){
        let command_type = match key.as_str() {
            "core" => {
                CommandType::Core
            },
            "sub" => {
                CommandType::Sub
            },
            "object" => {
                CommandType::Object
            },
            "flag" => {
                CommandType::Flag
            },
            _ => {
                CommandType::INVALID
            }
        };
        grammar.insert(command_type, value.to_owned());
    }

    return grammar;
    
}