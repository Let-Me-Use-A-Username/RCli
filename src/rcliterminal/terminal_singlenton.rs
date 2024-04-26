use uuid::Uuid;
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
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
        *current_dir = path.clone().canonicalize().unwrap();
        let res = env::set_current_dir(path);

        match res {
            Ok(_) => return,
            Err(_) => todo!(),
        }
    }

    pub fn get_current_directory_to_string(&self) -> String{
        return self.current_directory.lock().unwrap().display().to_string()
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