use std::collections::{HashMap, VecDeque};
use std::io::Error;
use std::path::PathBuf;

use crate::rcliterminal::terminal_singlenton::Terminal;

use super::input_reader::accept_input;
use super::invoker;
use super::lexical_analyzer::analyze;
use super::objects::data_types::Data;
use super::objects::grammar_objects::FlagType;
use super::objects::token_objects::{GetValue, InvocationFlag, InvocationPair, InvocationPipe, Token, TokenObject};

/// Function that creates a token stream
pub fn create_stream(mut input_tokens: VecDeque<Token>, terminal_instance: &mut Terminal) -> Result<VecDeque<Token>, Error>{
    let grammar = terminal_instance.get_instance_grammar();

    let mut output_tokens = VecDeque::<Token>::new();

    //pop first item, should be command
    let command_token = input_tokens.pop_front().unwrap();
    //check if exists 
    let mut core_command = grammar.get_command(command_token.get_value());

    //if core command is none exit
    if core_command.is_none(){
        return Err(Error::new(std::io::ErrorKind::InvalidInput, "Parser error: Invalid command."));
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
                        return Err(Error::new(std::io::ErrorKind::InvalidInput, "Parser error: Invalid command."));
                    }
                    //if a core command is found, change it with the current core_command
                    //for flag checks
                    output_tokens.push_back(Token::InvocationToken(command.clone().unwrap()));
                    core_command = command;
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


/// Function that recursively parses input stream to call the invoker.
/// Recursion is needed mainly for piping or redirecting operations
pub fn call_invoker(mut output_tokens: VecDeque<Token>, terminal_instance: &mut Terminal)  -> Result<Data, Error>{
    //Core command that is executed
    let core_command = match output_tokens.pop_front().unwrap() {
        Token::InvocationToken(core) => core,
        //If core command wasn't first we would have exited aready
        _ => unreachable!()
    };
    

    //If core object doesn;t exist we assign CWD.
    //The item isnt popped only checked with front()
    let data = match output_tokens.front() {
        Some(Token::TokenObject(_)) => {
            //safe convertion since all functions accept a path
            let data_token = output_tokens.pop_front().unwrap();
            let token_value = data_token.get_value();
            Data::PathData(PathBuf::from(token_value))
        },
        _ => {
            let path_data = terminal_instance.get_current_directory_to_string();
            Data::PathData(PathBuf::from(path_data))
        }
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
            Token::InvocationPipe(_) => {
                //if pipe is found it means there are more tokens next
                //call invoker to get result
                let res = invoker::invoke(core_command.clone(), data.clone(), flags.clone(), terminal_instance);

                //if result is valid
                if res.is_ok(){
                    let data_returned = res.unwrap();
                    
                    //if no tokens are after the pipe the syntax is wrong
                    if output_tokens.len() < 1{
                        return Err(Error::new(std::io::ErrorKind::UnexpectedEof, "Parser error: Cannot pipe with empty right hand arguments."))
                    }
                    
                    //match returned data type, add it to the token vector and invoke
                    match data_returned{
                        //append to token stream of next invoker call
                        Data::PathData(path) => {
                            match output_tokens.get(1) {
                                //if object exists at index 1, append it to object
                                Some(Token::TokenObject(obj)) => {
                                    let new_object = path.join(obj.get_value());
                                    output_tokens.remove(1);
                                    output_tokens.insert(1, Token::TokenObject(TokenObject::OBJECT(new_object.display().to_string())));
                                },
                                //else create the object
                                _ => {
                                    output_tokens.insert(1, Token::TokenObject(TokenObject::OBJECT(path.display().to_string())));
                                }
                            }
                            let recursive_result = call_invoker(output_tokens.clone(), terminal_instance);

                            return recursive_result;
                        },
                        Data::StringData(string_data) => {
                            let new_object = Token::TokenObject(TokenObject::OBJECT(string_data));
                            output_tokens.insert(1, new_object);
                        },
                        Data::VecStringData(_) => todo!(),
                        Data::DirPathData(_) => todo!(),
                        _ => todo!("no")
                    }
                }
                //if result not valid return error
                else{
                    return res;
                }
            }
            //todo! add redirection pipe later
            //This is not required since the syntax is checked at the lexer
            _ => break
        }
    }
    
    return invoker::invoke(core_command, data, flags, terminal_instance);
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