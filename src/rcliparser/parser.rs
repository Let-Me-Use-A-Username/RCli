use std::collections::VecDeque;

use crate::rcliterminal::terminal_singlenton::Terminal;

use super::invoker;

use super::input_reader::accept_input;

use super::objects::tokens::FlagObjectPair;
use super::objects::tokens::{TokenCommands, TokenFlag, TokenObjects, Tokens, FlagType::NONTERMINAL};
use super::utils::bsftree;

use super::lexical_analyzer::analyze;


pub fn match_parse(user_input: String, terminal_instance: &mut Terminal){
    let mut input = accept_input(user_input);
    let mut tokens = analyze(&mut input, terminal_instance);

    //pop command or insert invalid token
    let invalid_token: TokenCommands = terminal_instance.get_instance_syntax().get_token(&"invalid".to_string()).unwrap();
    let command: TokenCommands = match tokens.pop_front().unwrap() {
        Tokens::TokenCommand(core_command) => {
            core_command
        },
        _ => invalid_token
    };

    //failsafe in case directory obj isnt present but core requires it.
    let current_dir_string = terminal_instance.get_current_directory_to_string();
    //mainly used when commands do not require an additional parameter like list.
    let mut path: TokenObjects = TokenObjects::DIRECTORY(current_dir_string);
    //flag vector
    let mut flag_vector: VecDeque<FlagObjectPair> = VecDeque::new();

    'parser: loop{
        //while tokens stream isnt empty
        if !tokens.is_empty(){
            //pop next token
            match tokens.pop_front().unwrap() {
                //match object (this means that the syntax is core -> object)
                Tokens::TokenObjects(obj) => {
                    path = obj;
                },
                //match flag 
                Tokens::TokenFlag(flag) => {
                    match flag {
                        TokenFlag::FLAG(flag_type, _) => {
                            //if flag type found is non terminal then next item also belongs to the flag pair
                            if flag_type.eq(&NONTERMINAL){
                                //todo! this unwrap might cause problems
                                let obj = tokens.pop_front().unwrap_or(Tokens::TokenObjects(TokenObjects::INVALID));
                                let pair = FlagObjectPair::PAIR(flag, obj.try_into().unwrap());
                                flag_vector.push_back(pair);
                            }
                            //else this is a terminal flag which doesnt have a pair
                            else{
                                let sole = FlagObjectPair::SOLE(flag);
                                flag_vector.push_back(sole);
                                break 'parser;
                            }
                        },
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            };
        }
        else{
            break 'parser;
        }
    }
    invoker::invoke(command, path, flag_vector, terminal_instance);
}


// //Needs testing, is more refined than matching.. I think
// pub fn tree_parse(user_input: String, teminal_instance: &mut Terminal) -> Result<bsftree::Tree<Tokens>, String>{
//     let mut input = accept_input(user_input);
//     let mut tokens = analyze(&mut input, teminal_instance);
    
//     //pop command or insert invalid
//     let command = tokens.pop_front().unwrap_or(Tokens::TokenCommands(TokenCommands::INVALID));

//     //if valid command
//     if !command.eq(&Tokens::TokenCommands(TokenCommands::INVALID)){
//         let mut command_tree = bsftree::Tree::new(command);

//         let mut last_tree_looped: Option<bsftree::Tree<Tokens>> = None;

//         loop {
//             //while token stream isnt empty
//             match tokens.is_empty(){
//                 true => {
//                     break;
//                 },
//                 false => {
//                     //get next token and create a tree
//                     let next_token = tokens.pop_front().unwrap();
//                     let mut next_tree = bsftree::Tree::new(next_token);
                    
//                     //if this is the first iteration
//                     if last_tree_looped.as_ref().is_none(){
//                         command_tree.add_subtree(next_tree.clone());
//                         next_tree.add_parent(command_tree.clone());
//                     }
//                     //else this isnt
//                     else{
//                         let mut last_tree_unwrapede = last_tree_looped.unwrap();
//                         last_tree_unwrapede.add_subtree(next_tree.clone());
//                         next_tree.add_parent(last_tree_unwrapede);
//                     }
//                     last_tree_looped = Some(next_tree);
//                 }
//             }
//         }
//         return Ok(command_tree);
//     }
//     return Err("Invalid".to_string());
// }


// #[cfg(test)]
// mod test{
// }