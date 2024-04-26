use std::ops::DerefMut;

use crate::rcliterminal::terminal_singlenton::Terminal;

use super::invoker;

use super::input_reader::accept_input;
use super::utils::bsftree;

use super::lexical_analyzer::analyze;
use super::lexical_analyzer::Tokens;
use super::lexical_analyzer::TokenCommands;


pub fn match_parse(user_input: String, teminal_instance: &mut Terminal){
    let mut input = accept_input(user_input.as_str());
    let mut tokens = analyze(&mut input, teminal_instance);

    //pop command or insert invalid
    let command = tokens.pop_front().unwrap_or(Tokens::TokenCommands(TokenCommands::INVALID));

    //token order has already be checked (lexical analyzer) so no need to re check
    match command{
        Tokens::TokenCommands(core) => {
            //invoke the core command with the parameters
            invoker::invoke(core, tokens, teminal_instance);
        },
        _ => todo!("throw error")
    }
}


//Needs testing, is more refined than matching.. I think
pub fn tree_parse(user_input: String, teminal_instance: &mut Terminal) -> Result<bsftree::Tree<Tokens>, String>{
    let mut input = accept_input(user_input.as_str());
    let mut tokens = analyze(&mut input, teminal_instance);
    
    //pop command or insert invalid
    let command = tokens.pop_front().unwrap_or(Tokens::TokenCommands(TokenCommands::INVALID));

    //if valid command
    if !command.eq(&Tokens::TokenCommands(TokenCommands::INVALID)){
        let mut command_tree = bsftree::Tree::new(command);

        let mut last_tree_looped: Option<bsftree::Tree<Tokens>> = None;

        loop {
            //while token stream isnt empty
            match tokens.is_empty(){
                true => {
                    break;
                },
                false => {
                    //get next token and create a tree
                    let next_token = tokens.pop_front().unwrap();
                    let mut next_tree = bsftree::Tree::new(next_token);
                    
                    //if this is the first iteration
                    if last_tree_looped.as_ref().is_none(){
                        command_tree.add_subtree(next_tree.clone());
                        next_tree.add_parent(command_tree.clone());
                    }
                    //else this isnt
                    else{
                        let mut last_tree_unwrapede = last_tree_looped.unwrap();
                        last_tree_unwrapede.add_subtree(next_tree.clone());
                        next_tree.add_parent(last_tree_unwrapede);
                    }
                    last_tree_looped = Some(next_tree);
                }
            }
        }
        return Ok(command_tree);
    }
    return Err("Invalid".to_string());
}


#[cfg(test)]
mod test{
}