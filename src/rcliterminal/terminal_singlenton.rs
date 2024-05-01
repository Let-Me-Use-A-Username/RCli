use uuid::Uuid;
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use std::sync::{Mutex, Once};
use std::mem::MaybeUninit;

use crate::rcliparser::utils::grammar_reader::Command;
use crate::rcliparser::utils::grammar_reader::CommandType;

//singlenton terminal instance_id
pub struct Terminal{
    instance_id: Mutex<Uuid>,
    current_directory: Mutex<PathBuf>,
    grammar: Mutex<HashMap<CommandType, Command>>
}

/*
Terminal singlenton methods.
At the moment only path changing functions.
*/
impl Terminal{
    pub fn set_current_directory(&mut self, path: PathBuf){
        let mut current_dir = self.current_directory.lock().unwrap();
        
        match path.canonicalize() {
            //check if path is valid
            Ok(new_path) => {
                //set it as current directory
                let operation_results = env::set_current_dir(new_path.clone());
                //todo! handle error in case the operation fails
                match operation_results {
                    Ok(_) => {
                        *current_dir = new_path;
                    },
                    Err(error) => {
                        println!("SINGLENTON SETTER: {:?}", error);
                    },
                }
            },
            //path not found
            Err(error) => {
                println!("SINGLENTON ERROR: Path not found {:?}", error);
            },
        };
    }

    pub fn get_instance_id(&self) -> Uuid{
        return  *self.instance_id.lock().unwrap();
    }

    pub fn get_instance_grammar(&self) -> HashMap<CommandType, Command>{
        return self.grammar.lock().unwrap().clone();
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
                instance_id: Mutex::new(Uuid::new_v4()),
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