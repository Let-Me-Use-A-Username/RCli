use std::collections::VecDeque;
use std::ffi::{OsStr, OsString};
use std::fs::{self, DirBuilder, File, OpenOptions};
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
            let flag: bool = match flag_vector.pop_front() {
                Some(flag) => {
                    match flag.get_value().0 {
                        Some(value) => {
                            core.containt_flag(&value)
                        },
                        None => false
                    }
                }
                None => false
            };

            let _ = mkdir(path_value, flag);
        },
        TokenCommandType::REMOVE => {
            let flag: bool = match flag_vector.pop_front() {
                Some(flag) => {
                    match flag.get_value().0 {
                        Some(value) => {
                            core.containt_flag(&value)
                        },
                        None => false
                    }
                }
                None => false
            };

            let _ = remove(path_value, flag);
        },
        TokenCommandType::COPY => {
            match flag_vector.pop_front() {
                Some(flag) => {
                    let flag_value = flag.get_value();

                    let flag = flag_value.0.unwrap_or("INVALID".to_string());
                    let obj = flag_value.1.unwrap_or("INVALID".to_string());

                    if core.containt_flag(&flag) & !obj.eq("INVALID"){
                        let new_path = terminal_instance.get_current_directory().join(path_value);
                        let destination_path = terminal_instance.get_current_directory().join(obj);
                        
                        copy(new_path.as_path(), destination_path.as_path());
                    }
                    else{
                        eprintln!("INTERNAL ERROR: Unknown flag {flag:?}.")
                    }
                },
                None => {
                    eprintln!("INTERNAL ERROR: No destination provided");
                }
            };
        },
        TokenCommandType::MOVE => {
            //move or rename file
            //after confirming it moved, delete origin
            match flag_vector.pop_front() {
                Some(flag) => {
                    let flag_value = flag.get_value();

                    let flag = flag_value.0.unwrap_or("INVALID".to_string());
                    let obj = flag_value.1.unwrap_or("INVALID".to_string());

                    if core.containt_flag(&flag) & !obj.eq("INVALID"){
                        let new_path = terminal_instance.get_current_directory().join(path_value);
                        let destination_path = terminal_instance.get_current_directory().join(obj);
                        
                        rename(new_path.as_path(), destination_path.as_path());
                    }
                    else{
                        eprintln!("INTERNAL ERROR: Unknown flag {flag:?}.")
                    }
                },
                None => {
                    eprintln!("INTERNAL ERROR: No destination provided");
                }
            };
        },
        TokenCommandType::READ => todo!(),
        TokenCommandType::LIST => {
            let flag: bool = match flag_vector.pop_front() {
                Some(flag) => {
                    match flag.get_value().0 {
                        Some(value) => {
                            core.containt_flag(&value)
                        },
                        None => false
                    }
                }
                None => false
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
    if path.try_exists().unwrap(){

        if path.is_file(){
            match fs::copy(path, destination){
                Ok(_) => return,
                Err(error) => {
                    handle_error(&error)
                },
            }
        }
        else if path.is_dir(){
            read_dir(path, Some(destination));
        }
        //tricky clause.
        else{
            eprintln!("INTERNAL ERROR: Path not recognized as a file or a directory.")
        }
    }
    else{
        eprintln!("INTERNAL ERROR: Path |{path:?}| doesn't exist.")
    }
}

///Moves and renames a file or directory
fn rename(path: &Path, destination: &Path){
    //https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-movefileexa
    
    
    //problem due to permissions
    //let res = fs::rename(path, destination);
    //println!("result {res:?}");
}

///Lists items in a directory
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
                        
                        //if hidden is true show everything
                        if hidden{
                            outputbuffer.push(dir_path.display().to_string().replace("\\\\?\\", ""));
                        }
                        //else hidden flag is false. if dir DOESNT have hidden flag append it
                        else if !hidden & !entry_attributes.contains(&windows_file_attributes::WindowsAttributes::HIDDEN){
                            outputbuffer.push(dir_path.display().to_string().replace("\\\\?\\", ""));
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


///Traverses given path if valid
fn traverse_directory(path: &Path, terminal_instance: &mut Terminal) -> Result<PathBuf, PathBuf>{
    let mut pathbuffer = PathBuf::new();
    pathbuffer.push(path);

    return terminal_instance.set_current_directory(pathbuffer);
}


///Exits RCli
fn exit(){
    //todo! handle process exit more robustly, check doc for std::process::exit
    //return res to parser, and then to terminal and exit
    std::process::exit(1);
}

///Processes invalid commands
fn invalid(){
    eprintln!("ERROR: Invalid command.")
}


/*
    HELPER FUNCTIONS
*/

///Helper function to recursively copy a directory with its content.
///Mimics DFS algorithms. Used in cp/copy
fn read_dir(original_path: &Path, destination: Option<&Path>){

    if destination.is_some(){
        fs::create_dir_all(destination.unwrap()).ok();
    }
    
    match fs::read_dir(original_path){
        Ok(dir_paths) => {
            for path in dir_paths{
                let path_type = path.unwrap().path();

                let new_file = path_type.file_name().unwrap();
                let new_path = destination.unwrap().join(new_file);

                if path_type.is_dir(){
                    //recurse
                    if destination.is_some(){
                        read_dir(&path_type, Some(&new_path))
                    }
                }
                else{
                    //add to stack
                    let _ = fs::copy(path_type, new_path);
                }
            }
        },
        Err(error) => {
            handle_error(&error)
        },
    }
}