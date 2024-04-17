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
    let parameters = Vec::from_iter(tokens[1..].iter().cloned());
    
    match tokens.get(0).unwrap() {
        Tokens::TokenCommands(TokenCommands::CREATE) => {
            rcli_create(parameters);
        },
        Tokens::TokenCommands(TokenCommands::DELETE) => {
            rcli_delete()
        },
        Tokens::TokenCommands(TokenCommands::COPY) => {
            rcli_copy()
        },
        Tokens::TokenCommands(TokenCommands::MOVE) => {
            rcli_move()
        },
        Tokens::TokenCommands(TokenCommands::READ) => {
            rcli_read()
        },
        Tokens::TokenCommands(TokenCommands::LIST) => {
            rcli_list()
        },
        _ => {

        }
    }
}

fn rcli_create(parameters: Vec<Tokens>){
    
}

fn rcli_delete(){

}

fn rcli_copy(){

}

fn rcli_move(){

}

fn rcli_read(){

}

fn rcli_list(){

}

#[cfg(test)]
mod test{
    use super::*;
}