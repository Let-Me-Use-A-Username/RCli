use std::collections::VecDeque;

use super::invoker::invoke;

use super::input_reader::accept_input;
use super::utils::bsftree;

use super::lexical_analyzer::analyze;
use super::lexical_analyzer::TokenFlag;
use super::lexical_analyzer::Tokens;
use super::lexical_analyzer::TokenObjects;
use super::lexical_analyzer::TokenCommands;

pub fn parse(user_input: String){
    let mut input = accept_input(user_input.as_str());
    let mut tokens = analyze(&mut input);
    
    //pop command or insert invalid
    let command = tokens.pop_front().unwrap_or(Tokens::TokenCommands(TokenCommands::INVALID));

    //if not invalid command
    if command.eq(&Tokens::TokenCommands(TokenCommands::INVALID)){
        let mut command_tree = bsftree::Tree::new(command);

        let mut last_tree_looped: Option<bsftree::Tree<T>> = None;

        loop {
            if !tokens.is_empty(){
                let next_token = tokens.pop_front().unwrap();
                let mut next_tree = bsftree::Tree::new(next_token);
                
                //if none this is the first loop
                if last_tree_looped.is_none(){
                    command_tree.add_subtree(next_tree.clone());
                    next_tree.add_parent(command_tree);
                }
            }
        }
    }
    

}


#[cfg(test)]
mod test{
    use super::*;
}