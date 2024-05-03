use std::collections::VecDeque;
use std::fs::{self, DirBuilder, File, OpenOptions};
use std::io::{Error, ErrorKind};
use std::os::windows::fs::MetadataExt;
use std::path::{Path, PathBuf};

use crate::rcliparser::utils::windows_file_attributes;
use crate::rcliterminal::terminal_singlenton::Terminal;

use super::objects::tokens::{FlagObjectPair, GetValue, TokenCommands, TokenObjects};
use super::utils::dotparser;


pub fn invoke(core: TokenCommands, path: TokenObjects, mut flag_vector: VecDeque<FlagObjectPair>, terminal_instance: &mut Terminal){

    let syntax = terminal_instance.get_instance_syntax();

    // println!("\nCORE: {:?}", core.clone());
    // println!("OBJECT: {:?}", path.clone());
    // println!("FLAGS: {:?}", flag_vector.clone());

    let token_value = path.get_value();
    let path_value = Path::new(&token_value);

    match core{
        TokenCommands::CWD => {
            cwd(terminal_instance.get_current_directory_to_string());
        }
        TokenCommands::TOUCH => {
            let _ = touch(path_value);
        },
        TokenCommands::MKDIR => {
            let _ = mkdir(path_value, false);
        },
        TokenCommands::DELETE => todo!(),
        TokenCommands::COPY => todo!(),
        TokenCommands::MOVE => todo!(),
        TokenCommands::READ => todo!(),
        TokenCommands::LIST => {
            let flag: bool = match flag_vector.pop_front() {
                Some(f) => {
                    let found: bool = match f{
                        FlagObjectPair::SOLE(terminal) => {
                            let terminal_value = terminal.get_value();
                            let mut res = false;

                            //todo! change this to something better
                            for command in syntax{
                                if command.match_name_iter(&"list".to_string()){
                                    res = command.match_flag_iter(&terminal_value);
                                    if res {break}
                                }
                            }
                            res
                        },
                        _ => {
                            false
                        }
                    };
                    found
                },
                None => false,
            };

            list(path_value, flag);
            
        },
        TokenCommands::CD => {
            let exists = path_value.exists();
            
            if path_value.is_dir() & exists{
                traverse_directory(path_value, terminal_instance);
            }
            else{
                //todo! fix, this is dumb
                let error_path = dotparser::parse_root_dir(path_value, terminal_instance);
                if error_path.exists() {
                    traverse_directory(&error_path, terminal_instance);
                    return;
                }
                eprintln!("Path is not dir or doesn't exist");
            }
        },
        TokenCommands::EXIT => {
            exit();
        },
        TokenCommands::INVALID => {
            invalid();
        },
    }
}


fn handle_error(error: &Error){
    match error.kind(){
        ErrorKind::PermissionDenied => {
            eprintln!("Permission denied.")
        },
        ErrorKind::NotFound => {
            eprintln!("Object not found.")
        },
        ErrorKind::AlreadyExists => {
            eprintln!("Object already exists.")
        },
        _ => {

        }
    }

}

fn cwd(path: String){
    println!("{path}");
}
  
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

fn traverse_directory(path: &Path, terminal_instance: &mut Terminal){
    let mut pathbuffer = PathBuf::new();
    pathbuffer.push(path);

    terminal_instance.set_current_directory(pathbuffer);
}

fn exit(){
    //todo! handle process exit more robustly, check doc for std::process::exit
    //return res to parser, and then to terminal and exit
    std::process::exit(1);
}

fn invalid(){
    eprintln!("ERROR: Invalid command.")
}