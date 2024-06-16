use std::collections::{HashMap, VecDeque};
use std::io::Error;
use std::vec;

use crate::rcliterminal::terminal::Terminal;

use super::input_reader::accept_input;
use super::invoker;
use super::lexical_analyzer::analyze;
use super::objects::data_types::Data;
use super::objects::grammar_objects::{FlagType, PipeliningType};
use super::objects::token_objects::{GetValue, InvocationCommand, InvocationFlag, InvocationObject, InvocationPair, InvocationPipe, Invocator, Token};

/// Function that creates a token stream
pub fn create_stream(mut input_tokens: VecDeque<Token>, terminal_instance: &mut Terminal) -> Result<VecDeque<Token>, Error>{
    let grammar = terminal_instance.get_instance_grammar();

    let mut output_tokens = VecDeque::<Token>::new();
    let mut command: Option<InvocationCommand> = None;
    
    'parse: loop{
        //If tokens are consumed break, else consume.
        if !input_tokens.is_empty(){

            match input_tokens.pop_front().unwrap(){
                Token::TokenCommand(com) => {
                    let token_command = grammar.get_command(com.get_value());

                    if token_command.is_none(){
                        return Err(Error::new(std::io::ErrorKind::InvalidInput, "Parser error: Invalid command."));
                    }
                    //if a core command is found, change it with the current core_command
                    //for flag checks
                    output_tokens.push_back(Token::InvocationCommand(token_command.clone().unwrap()));
                    command = token_command;
                }
                //if an object is found then simply push it to stream
                Token::TokenObject(obj) => {
                    let invocation_obj = InvocationObject::new(obj.get_value().to_string());
                    output_tokens.push_back(Token::InvocationObject(invocation_obj));
                },
                //if a flag is found we have to check a few things
                Token::TokenFlag(flag) => {
                    let flag_exists = grammar.get_flag(flag.get_value());

                    //flag as a string exists in some FlagType
                    if flag_exists.is_some(){
                        
                        if command.is_none(){
                            return Err(Error::new(std::io::ErrorKind::InvalidInput, "Parser error: Invalid command."));
                        }
                        //core command accepts this flag at current iteration
                        let accepts_flag = command.as_ref().unwrap().get_flags().contains(&flag_exists.unwrap().0);

                        if accepts_flag{
                            //flag accepts object so pop next as well as a pair
                            if grammar.flag_accepts_obj(flag_exists.unwrap().0){
                                match input_tokens.pop_front().unwrap() {
                                    Token::TokenObject(obj) => {
                                        let invocation_obj = InvocationObject::new(obj.get_value().to_string());
                                        let pair = InvocationPair::new(flag_exists.unwrap().0.clone(), invocation_obj);
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
                            return Err(Error::new(std::io::ErrorKind::InvalidInput, "Parser error: Invalid flag for given command."));
                        }
                    }
                    //Invalid flag
                    else{
                        return Err(Error::new(std::io::ErrorKind::InvalidInput, "Parser error: Invalid flag."));
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
                        return Err(Error::new(std::io::ErrorKind::InvalidInput, "Parser error: Invalid piping for none command."));
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



pub fn call_invoker(mut input_tokens: VecDeque<Token>, terminal_instance: &mut Terminal) -> Result<Data, Error>{
    
    let core_command = match input_tokens.pop_front().unwrap() {
        Token::InvocationCommand(core) => core,
        //If core command wasn't first we would have exited aready
        _ => unreachable!()
    };
    

    let output_data: Data;
    let mut flags = HashMap::<FlagType, Option<InvocationObject>>::new();

    let mut data_vector: Vec<Data> = vec![];
    
    'extract_tokens: loop{
        let next_item = input_tokens.front();
        
        if next_item.is_some(){
            match input_tokens.pop_front().unwrap(){
                //Object found
                Token::InvocationObject(object) => {
                    let token_value = object.get_object();
    
                    data_vector.push(Data::SimpleData(token_value));
                },
                //Sole flag found
                Token::InvocationFlag(sole) => {
                    flags.insert(sole.get_type(), None);
                },
                //Flag object pair found
                Token::InvocationPair(pair) => {
                    flags.insert(pair.get_type(), Some(pair.get_object()));
                },
                //Pipe found
                Token::InvocationPipe(pipe) => {
                    //if vec is empty it means no objects found so add cwd
                    if data_vector.is_empty(){
                        let path_data = terminal_instance.get_current_directory().display().to_string();
                        data_vector.push(Data::SimpleData(path_data));
                    }
                    
                    output_data = Data::DataVector(Box::new(VecDeque::from(data_vector.clone())));
                    
                    //call invoker with the first half of the command
                    let invocation_token = Invocator::new(core_command.get_type(), output_data, flags.clone());
                    let invocation_result = invoker::invoke(invocation_token, terminal_instance);

                    if invocation_result.is_err(){
                        return invocation_result
                    }
                    
                    match pipe.get_type(){
                        //append invoker return type to end of tokens and call parser again
                        PipeliningType::PIPE => {
                            match invocation_result.unwrap(){
                                Data::SimpleData(data) | Data::StringData(data)=> {
                                    input_tokens.push_back(Token::InvocationObject(InvocationObject::new(data)));
                                },
                                Data::PathData(path) => {
                                    input_tokens.push_back(Token::InvocationObject(InvocationObject::new(path.display().to_string())));
                                },
                                Data::VecStringData(string_vec) => {
                                    for string in string_vec{
                                        let token_object = Token::InvocationObject(InvocationObject::new(string));
                                        input_tokens.push_back(token_object);
                                    }
                                },
                                Data::DirPathData(pathbuf_vec) => {
                                    for path in pathbuf_vec{
                                        let token_object = Token::InvocationObject(InvocationObject::new(path.display().to_string()));
                                        input_tokens.push_back(token_object);
                                    }
                                }
                                _ => unreachable!()
                            }
                            
                            return call_invoker(input_tokens, terminal_instance)
                        },
                        //if redirect found then object is next
                        PipeliningType::REDIRECT => {
                            todo!("redirect to file")
                        },
                    }
                },
                _ => return Err(Error::new(std::io::ErrorKind::InvalidInput, "Parser error: Invalid piping for none command."))
            }
        }
        else{
            break 'extract_tokens;
        }

    }  
    
    if data_vector.is_empty(){
        let path_data = terminal_instance.get_current_directory().display().to_string();
        data_vector.push(Data::SimpleData(path_data));
    }

    output_data = Data::DataVector(Box::new(VecDeque::from(data_vector)));

    let invocation_token = Invocator::new(core_command.get_type(), output_data, flags);
    let invocation_result = invoker::invoke(invocation_token, terminal_instance);
    
    return invocation_result
}


///Main parser functions. Checks if all the stages until now are correct and calls call_invoker function.
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
    
    return call_invoker(parser_output.unwrap(), terminal_instance)
}