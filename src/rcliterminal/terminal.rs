use std::env;
use std::io::Error;
use std::path::PathBuf;
use std::sync::Mutex;

use crate::rcliparser::objects::data_types::Data;
use crate::rcliparser::objects::grammar_objects::Grammar;
use crate::rcliparser::utils::grammar_reader;


///Singlenton terminal
pub struct Terminal{
    user_home_directory: Mutex<PathBuf>,
    current_directory: Mutex<PathBuf>,
    grammar: Mutex<Grammar>
}

impl Terminal{
    pub fn new() -> Self{
        Terminal {
            user_home_directory: Mutex::new(dirs::home_dir().unwrap()), 
            current_directory: Mutex::new(env::current_dir().unwrap()), 
            grammar: Mutex::new(grammar_reader::load_grammar()) 
        }
    }

    pub fn set_current_directory(&mut self, path: PathBuf) -> Result<Data, Error>{
        let mut current_dir = self.current_directory.lock().unwrap();
        match path.canonicalize() {
            //check if path is valid
            Ok(new_path) => {
                //set it as current directory
                let operation_results = env::set_current_dir(new_path.clone());
                //todo! handle error in case the operation fails
                match operation_results {
                    Ok(_) => {
                        *current_dir = env::current_dir().unwrap();
                        return Ok(Data::StatusData(100));
                    },
                    Err(error) => {
                        return Err(Error::new(error.kind(), error.to_string()));
                    },
                }
            },
            //path not found
            Err(error) => {
                return Err(Error::new(error.kind(), error.to_string()));
            },
        };
    }

    pub fn get_home_directory(&self) -> PathBuf{
        return self.user_home_directory.lock().unwrap().to_path_buf()
    }

    pub fn get_instance_grammar(&self) -> Grammar{
        return self.grammar.lock().unwrap().clone();
    }

    pub fn get_current_directory_formatted(&self) -> String{
        return self.current_directory.lock().unwrap().display().to_string().replace(r"\\", r"\").replace(r"\?\", r"")
    }

    pub fn get_current_directory(&self) -> PathBuf{
        return self.current_directory.lock().unwrap().to_path_buf()
    }
}