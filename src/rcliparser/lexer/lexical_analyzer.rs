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
    DELETE
}

#[derive(PartialEq, Debug)]
pub enum TokenObjects{
    FILE,
    DIRECTORY
}


#[derive(PartialEq, Debug)]
pub enum TokenFlag{
    TERMINAL,
    NONTERMINAL
}


#[derive(PartialEq, Debug)]
pub enum Tokens{
    TokenCommands(TokenCommands),
    TokenObjects(TokenObjects),
    TokenFlag(TokenFlag)
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
        let flag_match = Regex::new(r"([-]+\w+)").unwrap();

        let mut next_command = input.peek(1).unwrap();

        let file_found = file_matcher.captures(&next_command.as_str());
        let dir_found = directory_match.captures(&next_command.as_str());
        
        //if file found. Check for flags
        if file_found.is_some(){
            let file_object = validate_object(file_found.unwrap().get(0).unwrap().as_str());
            tokens.push(Tokens::TokenObjects(file_object.unwrap()));
            //if file found increment input
            next_command = input.peek_next().unwrap();
        }
        else if dir_found.is_some(){
            //file not found. But found dir. Check flags
            let dir_object = validate_object(dir_found.unwrap().get(0).unwrap().as_str());
            tokens.push(Tokens::TokenObjects(dir_object.unwrap()));
            //if dir found increment input
            next_command = input.peek_next().unwrap();
        }
        //STEP 3: valid flag(s)
        //If no file found and no dir found then depending on the command the dir is the current working directory
        //see if flag is terminal (--hidden) or not (-p)
        while !next_command.eq("?"){
            let flag_found = flag_match.captures(&next_command.as_str());

            if flag_found.is_some(){
                let flag_object = validate_flag(flag_found.unwrap().get(0).unwrap().as_str());
                //if terminal flag stop loop
                if flag_object.unwrap().eq(TokenFlag::TERMINAL){
                    break;
                }
                //else push nonterminal flag and push the flag value
                tokens.push(Tokens::TokenFlag(flag_object.unwrap()));
                next_command = input.peek_next().unwrap();
            }
        }
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


fn validate_flag(flag: &str) -> Option<TokenFlag>{
    match flag{
        "terminal" if flag.starts_with("--") => {
            return Some(TokenFlag::TERMINAL);
        },
        "nonterminal" if flag.starts_with("-") => {
            return Some(TokenFlag::NONTERMINAL);
        }
        _ => {
            return None;
        }
    }
}