use std::collections::VecDeque;
use regex::Regex;

use crate::rcliterminal::terminal_singlenton::Terminal;

use super::objects::grammar_objects::{Grammar, BnfType};
use super::objects::token_objects::{Token, TokenCommand::COMMAND, TokenObject::OBJECT, TokenFlag::FLAG};
use super::objects::user_input::{UserInput, Consumable};


//Analyze returns a tokenqueue
pub fn analyze(input: &mut UserInput, terminal_instance: &Terminal) -> VecDeque<Token>{
    let grammar: Grammar = terminal_instance.get_instance_grammar();
    
    let bnf_grammar = grammar.get_bnf_grammar();

    let mut tokens: Vec<Token> = Vec::new();
    //validates if token stream is correct by checking against the `next` filed in Command struct
    let mut token_validator = bnf_grammar.get(&BnfType::CORE).unwrap().next;

    
    //STEP 1: Valid core command
    let core_token = input.consume();
    if core_token.is_some(){

        tokens.push(Token::TokenCommand(COMMAND(core_token.unwrap())));
        
        //STEP 2: valid object. Match  ./Desktop/Files/readme.txt or ./Desktop/Files
        let object_matcher = Regex::new(r"^[^-]([.]*[/]|[..])*(\w+?\S+)?").unwrap();
        let flag_match = Regex::new(r"([-]+\w+)").unwrap();

        let mut next_command = input.consume();
        let mut command_string = next_command.clone().unwrap_or({
            terminal_instance.get_current_directory_to_string()
        });

        
        loop{
            let object_found = object_matcher.captures(&command_string.as_str());
            
            //if object found
            if object_found.is_some(){
                tokens.push(Token::TokenObject(OBJECT(command_string.clone())));

                if token_validator.contains(&BnfType::OBJECT){
                    token_validator.clear();
                    token_validator = bnf_grammar.get(&BnfType::OBJECT).unwrap().next;
                }
                else {
                    todo!("throw error, incorrect format");
                }
            }

            //STEP 3: valid flag(s)
            let flag_found = flag_match.captures(&command_string);
            
            if flag_found.is_some(){
                tokens.push(Token::TokenFlag(FLAG(command_string.clone())));

                if token_validator.contains(&BnfType::FLAG){
                    token_validator.clear();
                    token_validator = bnf_grammar.get(&BnfType::FLAG).unwrap().next;
                }
                else {
                    todo!("throw error, incorrect format");
                }
            }
            //if input analyzed break
            if input.analyzed{
                break;
            }
            
            //if next command is None break
            next_command = input.consume();
            command_string = match next_command.clone() {
                Some(obj) => {
                    obj
                }
                None => {
                    break;
                }
            };
        };
    }
    //STEP 1.1: validate soft command_grammar. Newline, CTRL^C etc
    else{
        tokens.push(Token::TokenCommand(COMMAND("invalid".to_string())));
    }

    return VecDeque::from(tokens);
}