use uuid::Uuid;
use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, Once};
use std::mem::MaybeUninit;

use crate::rcliparser::utils::grammar_reader::Command;
use crate::rcliparser::utils::grammar_reader::CommandType;

//singlenton terminal instance
pub struct Terminal{
    pub(crate) instance: Mutex<Uuid>,
    pub(crate) current_directory: Mutex<PathBuf>,
    pub(crate) grammar: Mutex<HashMap<CommandType, Command>>
}

/*
Terminal methods.
At the moment only path changing functions.
*/
impl Terminal{
    pub fn change_current_directory(&mut self, path: PathBuf){
        let mut current_dir = self.current_directory.lock().unwrap();
        
        let path_exists = match path.canonicalize() {
            //check if path is valid
            Ok(res) => {
                *current_dir = res.clone();
                //set it as current directory
                let operation_results = env::set_current_dir(res);
                //handle error in case the operation fails
                match operation_results {
                    Ok(_) => return,
                    Err(_) => todo!(),
                }
            },
            //path not found
            Err(error) => {
                println!("SINGLENTON ERROR: Path not found {:?}", error);
            },
        };
    }

    pub fn get_current_directory_to_string(&self) -> String{
        return self.current_directory.lock().unwrap().display().to_string()
    }

    pub fn get_current_directory(&self) -> PathBuf{
        return self.current_directory.lock().unwrap().to_path_buf()
    }
}


pub fn singlenton(input_grammar: HashMap<CommandType, Command>) -> &'static mut Terminal{
    //create uninitialized static structure
    static mut SINGLENTON: MaybeUninit<Terminal> = MaybeUninit::uninit();
    static ONCE: Once = Once::new();

    //unsafe code goes brrrrrrrr
    unsafe{
        ONCE.call_once(|| {
            let singlenton_instance: Terminal = Terminal{
                instance: Mutex::new(Uuid::new_v4()),
                current_directory: Mutex::new(env::current_dir().unwrap()),
                grammar: Mutex::new(input_grammar)
            };
            SINGLENTON.write(singlenton_instance);
        });

        //mutable reference because it contains the working directory
        //cd is the sole command that manipulates it
        SINGLENTON.assume_init_mut()
    }
}