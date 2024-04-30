use std::fs::{self, DirBuilder, File, OpenOptions};
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

use crate::rcliterminal::terminal_singlenton::Terminal;

use crate::rcliparser::parser::FlagObjectPair;

use super::objects::tokens::{TokenCommands, TokenObjects};


pub fn invoke(core: TokenCommands, path: TokenObjects, flag_vector: Vec<FlagObjectPair>, terminal_instance: &mut Terminal){

    println!("CORE: {:?}", core.clone());
    println!("OBJECT: {:?}", path.clone());
    println!("FLAGS: {:?}", flag_vector.clone());

    match core{
        TokenCommands::TOUCH => {
            match &path{
                TokenObjects::FILE(file) => {
                    touch(&Path::new(file));
                },
                _ => {
                    todo!("throw error");
                }
            }
        },
        TokenCommands::MKDIR => {
            match &path{
                TokenObjects::DIRECTORY(dir) => {
                    mkdir(&Path::new(dir), false);
                },
                _ => {
                    todo!("throw error");
                }
            }
        },
        TokenCommands::DELETE => todo!(),
        TokenCommands::COPY => todo!(),
        TokenCommands::MOVE => todo!(),
        TokenCommands::READ => todo!(),
        TokenCommands::LIST => {
            //todo! check for hidden flag
            //https://users.rust-lang.org/t/read-windows-hidden-file-attribute/51180/6
            match &path{
                TokenObjects::DIRECTORY(dir) => {
                    list(&Path::new(dir) , false);
                },
                _ => {
                    todo!("throw error");
                }
            }
            
        },
        TokenCommands::CD => {
            match &path{
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

    let paths = fs::read_dir(dir_path).unwrap();

    for path in paths{
        outputbuffer.push(path.unwrap().path().display().to_string().replace("\\\\?\\", ""));
    }

    for obj in outputbuffer{
        println!("{}", obj);
    }
}

fn traverse_directory(path: &Path, terminal_instance: &mut Terminal){
    let mut pathbuffer = PathBuf::new();
    pathbuffer.push(path);

    terminal_instance.set_current_directory(pathbuffer);
}