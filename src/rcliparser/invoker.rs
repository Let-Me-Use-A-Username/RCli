use std::collections::VecDeque;
use std::env;
use std::fs::{self, DirBuilder, File, OpenOptions};
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

use crate::rcliterminal::terminal_singlenton::Terminal;

use super::lexical_analyzer::Tokens;
use super::lexical_analyzer::TokenCommands;
use super::lexical_analyzer::TokenObjects;


pub fn invoke(core: TokenCommands, mut parameters: VecDeque<Tokens>, terminal_instance: &mut Terminal){
    //set the current directory in case the core command is on local dir and full path isnt specified
    terminal_instance.change_current_directory(env::current_dir().unwrap());
    let current_dir_string =  terminal_instance.get_current_directory_to_string();

    //current problem: first parameter is always path, has to be changed
    //if path is something invalid , then it is set to current working directory
    let path: Result<TokenObjects, _> = parameters.pop_front().unwrap_or(Tokens::TokenObjects(TokenObjects::DIRECTORY(current_dir_string))).try_into();

    //interpret . .. ./ path files

    println!("{:?}", path.clone().unwrap());

    match core{
        TokenCommands::CREATE => {
            create(&path.ok().unwrap());
        },
        TokenCommands::DELETE => todo!(),
        TokenCommands::COPY => todo!(),
        TokenCommands::MOVE => todo!(),
        TokenCommands::READ => todo!(),
        TokenCommands::LIST => {
            //todo! check for hidden flag
            //https://users.rust-lang.org/t/read-windows-hidden-file-attribute/51180/6
            match &path.ok().unwrap(){
                TokenObjects::DIRECTORY(dir) => {
                    list(&Path::new(dir) , false);
                },
                _ => {
                    todo!("throw error");
                }
            }
            
        },
        TokenCommands::CD => {
            match &path.ok().unwrap(){
                TokenObjects::DIRECTORY(dir) => {
                    traverse_directory(&Path::new(dir), terminal_instance);
                },
                _ => {
                    todo!("throw error");
                }
            }
            
        },
        TokenCommands::EXIT => todo!(),
        TokenCommands::INVALID => todo!(),
    }
}


fn handle_error(error: &Error){
    match error.kind(){
        ErrorKind::NotFound => {

        },
        _ => {

        }
    }

}

fn create(path: &TokenObjects){
    match path{
        TokenObjects::FILE(file) => {
            let res = create_file(Path::new(file));
            if res.is_ok(){
                //todo!("log message created");
                return;
            }
            handle_error(&res.err().unwrap());
        },
        TokenObjects::DIRECTORY(dir) => {
            let res = create_dir(Path::new(dir), false);
            if res.is_ok(){
                //todo!("log message created");
                return;
            }
            handle_error(&res.err().unwrap());
        },
    }

}
  
fn create_file(file_path: &Path) -> Result<File, Error>{
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

fn create_dir(path: &Path, recursive: bool) -> Result<(), Error>{
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

    let paths = fs::read_dir(dir_path).unwrap();

    for path in paths{
        outputbuffer.push(path.unwrap().path().display().to_string().replace("\\", "/"));
    }

    for obj in outputbuffer{
        println!("{}", obj);
    }
}

fn traverse_directory(path: &Path, terminal_instance: &mut Terminal){
    //todo! check if path like .. , ./ and . have to be checked by hand or if Rust understands them
    let mut pathbuffer = PathBuf::new();
    pathbuffer.push(path);

    terminal_instance.change_current_directory(pathbuffer);
}