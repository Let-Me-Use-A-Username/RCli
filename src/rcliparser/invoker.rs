use std::fs::{self, DirBuilder, File, OpenOptions};
use std::io::{Error, ErrorKind};

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

fn create_file(file_name: String) -> Result<File, Error>{
    //could not need the open() clause unless pipelining
    let file = OpenOptions::new().write(true).create(true).open(file_name);

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

fn create_dir(path: String, recursive: bool) -> Result<(), Error>{
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

fn list_dir(dir_path: String, hidden: bool){
    let dir_entries = fs::read_dir(dir_path);
}