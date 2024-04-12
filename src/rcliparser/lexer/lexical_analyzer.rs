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


#[derive(PartialEq, Debug)]
pub enum TokenCommands{
    CREATE,
    DELETE,
    INVALID
}

#[derive(PartialEq, Debug)]
pub enum TokenObjects{
    FILE,
    DIRECTORY,
    INVALID
}


#[derive(PartialEq, Debug)]
pub enum Tokens{
    TokenCommands(TokenCommands),
    TokenObjects(TokenObjects)
}


pub fn analyze(input: &mut UserInput) -> Vec<Tokens>{
    let commands : HashMap<String, Command> = grammar_reader::load_grammar();
    let core: Vec<String> = commands.get("core").unwrap().command.clone();

    let mut tokens: Vec<Tokens> = Vec::new();

    //STEP 1: Valid core command
    if core.contains(&input.core_command){
        let core_token: Option<TokenCommands> = validate_command(&input.core_command);

        if core_token.is_some(){
            tokens.push(Tokens::TokenCommands(core_token.unwrap()));
        }
        else {
            todo!("throw error")
        }
        //STEP 2: valid object. Match  ./Desktop/Files/readme.txt or ./Desktop/Files
        let file_matcher = Regex::new(r"[/]?\w+[.]{1}.*").unwrap();
        let directory_match = Regex::new(r"[/]\w+$").unwrap();

        let next_command = input.peek(1).unwrap();
        let file_found = file_matcher.captures(&next_command.as_str());
        let dir_found = directory_match.captures(&next_command.as_str());
        
        //if file found. Check for flags
        if file_found.is_some(){
            let file_object = validate_object(file_found.unwrap().get(0).unwrap().as_str());
            tokens.push(Tokens::TokenObjects(file_object.unwrap()));
        }
        else if dir_found.is_some(){
            //file not found. But found dir. Check flags
            let dir_object = validate_object(dir_found.unwrap().get(0).unwrap().as_str());
            tokens.push(Tokens::TokenObjects(dir_object.unwrap()));
        }
        //STEP 3: valid flag(s)
        //At this points, we either have a file, a dir or nothing. All are acceptable
        //Note to self. There are commands that do not require an object, so having CORE -> FLAG is valid.
        //Additionally CORE -> OBJECT -> FLAG is also valid, so we only need to check for valid flags
        todo!("check flags");
    }
    return tokens;

}

fn validate_command(command: &String) -> Option<TokenCommands>{
    match command.as_str() {
        "create" => {
            return Some(TokenCommands::CREATE)
        },
        "delete" => {
            return Some(TokenCommands::DELETE)
        }
        _ => {
            return None;
        }
    }
}

fn validate_object(object: &str) -> Option<TokenObjects>{
    if object.contains("."){
        return Some(TokenObjects::FILE)
    }
    else if object.contains("/"){
        return Some(TokenObjects::DIRECTORY)
    }
    else{
        return None;
    }
}

// #[cfg(test)]
// mod tests{
//     use super::*;
//     use crate::lexical_analyzer::UserInput;


//     #[test]
//     fn test_validation(){
//         let mut create_input = input_reader::accept_input("create readme.txt");
//         let mut create_input_with_dir = input_reader::accept_input("create ./path/to/file/readme.txt");
//         let mut create_dir = input_reader::accept_input("create ./path/to/dir");

//         let tokens = analyze(&mut create_input);
//         assert_eq!(, vec![Tokens::TokenCommands::CREATE, Tokens::TokenObjects::FILE])
//     }
// }