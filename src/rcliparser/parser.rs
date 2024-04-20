use std::collections::VecDeque;
use std::vec;

use super::invoker::invoke;

use super::input_reader::accept_input;

use super::lexical_analyzer::analyze;
use super::lexical_analyzer::TokenFlag;
use super::lexical_analyzer::Tokens;
use super::lexical_analyzer::TokenObjects;
use super::lexical_analyzer::TokenCommands;

pub fn parse(user_input: String){
    let mut input = accept_input(user_input.as_str());
    let tokens = analyze(&mut input);
    
    for token in tokens{
        match token{
            Tokens::TokenCommands(_) => {
                
            },
            Tokens::TokenObjects(_) => {

            },
            Tokens::TokenFlag(_) => {

            }
        }
        invoke(token);
    }
}


#[cfg(test)]
mod test{
    use super::*;
}