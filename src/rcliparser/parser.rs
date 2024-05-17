use std::collections::VecDeque;

use crate::rcliterminal::terminal_singlenton::Terminal;

use super::input_reader::accept_input;
use super::invoker;
use super::lexical_analyzer::analyze;
use super::objects::token_objects::{GetValue, InvocationPair, Token, TokenObject};


pub fn create_stream(mut input_tokens: VecDeque<Token>, terminal_instance: &mut Terminal) -> VecDeque<Token>{
    let grammar = terminal_instance.get_instance_grammar();

    let mut output_tokens = VecDeque::<Token>::new();

    //pop first item, should be command
    let command_token = input_tokens.pop_front().unwrap();
    //check if exists 
    let core_command = grammar.get_command(command_token.get_value());

    //if core command is none exit
    if core_command.is_none(){
        eprintln!("INVALID COMMAND");
        return VecDeque::<Token>::new()
    }

    //core command
    output_tokens.push_front(Token::InvocationToken(core_command.clone().unwrap()));

    
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
                        if core_command.clone().unwrap().get_flags().contains(&flag_exists.unwrap().0){
                            //flag accepts object so pop next as well as a pair
                            if grammar.flag_accepts_obj(flag_exists.unwrap().0){
                                match input_tokens.pop_front().unwrap() {
                                    Token::TokenObject(obj) => {
                                        let pair = InvocationPair::new(flag_exists.unwrap().0.clone(), obj);
                                        output_tokens.push_back(Token::InvocationPair(pair));
                                    },
                                    _ => unreachable!()
                                };
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
            break 'parse;
        }
    }

    return output_tokens
}

pub fn parse(user_input: String, terminal_instance: &mut Terminal){
    let mut input = accept_input(user_input);
    let mut input_tokens = analyze(&mut input, terminal_instance);

    let mut output_tokens = create_stream(input_tokens, terminal_instance);
    
    //Core command that is executed
    let core_command = match output_tokens.pop_front().unwrap() {
        Token::InvocationToken(core) => core,
        _ => unreachable!()
    };
    //core object that the command invokes. If it doesn't exist we assign the cwd
    let core_object = match output_tokens.front().unwrap() {
        Token::TokenObject(_) => output_tokens.pop_front().unwrap(),
        _ => Token::TokenObject(TokenObject::OBJECT(terminal_instance.get_current_directory_to_string()))
    };

    //Can be either a solo flag, or a pair of flag and target object
    let flag_pairs = VecDeque::from(output_tokens);

    
    invoker::invoke(core_command, core_object, flag_pairs, terminal_instance);
}