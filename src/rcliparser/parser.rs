use std::collections::{HashMap, VecDeque};
use std::io::Error;

use crate::rcliterminal::terminal_singlenton::Terminal;

use super::input_reader::accept_input;
use super::invoker;
use super::lexical_analyzer::analyze;
use super::objects::data_types::Data;
use super::objects::grammar_objects::FlagType;
use super::objects::token_objects::{GetValue, InvocationFlag, InvocationPair, InvocationPipe, Token, TokenObject};


pub fn create_stream(mut input_tokens: VecDeque<Token>, terminal_instance: &mut Terminal) -> Result<VecDeque<Token>, Error>{
    let grammar = terminal_instance.get_instance_grammar();

    let mut output_tokens = VecDeque::<Token>::new();

    //pop first item, should be command
    let command_token = input_tokens.pop_front().unwrap();
    //check if exists 
    let core_command = grammar.get_command(command_token.get_value());

    //if core command is none exit
    if core_command.is_none(){
        return Err(Error::new(std::io::ErrorKind::InvalidInput, "Invalid command."));
    }

    //core command
    output_tokens.push_front(Token::InvocationToken(core_command.clone().unwrap()));

    
    'parse: loop{
        //If tokens are consumed break, else consume.
        if !input_tokens.is_empty(){

            match input_tokens.pop_front().unwrap(){
                Token::TokenCommand(com) => {
                    let command = grammar.get_command(com.get_value());

                    if command.is_none(){
                        return Err(Error::new(std::io::ErrorKind::InvalidInput, "Invalid command."));
                    }

                    output_tokens.push_back(Token::InvocationToken(command.clone().unwrap()));
                }
                //if an object is found then simply push it to stream
                Token::TokenObject(obj) => {
                    output_tokens.push_back(Token::TokenObject(obj));
                },
                //if a flag is found we have to check a few things
                Token::TokenFlag(flag) => {
                    let flag_exists = grammar.get_flag(flag.get_value());
                    
                    //flag as a string exists in some FlagType
                    if flag_exists.is_some(){
                        //core command accepts this flag at current iteration
                        let accepts_flag = core_command.clone().unwrap().get_flags().contains(&flag_exists.unwrap().0);

                        if accepts_flag{
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
                                let sole: InvocationFlag = InvocationFlag::new(flag_exists.unwrap().0.clone());
                                output_tokens.push_back(Token::InvocationFlag(sole))
                            }
                        }
                        else{
                            return Err(Error::new(std::io::ErrorKind::InvalidInput, "Invalid flag for given command."));
                        }
                    }
                    //Invalid flag
                    else{
                        return Err(Error::new(std::io::ErrorKind::InvalidInput, "Invalid flag."));
                    }
                },
                Token::TokenPipe(pipe) => {
                    let last_token = output_tokens.back();
                    //if last token is some
                    if !last_token.is_none(){
                        //and last token is invocation token
                        let invocation_type = grammar.get_pipe(pipe.get_value());
                        if invocation_type.is_some(){
                            let pipe = InvocationPipe::new(invocation_type.unwrap());
                            output_tokens.push_back(Token::InvocationPipe(pipe));
                        }
                    }
                    else{
                        return Err(Error::new(std::io::ErrorKind::InvalidInput, "Invalid piping for none command."));
                    }
                }
                _ => unreachable!()
            }
        }
        else{
            break 'parse;
        }
    }

    return Ok(output_tokens)
}

pub fn parse(user_input: String, terminal_instance: &mut Terminal) -> Result<Data, Error>{
    let user_input = accept_input(user_input);
    if user_input.is_err(){
        return Err(user_input.err().unwrap());
    }

    let input_tokens = analyze(&mut user_input.ok().unwrap(), terminal_instance);
    if input_tokens.is_err(){
        return Err(input_tokens.err().unwrap());
    } 
    
    let parser_output = create_stream(input_tokens.ok().unwrap(), terminal_instance);
    if parser_output.is_err(){
        return Err(parser_output.err().unwrap());
    }
    
    let mut output_tokens = parser_output.unwrap();


    //Core command that is executed
    let core_command = match output_tokens.pop_front().unwrap() {
        Token::InvocationToken(core) => core,
        //If core command wasn't first we would exited aready
        _ => unreachable!()
    };
    

    //If core object doesn;t exist we assign CWD.
    //The item isnt popped only checked with front()
    let core_object = match output_tokens.front() {
        Some(Token::TokenObject(_)) => output_tokens.pop_front().unwrap(),
        _ => Token::TokenObject(TokenObject::OBJECT(terminal_instance.get_current_directory_to_string()))
    };

    //Flag extraction
    let mut flags: HashMap<FlagType, Option<TokenObject>> = HashMap::new();

    loop{
        if output_tokens.is_empty(){
            break
        }

        match output_tokens.pop_front().unwrap() {
            Token::InvocationFlag(sole) => {
                flags.insert(sole.get_type(), None);
            },
            Token::InvocationPair(pair) => {
                flags.insert(pair.get_type(), Some(pair.get_object()));
            }
            _ => break
        }
    }
    
    return invoker::invoke(core_command, core_object, flags, terminal_instance);
}