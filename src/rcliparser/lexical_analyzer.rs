use std::collections::HashMap;
use regex::Regex;

use super::input_reader::Consumable;
use super::input_reader::UserInput;

use super::utils::grammar_reader;
use super::utils::grammar_reader::Command;


#[derive(PartialEq, Debug, Clone, Eq)]
pub enum TokenCommands{
    CREATE,
    DELETE,
    COPY,
    MOVE,
    READ,
    LIST
}

#[derive(PartialEq, Debug, Clone, Eq)]
pub enum TokenObjects{
    FILE,
    DIRECTORY
}


#[derive(PartialEq, Debug, Clone, Eq)]
pub enum FlagType{
    TERMINAL,
    NONTERMINAL
}


#[derive(PartialEq, Debug, Clone, Eq)]
pub enum TokenFlag{
    FLAG(FlagType, String),
    FlagType(FlagType)
}


#[derive(PartialEq, Debug, Clone, Eq)]
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
        let core_token: Option<TokenCommands> = validate_command(&input.consume().unwrap_or("?".to_string()));
        
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

        let mut next_command = input.consume();
        let mut command_string = next_command.clone().unwrap_or("?".to_string());

        let file_found = file_matcher.captures(&command_string.as_str());
        let dir_found = directory_match.captures(&command_string.as_str());
        
        //if file found
        if file_found.is_some(){
            tokens.push(Tokens::TokenObjects(TokenObjects::FILE));
        }
        //if dir found
        if dir_found.is_some(){
            tokens.push(Tokens::TokenObjects(TokenObjects::DIRECTORY));
            
        }

        //STEP 3: valid flag(s)
        //match for terminal non terminal flags
        loop{
            let flag_found = flag_match.captures(&command_string);
            
            if flag_found.is_some(){
                let flag_object = validate_flag(flag_found.unwrap().get(0).unwrap().as_str());
                //if terminal flag stop loops
                if flag_object.clone().unwrap().eq(&TokenFlag::FlagType(FlagType::TERMINAL)){
                    tokens.push(Tokens::TokenFlag(TokenFlag::FLAG(FlagType::TERMINAL, next_command.unwrap())));
                    break;
                }
                //else push nonterminal flag and push the flag value
                tokens.push(Tokens::TokenFlag(TokenFlag::FLAG(FlagType::NONTERMINAL, next_command.unwrap())));
            }
            if input.analyzed{
                break;
            }
            next_command = input.consume();
            command_string = next_command.clone().unwrap_or("?".to_string());
        };
    }
    return tokens;
}


fn validate_command(command: &str) -> Option<TokenCommands>{
    match command {
        "create" => {
            return Some(TokenCommands::CREATE)
        },
        "delete" => {
            return Some(TokenCommands::DELETE)
        },
        "copy" => {
            return Some(TokenCommands::COPY)
        },
        "move" => {
            return Some(TokenCommands::MOVE)
        },
        "read" => {
            return Some(TokenCommands::READ)
        },
        "list" => {
            return Some(TokenCommands::LIST)
        },
        _ => {
            return None;
        }
    }
}


fn validate_flag(flag: &str) -> Option<TokenFlag>{
    match flag{
        flag if flag.starts_with("--") => {
            return Some(TokenFlag::FlagType(FlagType::TERMINAL));
        },
        flag if flag.starts_with("-") => {
            return Some(TokenFlag::FlagType(FlagType::NONTERMINAL));
        }
        _ => {
            return None;
        }
    }
}

#[cfg(test)]
mod lexical_tests {
    use super::*;
    use crate::rcliparser::input_reader::accept_input;

    #[test]
    fn validate_create() {
        println!("Testing input <create readme.txt>.");
        let mut input: UserInput = accept_input("create readme.txt");
        let tokens: Vec<Tokens> = vec![Tokens::TokenCommands(TokenCommands::CREATE), Tokens::TokenObjects(TokenObjects::FILE)];
        assert_eq!(analyze(&mut input), tokens);
    }

    #[test]
    fn validate_create_dir(){
        println!("Testing input <create ./Desktop/Some/Dir>.");
        let mut input2 = accept_input("create ./Desktop/Some/Dir");
        let tokens2: Vec<Tokens> = vec![Tokens::TokenCommands(TokenCommands::CREATE), Tokens::TokenObjects(TokenObjects::DIRECTORY)];
        assert_eq!(analyze(&mut input2), tokens2);
    }
        
    #[test]
    fn validate_list_dir(){
        println!("Testing input <list ./Desktop/Some/Dir --hidden>.");
        let mut input4 = accept_input("list ./Desktop/Some/Dir --hidden");
        let tokens4: Vec<Tokens> = vec![Tokens::TokenCommands(TokenCommands::LIST), Tokens::TokenObjects(TokenObjects::DIRECTORY), Tokens::TokenFlag(TokenFlag::FLAG(FlagType::TERMINAL, "--hidden".to_string()))];
        assert_eq!(analyze(&mut input4), tokens4);
    }

    #[test]
    fn validate_flag_hidden(){
        println!("Testing input <list --hidden>.");
        let mut input3 = accept_input("list --hidden");
        let tokens3: Vec<Tokens> = vec![Tokens::TokenCommands(TokenCommands::LIST), Tokens::TokenFlag(TokenFlag::FLAG(FlagType::TERMINAL, "--hidden".to_string()))];
        assert_eq!(analyze(&mut input3), tokens3);
    }
}