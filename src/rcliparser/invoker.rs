use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

use crate::rcliparser::objects::grammar_objects::CommandType;
use crate::rcliparser::objects::token_objects::GetValue;
use crate::rcliterminal::terminal_singlenton::Terminal;

use super::objects::data_types::Data;
use super::objects::grammar_objects::FlagType;
use super::objects::token_objects::{InvocationToken, TokenObject};
use super::utils::functions;


pub fn invoke(core: InvocationToken, data_object: Data, flags: HashMap<FlagType, Option<TokenObject>>, terminal_instance: &mut Terminal) -> Result<Data, Error>{

    //Prints for debug purposes
    // println!("\nCORE: {:?}", core.clone());
    // println!("DATA: {:?}", object.clone());
    // println!("FLAGS: {:?}", flags.clone());

    let operation_status: Result<Data, Error>;

    let object = match data_object{
        Data::DataVector(mut vector) => {
            if vector.len() == 1{
                Data::PathData(vector.pop().unwrap().get_path().unwrap().to_path_buf())
            }
            else{
                todo!()
            }
        },
        Data::PathData(path) => {
            Data::PathData(path.to_path_buf())
        }
        //Only types that might come are pathdata and datavector
        _ => unreachable!()
    };


    match core.get_type(){
        CommandType::HOME => {
            operation_status = home(terminal_instance);
        },
        CommandType::CWD => {
            operation_status = cwd(terminal_instance);
        },
        CommandType::TOUCH => {
            operation_status = touch(object);
        },
        CommandType::MKDIR => {
            let recursive = flags.get(&FlagType::RECURSIVE);

            if recursive.is_some(){
                operation_status = mkdir(object, true);
            }
            else{
                operation_status = mkdir(object, false);
            }
        },
        CommandType::REMOVE => {
            let recursive = flags.get(&FlagType::RECURSIVE);

            if recursive.is_some(){
                operation_status = remove(object, true);
            }
            else{
                operation_status = remove(object, false);
            }
        },
        CommandType::COPY => {
            let destination = flags.get(&FlagType::DESTINATION);

            if destination.is_some(){
                let destination = Data::PathData(PathBuf::from(destination.unwrap().as_ref().unwrap().get_value()));
                
                operation_status = copy(object, destination, terminal_instance);
            }
            else{
                operation_status = Err(Error::new(ErrorKind::InvalidInput, "Invoker Error: Didn't provide destination."));
            }
        },
        CommandType::MOVE => {
            let destination = flags.get(&FlagType::DESTINATION);

            if destination.is_some(){
                let destination = Data::PathData(PathBuf::from(destination.unwrap().as_ref().unwrap().get_value()));
                
                operation_status = r#move(object, destination, terminal_instance);
            }
            else{
                operation_status = Err(Error::new(ErrorKind::InvalidInput, "Invoker Error: Didn't provide destination."));
            }
        },
        CommandType::READ => {
            operation_status = read(object);
        },
        CommandType::LIST => {
            let hidden = flags.get(&FlagType::HIDDEN);

            if hidden.is_some(){
                operation_status = list(object, true);
            }
            else{
                operation_status = list(object, false);
            }
        },
        CommandType::CD => {
            let mut destination_path = PathBuf::from(terminal_instance.get_current_directory());
            destination_path.push(object.get_path().unwrap());

            let destination_data = Data::PathData(destination_path);

            operation_status = traverse_directory(destination_data, terminal_instance);
        },
        CommandType::GREP => {
            let pattern = flags.get(&FlagType::PATTERN);

            if pattern.is_some(){
                let pattern_unwraped = pattern.unwrap().as_ref().unwrap().get_value();
                
                operation_status = grep(object, pattern_unwraped);
            }
            else{
                operation_status = Err(Error::new(ErrorKind::InvalidInput, "Invoker Error: Didn't provide destination."));
            }
        },
        CommandType::EXIT => {
            operation_status = exit();
        },
        CommandType::INVALID => {
            operation_status = invalid();
        },
    }

    return operation_status
}

/* 

    INVOKER MIDDLEWARE
    Intented to relieve some complexity form the function calls and add some abstraction to how the functions are called.

    
*/

fn home(terminal_instance: &mut Terminal) -> Result<Data, Error>{
    return functions::home(terminal_instance)
}


fn cwd(terminal_instance: &mut Terminal) -> Result<Data, Error>{
    return functions::cwd(terminal_instance)
}


fn touch(data: Data) -> Result<Data, Error>{
    match data {
        Data::PathData(path) => {
            return functions::touch(path.as_path());
        },
        Data::StringData(string) => {
            todo!("read")
        },
        Data::DirPathData(file_paths) => {
            todo!("list")
        },
        Data::VecStringData(string_vec) => {
            todo!("grep")
        }
        _ => Err(Error::new(ErrorKind::InvalidInput, "Invoker Error: Didn't provide a path.")),
    }
}


fn mkdir(data: Data, recursive: bool) -> Result<Data, Error>{
    match data {
        Data::PathData(path) => {
            return functions::mkdir(path.as_path(), recursive)
        }
        _ => Err(Error::new(ErrorKind::InvalidInput, "Invoker Error: Didn't provide a path.")),
    }
}


fn remove(data: Data, recursive: bool) -> Result<Data, Error>{
    match data {
        Data::PathData(path) => {
            return functions::remove(path.as_path(), recursive)
        }
        _ => Err(Error::new(ErrorKind::InvalidInput, "Invoker Error: Didn't provide a path.")),
    }
}


fn copy(origin_data: Data, destination_data: Data, terminal_instance: &mut Terminal) -> Result<Data, Error>{
    match (origin_data, destination_data) {
        (Data::PathData(origin), Data::PathData(destination)) => {
            return functions::copy(&origin.as_path(), &destination.as_path(), terminal_instance)
        },
        _ => Err(Error::new(ErrorKind::InvalidInput, "Invoker Error: Didn't provide a path."))
    }
}


fn r#move(origin_data: Data, destination_data: Data, terminal_instance: &mut Terminal) -> Result<Data, Error>{
    match (origin_data, destination_data) {
        (Data::PathData(origin), Data::PathData(destination)) => {
            return functions::r#move(&origin.as_path(), &destination.as_path(), terminal_instance)
        },
        _ => Err(Error::new(ErrorKind::InvalidInput, "Invoker Error: Didn't provide a path."))
    }
}


fn read(data: Data) -> Result<Data, Error>{
    match data {
        Data::PathData(path) => {
            return functions::read(path.as_path());
        }
        _ => Err(Error::new(ErrorKind::InvalidInput, "Invoker Error: Didn't provide a path.")),
    }
}



fn list(data: Data, hidden: bool) -> Result<Data, Error>{
    match data {
        Data::PathData(path) => {
            return functions::list(path.as_path(), hidden)
        }
        _ => Err(Error::new(ErrorKind::InvalidInput, "Invoker Error: Didn't provide a path.")),
    }
}



fn traverse_directory(data: Data, terminal_instance: &mut Terminal) -> Result<Data, Error>{
    match data {
        Data::PathData(path) => {
            return functions::traverse_directory(path.as_path(), terminal_instance)
        }
        _ => Err(Error::new(ErrorKind::InvalidInput, "Invoker Error: Didn't provide a path.")),
    }
}



fn grep(data: Data, pattern: &String) -> Result<Data, Error>{
    match data {
        //normal usage + touch
        Data::PathData(path) => {
            return functions::grep(path.as_path(), pattern)
        },
        //read result
        Data::StringData(string) => {
            let string_input: Vec<String> = vec![string];
            return functions::grep_from_string(string_input, pattern);
        },
        //list results
        Data::DirPathData(file_paths) => {
            let mut grep_output: Vec<Data> = vec![];

            for path in file_paths{
                let result = functions::grep(path.as_path(), pattern);
                if result.is_ok(){
                    grep_output.push(result.unwrap());
                }
                else{
                    return Err(result.err().unwrap());
                }
            }
            return Ok(Data::DataVector(Box::new(grep_output)))
        }
        _ => Err(Error::new(ErrorKind::InvalidInput, "Invoker Error: Didn't provide a path.")),
    }
}



fn exit() -> Result<Data, Error>{
    return functions::exit()
}



fn invalid() -> Result<Data, Error>{
    return functions::invalid()
}