use std::collections::HashMap;
use regex::Regex;

#[path="../utils/grammar_reader.rs"]
mod grammar_reader;

use grammar_reader::Command;
use grammar_reader::CommandType;

#[path="../inputreader/input_reader.rs"]
mod input_reader;
use crate::input_reader::UserInput;
use crate::input_reader::Peekable;
use crate::input_reader::Consumable;


#[derive(PartialEq)]
enum TOKEN_COMMANDS{
    CREATE,
    DELETE,
    INVALID
}


pub fn analyze(input: &mut UserInput){
    let commands : HashMap<String, Command> = grammar_reader::load_grammar();
    let core: Vec<String> = commands.get("core").unwrap().command.clone();

    let mut tokens: Vec<TOKEN_COMMANDS> = Vec::new();

    //STEP 1: Valid core command
    if core.contains(&input.core_command){
        let core_token: Option<TOKEN_COMMANDS> = validate_command(&input.core_command);

        if core_token.is_some(){
            tokens.push(core_token.unwrap());
        }
        else {
            todo!("throw error")
        }
        //STEP 2: valid object. Match readme.txt with regex. Match ../Desktop/Files with regex?
        let re = Regex::new(r"[/]\w*[.].*").unwrap();
        let Some(val) = re.captures(&input.peek_next().ok());
        
    }

}

fn validate_command(command: &String) -> Option<TOKEN_COMMANDS>{
    match command.as_str() {
        "create" => {
            return Some(TOKEN_COMMANDS::CREATE)
        },
        "delete" =>{
            return Some(TOKEN_COMMANDS::DELETE)
        }
        _ => {
            return None;
        }
    }
}