use std::collections::HashMap;
use std::collections::VecDeque;
use regex::Regex;

use super::input_reader::Consumable;
use super::input_reader::UserInput;

use super::utils::grammar_reader;
use super::utils::grammar_reader::Command;
use super::utils::grammar_reader::CommandType;


#[derive(PartialEq, Debug, Clone, Eq)]
pub enum TokenCommands{
    CREATE,
    DELETE,
    COPY,
    MOVE,
    READ,
    LIST,
    CD,
    EXIT,
    INVALID
}

#[derive(PartialEq, Debug, Clone, Eq)]
pub enum TokenObjects{
    FILE(String),
    DIRECTORY(String)
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


#[derive(PartialEq, Debug, Clone, Eq,)]
pub enum Tokens{
    TokenCommands(TokenCommands),
    TokenObjects(TokenObjects),
    TokenFlag(TokenFlag)
}

//Trait to downcast tokens to tokenobject enum in order to extract string value
impl TryFrom<Tokens> for TokenObjects{
    type Error = &'static str;  

    fn try_from(value: Tokens) -> Result<Self, Self::Error> {
        match value{
            Tokens::TokenObjects(TokenObjects::DIRECTORY(dir)) => {
                Ok(TokenObjects::DIRECTORY(dir))
            },
            Tokens::TokenObjects(TokenObjects::FILE(file)) => {
                Ok(TokenObjects::FILE(file))
            },
            _ => {
                Err("Convertion failed")
            }
        }
    }
}

//Analyze returns a queue(FIFO)
pub fn analyze(input: &mut UserInput) -> VecDeque<Tokens>{
    let commands : HashMap<CommandType, Command> = grammar_reader::load_grammar();
    let core_commands: Vec<String> = commands.get(&CommandType::Core).unwrap().command.clone();

    let mut tokens: Vec<Tokens> = Vec::new();
    //validates if token stream is correct by checking against the `next` filed in Command struct
    let mut token_validator: Vec<CommandType> = Vec::new();

    //STEP 1: Valid core command
    if core_commands.contains(&input.core_command){
        let core_token: Option<TokenCommands> = validate_command(&input.consume().unwrap_or("?".to_string()));
        
        if core_token.is_some(){
            tokens.push(Tokens::TokenCommands(core_token.unwrap()));
            token_validator = (&commands.get(&CommandType::Core).unwrap().next.clone()).clone();
        }
        else {
            todo!("throw error, no core command provided")
        }
        //STEP 2: valid object. Match  ./Desktop/Files/readme.txt or ./Desktop/Files
        let file_matcher = Regex::new(r"[/]?\w+[.]{1}.*").unwrap();
        let directory_match = Regex::new(r"[/]\w+$").unwrap();
        let flag_match = Regex::new(r"([-]+\w+)").unwrap();

        let mut next_command = input.consume();
        let mut command_string = next_command.clone().unwrap_or("?".to_string());

        
        loop{
            let file_found = file_matcher.captures(&command_string.as_str());
            let dir_found = directory_match.captures(&command_string.as_str());
            
            //if file found
            if file_found.is_some(){
                tokens.push(Tokens::TokenObjects(TokenObjects::FILE(command_string.clone())));
            }
            //if dir found
            if dir_found.is_some(){
                tokens.push(Tokens::TokenObjects(TokenObjects::DIRECTORY(command_string.clone())));
            }
            
            //we re-check to see if the token format is correct
            if file_found.is_some() | dir_found.is_some(){
                if token_validator.contains(&CommandType::Object){
                    token_validator.clear();
                    token_validator = (&commands.get(&CommandType::Object).unwrap().next.clone()).clone();
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
                if token_validator.contains(&CommandType::Flag){
                    tokens.push(Tokens::TokenFlag(TokenFlag::FLAG(FlagType::NONTERMINAL, next_command.unwrap())));
                    token_validator.clear();
                    token_validator = (&commands.get(&CommandType::Flag).unwrap().next.clone()).clone();
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
            command_string = next_command.clone().unwrap_or("?".to_string());
        };
    }
    else{
        todo!("Throw error, unknown core command");
    }
    return VecDeque::from(tokens);
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
        "cd" => {
            return Some(TokenCommands::CD)
        },
        "exit" => {
            return Some(TokenCommands::EXIT)
        }
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
        let tokens: Vec<Tokens> = vec![Tokens::TokenCommands(TokenCommands::CREATE), Tokens::TokenObjects(TokenObjects::FILE("readme.txt".to_string()))];
        assert_eq!(analyze(&mut input), tokens);
    }

    #[test]
    fn validate_create_dir(){
        println!("Testing input <create ./Desktop/Some/Dir>.");
        let mut input2 = accept_input("create ./Desktop/Some/Dir");
        let tokens2: Vec<Tokens> = vec![Tokens::TokenCommands(TokenCommands::CREATE), Tokens::TokenObjects(TokenObjects::DIRECTORY("./Desktop/Some/Dir".to_string()))];
        assert_eq!(analyze(&mut input2), tokens2);
    }
        
    #[test]
    fn validate_list_dir(){
        println!("Testing input <list ./Desktop/Some/Dir --hidden>.");
        let mut input4 = accept_input("list ./Desktop/Some/Dir --hidden");
        let tokens4: Vec<Tokens> = vec![Tokens::TokenCommands(TokenCommands::LIST), Tokens::TokenObjects(TokenObjects::DIRECTORY("./Desktop/Some/Dir".to_string())), Tokens::TokenFlag(TokenFlag::FLAG(FlagType::TERMINAL, "--hidden".to_string()))];
        assert_eq!(analyze(&mut input4), tokens4);
    }

    #[test]
    fn validate_flag_hidden(){
        println!("Testing input <list --hidden>.");
        let mut input3 = accept_input("list --hidden");
        let tokens3: Vec<Tokens> = vec![Tokens::TokenCommands(TokenCommands::LIST), Tokens::TokenFlag(TokenFlag::FLAG(FlagType::TERMINAL, "--hidden".to_string()))];
        assert_eq!(analyze(&mut input3), tokens3);
    }

    #[test]
    fn validate_flag_tuple(){
        println!("Testing input <copy readme.txt -d ./Desktop/Pathto/file >.");
        let mut input3 = accept_input("copy readme.txt -d ./Desktop/Pathto/file ");
        let tokens3: Vec<Tokens> = vec![Tokens::TokenCommands(TokenCommands::COPY), Tokens::TokenObjects(TokenObjects::FILE("readme.txt".to_string())), Tokens::TokenFlag(TokenFlag::FLAG(FlagType::NONTERMINAL, "-d".to_string())), Tokens::TokenObjects(TokenObjects::DIRECTORY("./Desktop/Pathto/file".to_string()))];
        assert_eq!(analyze(&mut input3), tokens3);
    }

    #[test]
    #[should_panic]
    fn validate_token_chain_double_core_command(){
        println!("Testing input <copy copy>.");
        let mut input3 = accept_input("copy copy");
        let tokens3: Vec<Tokens> = vec![Tokens::TokenCommands(TokenCommands::COPY), Tokens::TokenCommands(TokenCommands::COPY)];
        assert_eq!(analyze(&mut input3), tokens3);
    }

    #[test]
    #[should_panic]
    fn validate_token_chain_core_after_object(){
        println!("Testing input <copy readme.txt copy>.");
        let mut input3 = accept_input("copy readme.txt copy");
        let tokens3: Vec<Tokens> = vec![Tokens::TokenCommands(TokenCommands::COPY), Tokens::TokenCommands(TokenCommands::COPY)];
        assert_eq!(analyze(&mut input3), tokens3);
    }

    #[test]
    #[should_panic]
    fn validate_token_chain_only_object(){
        println!("Testing input <readme.txt>.");
        let mut input3 = accept_input("readme.txt");
        let tokens3: Vec<Tokens> = vec![Tokens::TokenCommands(TokenCommands::COPY), Tokens::TokenCommands(TokenCommands::COPY)];
        assert_eq!(analyze(&mut input3), tokens3);
    }
}