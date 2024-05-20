use std::collections::VecDeque;
use std::io::Error;
use regex::Regex;

use crate::rcliterminal::terminal_singlenton::Terminal;

use super::objects::grammar_objects::{Grammar, BnfType};
use super::objects::token_objects::{Token, TokenCommand::COMMAND, TokenObject::OBJECT, TokenFlag::FLAG};
use super::objects::user_input::{UserInput, Consumable};


//Analyze returns a tokenqueue
pub fn analyze(input: &mut UserInput, terminal_instance: &Terminal) -> Result<VecDeque<Token>, Error>{
    let grammar: Grammar = terminal_instance.get_instance_grammar();
    let mut tokens: Vec<Token> = Vec::new();

    //STEP 2: valid object. Match  ./Desktop/Files/readme.txt or ./Desktop/Files
    let object_matcher = Regex::new(r"^[^-]([.]*[/]|[..])*(\w+?\S+)?").unwrap();
    let flag_match = Regex::new(r"([-]+\w+)").unwrap();

    let mut next_command : Option<String>;
    let mut command_string : String;

    loop{
        
        //if input analyzed break
        if input.analyzed{
            break;
        }

        //if next command is None break
        next_command = input.consume();
        command_string = match next_command {
            Some(obj) => {
                obj
            }
            None => {
                break;
            }
        };

        //for available command invocations (Strings)
        let command_name = grammar.match_string_to_command(&command_string);
        
        //STEP 1: if name matches, add command
        if command_name.is_some(){
            tokens.push(Token::TokenCommand(COMMAND(command_string.clone())));
            continue;
        }

        let object_found = object_matcher.captures(&command_string.as_str());
        
        //if object found
        if object_found.is_some(){
            tokens.push(Token::TokenObject(OBJECT(command_string.clone())));

            if !grammar.accepts_next(&BnfType::CORE, &BnfType::OBJECT){
                return Err(Error::new(std::io::ErrorKind::InvalidInput, "Incorrect format."));
            }
            continue;
        }

        //STEP 3: valid flag(s)
        let flag_found = flag_match.captures(&command_string.as_str());

        if flag_found.is_some(){
            tokens.push(Token::TokenFlag(FLAG(command_string.clone())));

            if !grammar.accepts_next(&BnfType::OBJECT, &BnfType::FLAG){
                return Err(Error::new(std::io::ErrorKind::InvalidInput, "Incorrect format."));
            }
            continue;
        }
    }

    return Ok(VecDeque::from(tokens));
}