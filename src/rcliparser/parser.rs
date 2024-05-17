use std::collections::VecDeque;

use crate::rcliterminal::terminal_singlenton::Terminal;

use super::input_reader::accept_input;
use super::lexical_analyzer::analyze;
use super::objects::token_objects::{GetValue, Token};


pub fn match_parse(user_input: String, terminal_instance: &mut Terminal){
    let grammar = terminal_instance.get_instance_grammar();
    let commands = grammar.get_invocation_commands();
    let flags = grammar.get_flag_types();

    let mut input = accept_input(user_input);
    let mut input_tokens = analyze(&mut input, terminal_instance);
    let mut output_tokens = VecDeque::<Token>::new();

    //pop first item, should be command
    let command_token = input_tokens.pop_front().unwrap();
    //check if exists 
    let core_command = grammar.get_command(command_token.get_value());

    //if core command is none exit
    if core_command.is_none(){
        eprintln!("INVALID COMMAND");
        return
    }

    //got core command. Will be interpreted in invoker
    output_tokens.push_front(Token::InvocationToken(core_command.unwrap()));

    // mkdir /path/to/dir -r
    // copy afile.txt -d a/path
    
    'parse: loop{
        if !input_tokens.is_empty(){

            match input_tokens.pop_front().unwrap(){
                //if an object is found then simply push it to stream
                Token::TokenObject(obj) => {
                    output_tokens.push_back(Token::TokenObject(obj));
                },
                //if a flag is found we have to check a few things
                Token::TokenFlag(flag) => {
                    let flag_exists = grammar.get_flag(flag.get_value());
                    
                    //flag (in general) is valid
                    if flag_exists.is_some(){
                        //core command accepts this flag
                        if core_command.unwrap().get_flags().contains(&flag_exists.unwrap().0){
                            //flag accepts object so pop next as well as a pair
                            if grammar.flag_accepts_obj(flag_exists.unwrap().0){

                            }
                            //flag doesn't accept object so pop as a sole flag
                            else{
                                output_tokens.push_back(Token::TokenFlag(flag))
                            }
                        }
                        else{
                            eprintln!("INVALID FLAG FOR GIVEN COMMAND")
                        }
                    }
                    //Invalid flag
                    else{
                        todo!()
                    }
                },
                _ => unreachable!()
            }
        }
        else{
            break 'parse
        }
    }
}

// pub fn match_parse(user_input: String, terminal_instance: &mut Terminal){
//     let mut input = accept_input(user_input);
//     let mut input_tokens = analyze(&mut input, terminal_instance);
//     let mut output_tokens = VecDeque::<Token>::new();

//     let grammar = terminal_instance.get_instance_grammar();

//     //pop command or insert invalid token
//     let command: TokenCommand = input_tokens.pop_front().unwrap().try_into().unwrap();

//     output_tokens.push_front(command);

//     //failsafe in case directory obj isnt present but core requires it.
//     let current_dir_string = terminal_instance.get_current_directory_to_string();
//     //mainly used when commands do not require an additional parameter like list.
//     let mut path: TokenObject = TokenObject::OBJECT(current_dir_string);
//     //flag vector
//     let mut flag_vector: VecDeque<FlagObjectPair> = VecDeque::new();

//     'parser: loop{
//         //while tokens stream isnt empty
//         if !tokens.is_empty(){
//             //pop next token
//             match tokens.pop_front().unwrap() {
//                 //match object (this means that the syntax is core -> object)
//                 Token::TokenObject(obj) => {
//                     path = obj;
//                 },
//                 //match flag 
//                 Token::TokenFlag(flag) => {
//                     match flag {
//                         TokenFlag::FLAG(flag_type, _) => {
//                             //if flag type found is non terminal then next item also belongs to the flag pair
//                             if flag_type.eq(&NONTERMINAL){
//                                 //todo! this unwrap might cause problems
//                                 let obj = tokens.pop_front().unwrap_or(Tokens::TokenObjects(TokenObjects::INVALID));
//                                 let pair = FlagObjectPair::PAIR(flag, obj.try_into().unwrap());
//                                 flag_vector.push_back(pair);
//                             }
//                             //else this is a terminal flag which doesnt have a pair
//                             else{
//                                 let sole = FlagObjectPair::SOLE(flag);
//                                 flag_vector.push_back(sole);
//                                 break 'parser;
//                             }
//                         },
//                         _ => unreachable!(),
//                     }
//                 }
//                 _ => unreachable!(),
//             };
//         }
//         else{
//             break 'parser;
//         }
//     }
//     invoker::invoke(command, path, flag_vector, terminal_instance);
// }