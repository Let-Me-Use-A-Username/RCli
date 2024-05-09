use std::collections::HashMap;
use std::collections::VecDeque;
use regex::Regex;

use crate::rcliterminal::terminal_singlenton::Terminal;

use super::objects::bnf_commands::InvocationCommandSyntax;
use super::objects::tokens::{FlagType, TokenCommands, TokenFlag, TokenObjects, Tokens};

use super::objects::user_input::{Consumable, UserInput};
use super::objects::bnf_commands::{Command, CommandType};


//Analyze returns a tokenqueue
pub fn analyze(input: &mut UserInput, terminal_instance: &Terminal) -> VecDeque<Tokens>{
    let command_grammar: HashMap<CommandType, Command> = terminal_instance.get_instance_grammar();
    let command_syntax: InvocationCommandSyntax = terminal_instance.get_instance_syntax();

    let mut tokens: Vec<Tokens> = Vec::new();
    //validates if token stream is correct by checking against the `next` filed in Command struct
    let mut token_validator: Vec<CommandType> = (&command_grammar.get(&CommandType::CORE).unwrap().next).clone();

    
    //STEP 1: Valid core command
    let command = command_syntax.get_token(&input.core_command);
    if command.is_some(){

        let core_token: TokenCommands = command.unwrap();
        
        input.consume();
        tokens.push(Tokens::TokenCommand(core_token));
        
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
                tokens.push(Tokens::TokenObjects(TokenObjects::OBJECT(command_string.clone())));

                if token_validator.contains(&CommandType::OBJECT){
                    token_validator.clear();
                    token_validator = (&command_grammar.get(&CommandType::OBJECT).unwrap().next).clone();
                }
                else {
                    todo!("throw error, incorrect format");
                }
            }

            //STEP 3: valid flag(s)
            //match for terminal non terminal flags
            let flag_found = flag_match.captures(&command_string);
            
            if flag_found.is_some(){
                let flag_object = validate_flag(flag_found.unwrap().get(0).unwrap().as_str());
                //if terminal flag stop loops
                if flag_object.clone().unwrap().eq(&TokenFlag::FlagType(FlagType::TERMINAL)){
                    tokens.push(Tokens::TokenFlag(TokenFlag::FLAG(FlagType::TERMINAL, next_command.unwrap())));
                    break;
                }
                //else push nonterminal flag and push the flag value
                if token_validator.contains(&CommandType::FLAG){
                    tokens.push(Tokens::TokenFlag(TokenFlag::FLAG(FlagType::NONTERMINAL, next_command.unwrap())));
                    token_validator.clear();
                    token_validator = (&command_grammar.get(&CommandType::FLAG).unwrap().next).clone();
                }
                else{
                    todo!("throw error, incorrect format");
                }
            }
            //or if input analyzed break
            if input.analyzed{
                break;
            }
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
        println!("Input:[ {:?} ]", input.core_command);
        todo!("Parse commands like newline, CTRL^C etc");
    }

    
    return VecDeque::from(tokens);
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
    use crate::rcliparser::utils::grammar_reader;
    use crate::rcliterminal::terminal_singlenton;

    // #[test]
    // fn start_test(){
    //     //load grammar
    //     let grammar = grammar_reader::load_grammar();
    //     let syntax = grammar_reader::load_command_syntax();
    //     //load singlenton
    //     let instance: &mut Terminal = terminal_singlenton::singlenton(grammar, syntax);

    //     validate_touch(&instance);
    //     validate_mkdir(&instance);
    //     validate_list_dir(&instance);
    //     validate_flag_hidden(&instance);
    //     validate_flag_tuple(&instance);
    //     validate_token_chain_core_after_object(&instance);
    //     validate_token_chain_double_core_command(&instance);
    // }

    // fn validate_touch(terminal_instance: &Terminal) {
    //     println!("Testing input <touch readme.txt>.");
    //     let mut input: UserInput = accept_input("touch readme.txt".to_string());
    //     let tokens: Vec<Tokens> = vec![Tokens::TokenCommands(TokenCommands::TOUCH), Tokens::TokenObjects(TokenObjects::FILE("readme.txt".to_string()))];
    //     assert_eq!(analyze(&mut input, terminal_instance), tokens);
    // }

    // fn validate_mkdir(terminal_instance: &Terminal){
    //     println!("Testing input <mkdir ./Desktop/Some/Dir>.");
    //     let mut input2 = accept_input("mkdir ./Desktop/Some/Dir".to_string());
    //     let tokens2: Vec<Tokens> = vec![Tokens::TokenCommands(TokenCommands::MKDIR), Tokens::TokenObjects(TokenObjects::DIRECTORY("./Desktop/Some/Dir".to_string()))];
    //     assert_eq!(analyze(&mut input2, terminal_instance), tokens2);
    // }
        
    // fn validate_list_dir(terminal_instance: &Terminal){
    //     println!("Testing input <list ./Desktop/Some/Dir --hidden>.");
    //     let mut input4 = accept_input("list ./Desktop/Some/Dir --hidden".to_string());
    //     let tokens4: Vec<Tokens> = vec![Tokens::TokenCommands(TokenCommands::LIST), Tokens::TokenObjects(TokenObjects::DIRECTORY("./Desktop/Some/Dir".to_string())), Tokens::TokenFlag(TokenFlag::FLAG(FlagType::TERMINAL, "--hidden".to_string()))];
    //     assert_eq!(analyze(&mut input4, terminal_instance), tokens4);
    // }

    // fn validate_flag_hidden(terminal_instance: &Terminal){
    //     println!("Testing input <list --hidden>.");
    //     let mut input3 = accept_input("list --hidden".to_string());
    //     let tokens3: Vec<Tokens> = vec![Tokens::TokenCommands(TokenCommands::LIST), Tokens::TokenFlag(TokenFlag::FLAG(FlagType::TERMINAL, "--hidden".to_string()))];
    //     assert_eq!(analyze(&mut input3, terminal_instance), tokens3);
    // }

    // fn validate_flag_tuple(terminal_instance: &Terminal){
    //     println!("Testing input <copy readme.txt -d ./Desktop/Pathto/file >.");
    //     let mut input3 = accept_input("copy readme.txt -d ./Desktop/Pathto/file ".to_string());
    //     let tokens3: Vec<Tokens> = vec![Tokens::TokenCommands(TokenCommands::COPY), Tokens::TokenObjects(TokenObjects::FILE("readme.txt".to_string())), Tokens::TokenFlag(TokenFlag::FLAG(FlagType::NONTERMINAL, "-d".to_string())), Tokens::TokenObjects(TokenObjects::DIRECTORY("./Desktop/Pathto/file".to_string()))];
    //     assert_eq!(analyze(&mut input3, terminal_instance), tokens3);
    // }

    // fn validate_token_chain_double_core_command(terminal_instance: &Terminal){
    //     println!("Testing input <copy copy>.");
    //     let mut input3 = accept_input("copy copy".to_string());
    //     let tokens3: Vec<Tokens> = vec![Tokens::TokenCommands(TokenCommands::COPY)];
    //     assert_eq!(analyze(&mut input3, terminal_instance), tokens3);
    // }

    // fn validate_token_chain_core_after_object(terminal_instance: &Terminal){
    //     println!("Testing input <copy readme.txt copy>.");
    //     let mut input3 = accept_input("copy readme.txt copy".to_string());
    //     let tokens3: Vec<Tokens> = vec![Tokens::TokenCommands(TokenCommands::COPY), Tokens::TokenObjects(TokenObjects::FILE("readme.txt".to_string()))];
    //     assert_eq!(analyze(&mut input3, terminal_instance), tokens3);
    // }
}