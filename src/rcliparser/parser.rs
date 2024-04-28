use std::ops::DerefMut;

use crate::rcliterminal::terminal_singlenton::Terminal;

use super::invoker;

use super::input_reader::accept_input;

use super::utils::bsftree;

use super::lexical_analyzer::analyze;
use super::lexical_analyzer::Tokens;
use super::lexical_analyzer::TokenCommands;
use super::lexical_analyzer::TokenObjects;
use super::lexical_analyzer::TokenFlag;


pub fn match_parse(user_input: String, terminal_instance: &mut Terminal){
    let mut input = accept_input(user_input.as_str());
    let mut tokens = analyze(&mut input, terminal_instance);

    //pop command or insert invalid
    let command = tokens.pop_front().unwrap_or(Tokens::TokenCommands(TokenCommands::INVALID));

    //pop object or current directory
    //let current_dir_string = terminal_instance.get_current_directory_to_string();
    //let path: Result<TokenObjects, _> = tokens.pop_front().unwrap_or(Tokens::TokenObjects(TokenObjects::DIRECTORY(current_dir_string))).try_into();

    //pop tokens as follows
    // 'parser: loop{
    //     //if token stream isnt empty pop a token.
    //     //a token is either a terminal token. 
    //     //or a non terminal -d followed by an object
    //     if !tokens.is_empty(){
    //         let flag: Result<TokenFlag, _> = tokens.pop_front().unwrap().try_into();
    //     }
    //     break 'parser;
    // }
    //after the loop is done, call the invoker with the following parameters
    //invoker::invoke( core_command, object_command, flag_vector)
    //where flag vector contains flags that are either FLAG object that is terminal. I.e. --hidden
    //or an struct that is flag item and object item. I.E. -d ./localpath/to/something
    
    invoker::invoke(command.try_into().unwrap(), tokens, terminal_instance);
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