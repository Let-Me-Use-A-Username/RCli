use std::vec;

use super::invoker::invoke;

use super::input_reader::accept_input;

use super::lexical_analyzer::analyze;
use super::lexical_analyzer::TokenFlag;
use super::lexical_analyzer::Tokens;
use super::lexical_analyzer::TokenObjects;
use super::lexical_analyzer::TokenCommands;

#[derive(PartialEq, Eq, Debug)]
pub struct InvokerCommand{
    invoke_command: TokenCommands,
    invoke_file: Vec<Option<String>>,
    invoke_dir: Vec<Option<String>>,
    invoke_flags: Vec<Option<String>>
}


pub fn parse(user_input: String){
    let mut input = accept_input(user_input.as_str());
    let tokens = analyze(&mut input);
    let invoke_command = match_token_command(tokens);
}

fn match_token_command(tokens: Vec<Tokens>) -> InvokerCommand{
    let mut command: TokenCommands = TokenCommands::INVALID;
    let mut file: Vec<Option<String>> = vec![None];
    let mut dir: Vec<Option<String>> = vec![None];
    let mut flags: Vec<Option<String>> = vec![None];

    for token_item in tokens{
        match token_item {
            Tokens::TokenCommands(value) => {
                command = value;
            },
            Tokens::TokenObjects(TokenObjects::FILE(value)) => {
                if file.get(0) == None{
                    file.remove(0);
                }
                file.push(Some(value));
            },
            Tokens::TokenObjects(TokenObjects::DIRECTORY(value)) => {
                if dir.get(0) == None{
                    dir.remove(0);
                }
                dir.remove(0);
                dir.push(Some(value));
            },
            Tokens::TokenFlag(TokenFlag::FLAG(_, value)) => {
                if flags.get(0) == None{
                    flags.remove(0);
                }
                flags.remove(0);
                flags.push(Some(value))
            },
            _ => {
                todo!("Throw error, token not found");
            }
        }
    }
    InvokerCommand {
        invoke_command: command,
        invoke_file: file,
        invoke_dir: dir,
        invoke_flags: flags
    }
}


#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_invoke_command_SimpleCreate(){
        let mut input = accept_input("create readme.txt");
        let tokens = analyze(&mut input);
        let invoke_command = match_token_command(tokens);
        let command = InvokerCommand{ 
            invoke_command: TokenCommands::CREATE,
            invoke_file: vec![Some("readme.txt".to_string())],
            invoke_dir: vec![None],
            invoke_flags: vec![None]
        };
        assert_eq!(invoke_command, command);
    }

    #[test]
    fn test_invoke_command_ListHidden(){
        let mut input = accept_input("list ./Desktop/Some/Dir --hidden");
        let tokens = analyze(&mut input);
        let invoke_command = match_token_command(tokens);
        let command = InvokerCommand{ 
            invoke_command: TokenCommands::LIST,
            invoke_file: vec![None],
            invoke_dir: vec![Some("./Desktop/Some/Dir".to_string())],
            invoke_flags: vec![Some("--hidden".to_string())]
        };
        assert_eq!(invoke_command, command);
    }

    #[test]
    fn test_invoke_command_CopyFileToPath(){
        let mut input = accept_input("copy readme.txt -d ./Desktop/Pathto/file");
        let tokens = analyze(&mut input);
        let invoke_command = match_token_command(tokens);
        let command = InvokerCommand{ 
            invoke_command: TokenCommands::COPY,
            invoke_file: vec![Some("readme.txt".to_string())],
            invoke_dir: vec![Some("./Desktop/Pathto/file".to_string())],
            invoke_flags: vec![Some("-d".to_string())]
        };
        assert_eq!(invoke_command, command);
    }
}