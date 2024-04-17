#[warn(unused_imports)]
use super::input_reader::accept_input;

use super::lexical_analyzer::analyze;
use super::lexical_analyzer::Tokens;
use super::lexical_analyzer::TokenCommands;


pub fn parse(user_input: String){
    let mut input = accept_input(user_input.as_str());
    let tokens = analyze(&mut input);
    //create_tree(tokens);
}

fn match_token_command(tokens: Vec<Tokens>){
    for token in tokens{
        match token {
            Tokens::TokenCommands(TokenCommands::CREATE) => {
                
            },
            Tokens::TokenCommands(TokenCommands::DELETE) => {

            },
            Tokens::TokenCommands(TokenCommands::COPY) => {

            },
            Tokens::TokenCommands(TokenCommands::MOVE) => {

            },
            Tokens::TokenCommands(TokenCommands::READ) => {

            },
            Tokens::TokenCommands(TokenCommands::LIST) => {

            },
            _ => {

            }
        }
    }
}

#[cfg(test)]
mod test{
    use super::*;
}