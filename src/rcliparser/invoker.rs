use std::fs::{self, DirBuilder, File, OpenOptions};
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

use crate::rcliterminal::terminal_singlenton::Terminal;

use crate::rcliparser::parser::FlagObjectPair;

use super::objects::tokens::{GetValue, TokenCommands, TokenObjects};
use super::utils::dotparser;


pub fn invoke(core: TokenCommands, path: TokenObjects, flag_vector: Vec<FlagObjectPair>, terminal_instance: &mut Terminal){

    println!("\nCORE: {:?}", core.clone());
    println!("OBJECT: {:?}", path.clone());
    println!("FLAGS: {:?}", flag_vector.clone());

    let token_value = path.get_value();
    
    dotparser::parse_path(token_value.clone(), terminal_instance);
    
    let path_value = Path::new(&token_value);

    println!("Path value: {path_value:?}");

    match core{
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
            //todo! check for hidden flag
            //https://users.rust-lang.org/t/read-windows-hidden-file-attribute/51180/6
            list(path_value, false);
            
        },
        TokenCommands::CD => {
            let exists = path_value.exists();
            println!("Path exists: {exists:?}");
            
            if path_value.is_dir() & exists{
                traverse_directory(path_value, terminal_instance);
            }
            else{
                eprintln!("Path is not dir or doesn't exist");
            }
        },
        TokenCommands::EXIT => todo!(),
        TokenCommands::INVALID => todo!(),
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
                outputbuffer.push(path.unwrap().path().display().to_string().replace("\\\\?\\", ""));
            }
        
            for obj in outputbuffer{
                println!("{}", obj);
            }
        },
        Err(error) => {
            handle_error(&error);
        },
    };
}

fn traverse_directory(path: &Path, terminal_instance: &mut Terminal){
    let mut pathbuffer = PathBuf::new();
    pathbuffer.push(path);

    terminal_instance.set_current_directory(pathbuffer);
}