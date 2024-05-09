use std::collections::VecDeque;
use std::fs::{self, DirBuilder, DirEntry, File, OpenOptions};
use std::io::{Error, ErrorKind};
use std::os::windows::fs::MetadataExt;
use std::path::{Path, PathBuf};

use crate::rcliparser::utils::windows_file_attributes;
use crate::rcliterminal::terminal_singlenton::Terminal;

use super::objects::tokens::{FlagObjectPair, GetTupleValue, GetValue, TokenCommandType, TokenCommands, TokenObjects};


pub fn invoke(core: TokenCommands, path: TokenObjects, mut flag_vector: VecDeque<FlagObjectPair>, terminal_instance: &mut Terminal){

    // println!("\nCORE: {:?}", core.clone());
    // println!("OBJECT: {:?}", path.clone());
    // println!("FLAGS: {:?}", flag_vector.clone());

    let token_value = path.get_value();
    let path_value = Path::new(&token_value);

    match core.get_type(){
        TokenCommandType::CWD => {
            cwd(terminal_instance.get_current_directory_to_string());
        }
        TokenCommandType::TOUCH => {
            let _ = touch(path_value);
        },
        TokenCommandType::MKDIR => {
            //todo! change this
            let flag = match flag_vector.pop_front() {
                Some(flag) => {
                    let flag_value = flag.get_value();
                    let flag = flag_value.0;
                    let obj = flag_value.1;
                    
                    core.containt_flag(&flag.unwrap_or("?".to_string()))
                },
                _ => false
            };

            let _ = mkdir(path_value, flag);
        },
        TokenCommandType::REMOVE => {
            //todo! change this
            let flag = match flag_vector.pop_front() {
                Some(flag) => {
                    let flag_value = flag.get_value();
                    let flag = flag_value.0;
                    let obj = flag_value.1;
                    
                    core.containt_flag(&flag.unwrap_or("?".to_string()))
                },
                _ => false
            };

            let _ = remove(path_value, flag);
        },
        TokenCommandType::COPY => {
            //todo! change this
            let flag = match flag_vector.pop_front() {
                Some(flag) => {
                    let flag_value = flag.get_value();
                    let flag = flag_value.0;
                    let obj = flag_value.1.unwrap();
                    
                    let destination_path = Path::new(&obj);
                    copy(path_value, destination_path);
                },
                _ => {
                    eprintln!("INTERNAL ERROR: No destination provided");
                    return
                }
            };
        },
        TokenCommandType::MOVE => todo!(),
        TokenCommandType::READ => todo!(),
        TokenCommandType::LIST => {
            //todo! change this
            let flag = match flag_vector.pop_front() {
                Some(flag) => {
                    let flag_value = flag.get_value();
                    let flag = flag_value.0;
                    let obj = flag_value.1;
                    
                    core.containt_flag(&flag.unwrap_or("?".to_string()))
                },
                _ => false
            };

            list(path_value, flag);
            
        },
        TokenCommandType::CD => {
            let new_path = terminal_instance.get_current_directory().join(path_value);

            let original_path_exists = new_path.try_exists().unwrap_or(false);

            if original_path_exists{
                let _ = traverse_directory(new_path.as_path(), terminal_instance);
            }
            else{
                eprintln!("INVALID PATH");
            }
        },
        TokenCommandType::EXIT => {
            exit();
        },
        TokenCommandType::INVALID => {
            invalid();
        },
    }
}


fn handle_error(error: &Error){
    match error.kind(){
        ErrorKind::PermissionDenied => {
            eprintln!("ERROR HANDLER: Permission denied.")
        },
        ErrorKind::NotFound => {
            eprintln!("ERROR HANDLER: Object not found.")
        },
        ErrorKind::AlreadyExists => {
            eprintln!("ERROR HANDLER: Object already exists.")
        },
        _ => {

        }
    }

}


///Shows current working dir
fn cwd(path: String){
    println!("{path}");
}


///Creates a file at the given path
fn touch(file_path: &Path) -> Result<File, Error>{
    //could not need the open() clause unless pipelining
    let file = OpenOptions::new().write(true).create(true).open(file_path);

    match file{
        Ok(res) => {
            //if persmisions

            //else
            return Ok(res);

        },
        Err(error) => {
            handle_error(&error);
            return Err(error);
        }
    }
}


///Creates a directory at the given path
fn mkdir(path: &Path, recursive: bool) -> Result<(), Error>{
    let mut builder = DirBuilder::new();
    
    builder.recursive(recursive);

    let directory = builder.create(path);

    match directory{
        Ok(dir) => {
            //if persmisions

            //else
            return Ok(dir);
        },
        Err(error) => {
            handle_error(&error);
            return Err(error);
        }
        
    }
}


///Removes a file or dir.
fn remove(path: &Path, recursive: bool){
    let mut res : Result<(), Error> = Result::Err(Error::new(ErrorKind::NotFound, "Initialize"));
    if path.try_exists().is_ok(){
        if path.is_dir() & recursive{
            res = fs::remove_dir_all(path);            
        }
        else if path.is_dir(){
            res = fs::remove_dir(path);
        }
        else if path.is_file(){
            res = fs::remove_file(path);
        }
    }

    match res{
        Ok(_) => return,
        Err(error) => {
            handle_error(&error)
        },
    }
}


///Copies the content of either a file or a directory
fn copy(path: &Path, destination: &Path){
    if path.try_exists().unwrap_or(false){
        if path.is_file(){
            match fs::copy(path, destination){
                Ok(_) => return,
                Err(error) => {
                    handle_error(&error)
                },
            }
        }
        else if path.is_dir(){
            let directory_stack = read_dir(path);
        }
        //tricky clause.
        else{
            eprintln!("INTERNAL ERROR: Path not recognized as a file or a directory.")
        }
    }
    eprintln!("INTERNAL ERROR: Path doesn't exist.")
}


///recursive function that reads a directory and returns a stack(?)
//https://stackoverflow.com/questions/26958489/how-to-copy-a-folder-recursively-in-rust

//consider using an enum that contains either a dir or a file and add a stack
fn read_dir(path: &Path){

    let path_components = path.components().collect::<Vec<_>>();
    
    if path_components.len() == 1{
        fs::read_dir(path).unwrap().collect::<Vec<_>>();
    }

}


fn list(dir_path: &Path, hidden: bool){
    let mut outputbuffer: Vec<String> = vec![];

    match fs::read_dir(dir_path) {
        Ok(paths) => {
            for path in paths{
                let dir_path = path.unwrap().path();

                match fs::metadata(dir_path.clone()) {
                    Ok(meta) => {
                        let attributes = meta.file_attributes();

                        let entry_attributes = windows_file_attributes::match_attributes(attributes);
                        
                        if hidden{
                            outputbuffer.push(dir_path.display().to_string().replace("\\\\?\\", ""));
                        }
                        else{
                            if !entry_attributes.contains(&windows_file_attributes::WindowsAttributes::HIDDEN){
                                outputbuffer.push(dir_path.display().to_string().replace("\\\\?\\", ""));
                            }
                        }
                    },
                    Err(error) => {
                        eprintln!("INTERNAL ERROR: Couldn't read object metadata {error:?}");
                        handle_error(&error)
                    }
                }; 
            }
        },
        Err(error) => {
            handle_error(&error);
        },
    };

    for obj in outputbuffer{
        println!("{}", obj);
    }
}


fn traverse_directory(path: &Path, terminal_instance: &mut Terminal) -> Result<PathBuf, PathBuf>{
    let mut pathbuffer = PathBuf::new();
    pathbuffer.push(path);

    return terminal_instance.set_current_directory(pathbuffer);
}


fn exit(){
    //todo! handle process exit more robustly, check doc for std::process::exit
    //return res to parser, and then to terminal and exit
    std::process::exit(1);
}


fn invalid(){
    eprintln!("ERROR: Invalid command.")
}