use std::collections::VecDeque;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

use crate::rcliparser::objects::grammar_objects::CommandType;
use crate::rcliterminal::terminal::Terminal;

use super::objects::data_types::Data;
use super::objects::grammar_objects::FlagType;
use super::objects::token_objects::Invocator;
use super::utils::functions;


pub fn invoke(invocation: Invocator, terminal_instance: &mut Terminal) -> Result<Data, Error>{
    let operation_status: Result<Data, Error>;
    
    let core_command = invocation.get_type();
    let mut data = match invocation.get_data() {
        Data::DataVector(vector) => {
            *vector
        }
        _ => unreachable!()
    };
    let core_object = data.pop_front().unwrap();
    let flags = invocation.get_flags();

    match core_command{
        CommandType::HOME => {
            operation_status = home(terminal_instance);
        },
        CommandType::CWD => {
            operation_status = cwd(terminal_instance);
        },
        CommandType::ECHO => {
            operation_status = echo(core_object)
        }
        CommandType::TOUCH => {
            if data.len() >= 1{
                operation_status = touch(core_object, Some(data));
            }
            else{
                operation_status = touch(core_object, None);
            }
            
        },
        CommandType::MKDIR => {
            let recursive = flags.get(&FlagType::RECURSIVE);

            if recursive.is_some(){
                operation_status = mkdir(core_object, true);
            }
            else{
                operation_status = mkdir(core_object, false);
            }
        },
        CommandType::REMOVE => {
            let recursive = flags.get(&FlagType::RECURSIVE);

            if recursive.is_some(){
                operation_status = remove(core_object, true);
            }
            else{
                operation_status = remove(core_object, false);
            }
        },
        CommandType::COPY => {
            let destination = flags.get(&FlagType::DESTINATION);

            if destination.is_some(){
                let destination = Data::SimpleData(destination.unwrap().as_ref().unwrap().get_object());
                
                operation_status = copy(core_object, destination, terminal_instance);
            }
            else{
                operation_status = Err(Error::new(ErrorKind::InvalidInput, "Invoker Error: Didn't provide destination."));
            }
        },
        CommandType::MOVE => {
            let destination = flags.get(&FlagType::DESTINATION);

            if destination.is_some(){
                let destination = Data::SimpleData(destination.unwrap().as_ref().unwrap().get_object());
                
                operation_status = r#move(core_object, destination, terminal_instance);
            }
            else{
                operation_status = Err(Error::new(ErrorKind::InvalidInput, "Invoker Error: Didn't provide destination."));
            }
        },
        CommandType::READ => {
            operation_status = read(core_object);
        },
        CommandType::LIST => {
            let hidden: bool = (|| {
                let flag = flags.get(&FlagType::HIDDEN);

                if flag.is_some(){ 
                    return true
                }
                return false
            })();
            
            let recursive: bool = (|| {
                let flag = flags.get(&FlagType::RECURSIVE);

                if flag.is_some(){ 
                    return true
                }
                return false
            })();

            operation_status = list(core_object, hidden, recursive);
        },
        CommandType::CD => {
            let mut destination_path = PathBuf::from(terminal_instance.get_current_directory());
            destination_path.push(core_object.get_path().unwrap());

            let destination_data = Data::SimpleData(destination_path.display().to_string());
            
            operation_status = traverse_directory(destination_data, terminal_instance);
        },
        CommandType::GREP => {
            let destination = (|| {
                let flag = flags.get(&FlagType::DESTINATION);

                if flag.is_some(){
                    let data = Data::PathData(PathBuf::from(flag.unwrap().as_ref().unwrap().get_object()));
                    return Some(data)
                }
                return None
            })();

            if destination.is_some(){
                operation_status = grep_from_path(core_object, destination.unwrap());
            }
            else{
                operation_status = grep_from_string(core_object, data);
            }
        },
        CommandType::FIND => {
            let destination: Data = (|| {
                let flag = flags.get(&FlagType::DESTINATION);

                if flag.is_some(){ 
                    return Data::SimpleData(flag.unwrap().as_ref().unwrap().get_object());
                }
                return Data::SimpleData(terminal_instance.get_current_directory().display().to_string())
            })();

            operation_status = find(core_object, destination)
        }
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


fn echo(string: Data) -> Result<Data, Error>{
    match string{
        Data::SimpleData(simple) => {
            return functions::echo(&simple)
        }
        _ => unreachable!()
    }
}


fn touch(current_path: Data, data: Option<VecDeque<Data>>) -> Result<Data, Error>{
    let file_path = current_path.get_path();

    if file_path.is_some(){
        if data.is_some(){
            let mut return_result: VecDeque<Data> = vec![].into();

            for data_type in data.unwrap(){
                let touch_result: Result<Data, Error>;

                match data_type{
                    Data::SimpleData(simple) => {
                        touch_result = functions::touch(file_path.unwrap(), Some(simple))
                    }
                    _ => continue
                }

                if touch_result.is_ok(){
                    return_result.push_back(touch_result.unwrap());
                    continue;
                }
                return Err(touch_result.err().unwrap())
            }
            return Ok(Data::DataVector(return_result.into()))
        }
        return Ok(functions::touch(file_path.unwrap(), None).unwrap())
    }
    return Err(Error::new(ErrorKind::InvalidInput, "Invoker Error: First parameter wasn't a path."))
}


fn mkdir(data: Data, recursive: bool) -> Result<Data, Error>{
    match data {
        Data::SimpleData(path) => {
            return functions::mkdir(Path::new(&path), recursive)
        }
        _ => Err(Error::new(ErrorKind::InvalidInput, "Invoker Error: Didn't provide a path.")),
    }
}


fn remove(data: Data, recursive: bool) -> Result<Data, Error>{
    match data {
        Data::SimpleData(path) => {
            return functions::remove(Path::new(&path), recursive)
        }
        _ => Err(Error::new(ErrorKind::InvalidInput, "Invoker Error: Didn't provide a path.")),
    }
}


fn copy(origin_data: Data, destination_data: Data, terminal_instance: &mut Terminal) -> Result<Data, Error>{
    match (origin_data, destination_data) {
        (Data::SimpleData(origin), Data::SimpleData(destination)) => {
            return functions::copy(Path::new(&origin), Path::new(&destination), terminal_instance)
        },
        _ => Err(Error::new(ErrorKind::InvalidInput, "Invoker Error: Didn't provide a path."))
    }
}


fn r#move(origin_data: Data, destination_data: Data, terminal_instance: &mut Terminal) -> Result<Data, Error>{
    match (origin_data, destination_data) {
        (Data::SimpleData(origin), Data::SimpleData(destination)) => {
            return functions::r#move(Path::new(&origin), Path::new(&destination), terminal_instance)
        },
        _ => Err(Error::new(ErrorKind::InvalidInput, "Invoker Error: Didn't provide a path."))
    }
}


fn read(data: Data) -> Result<Data, Error>{
    match data {
        Data::SimpleData(path) => {
            return functions::read(Path::new(&path));
        }
        _ => Err(Error::new(ErrorKind::InvalidInput, "Invoker Error: Didn't provide a path.")),
    }
}



fn list(data: Data, hidden: bool, recursive: bool) -> Result<Data, Error>{
    match data {
        Data::SimpleData(path) => {
            return functions::list(Path::new(&path), hidden, recursive)
        }
        _ => Err(Error::new(ErrorKind::InvalidInput, "Invoker Error: Didn't provide a path.")),
    }
}



fn traverse_directory(data: Data, terminal_instance: &mut Terminal) -> Result<Data, Error>{
    match data {
        Data::SimpleData(path) => {
            return functions::traverse_directory(Path::new(&path), terminal_instance)
        }
        _ => Err(Error::new(ErrorKind::InvalidInput, "Invoker Error: Didn't provide a path.")),
    }
}



fn grep_from_path(pattern: Data, data_path: Data) -> Result<Data, Error>{
    let string_pattern = &pattern.get_value().unwrap();
    match data_path{
        Data::PathData(p) => {
            return functions::grep(p.as_path(), string_pattern)
        },
        _ => Err(Error::new(ErrorKind::InvalidInput, "Invoker Error: Invalid path.")),
    }
}


fn grep_from_string(pattern: Data, data: VecDeque<Data>) -> Result<Data, Error>{
    let string_pattern = &pattern.get_value().unwrap();

    let mut return_result: VecDeque<Data> = vec![].into();

    for data_type in data{
        let match_result: Option<String>;

        match data_type{
            Data::SimpleData(simple) => {
                match_result = functions::match_string(simple, string_pattern)
            },
            _ => continue
        }

        if match_result.is_some(){
            return_result.push_back(Data::StringData(match_result.unwrap()))
        }
    }

    return Ok(Data::DataVector(return_result.into()))
}


fn find(data: Data, target: Data) -> Result<Data, Error>{
    match data{
        Data::SimpleData(object) => {
            let result = functions::find(&object, target.get_path().unwrap());
            
            if result.is_ok(){
                if result.as_ref().unwrap().is_some(){
                    return Ok(result.unwrap().unwrap())
                }

                return Ok(Data::StringData("No object found".to_string()))
            }

            return Err(result.unwrap_err())
        },
        _ => unreachable!()
    }
}



fn exit() -> Result<Data, Error>{
    return functions::exit()
}



fn invalid() -> Result<Data, Error>{
    return functions::invalid()
}