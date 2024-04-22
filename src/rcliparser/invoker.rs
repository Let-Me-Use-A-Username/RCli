use std::fs::{self, DirBuilder, File, FileType, OpenOptions};
use std::io::{Error, ErrorKind};
use std::path::Path;

use super::lexical_analyzer::Tokens;


pub fn invoke(token: Tokens){

}

fn handle_error(error: &Error){
    match error.kind(){
        ErrorKind::NotFound => {

        },
        _ => {

        }
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

fn list_dir(dir_path: &Path, hidden: bool){
    let dir_entries = fs::read_dir(dir_path);

}