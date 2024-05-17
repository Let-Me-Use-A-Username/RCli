use std::{env, io};
use std::io::Write;

use crate::rcliparser::parser;
use crate::rcliparser::utils::grammar_reader;
use crate::rcliterminal::terminal_singlenton;
use crate::rcliterminal::terminal_singlenton::Terminal;

pub fn start_terminal(){
    //load grammar
    let grammar = grammar_reader::load_grammar();
    //current dir
    let current_dir = env::current_dir().unwrap();
    //get home dir
    //let home_dir = dirs::home_dir().unwrap_or(current_dir);
    let home_dir = dirs::home_dir().unwrap();
    //load singlenton
    let instance: &mut Terminal = terminal_singlenton::singlenton(home_dir.clone(), grammar);

    //set the current directory in case the core command is on local dir and full path isnt specified
    let _ = instance.set_current_directory(home_dir);

    //singlenton loop
    'terminal: loop  {
        println!("============RCLI TERMINAL============\n");
        let mut input = String::new();

        //display cwd
        //todo! this replace in dir_disply might cause problems
        let dir_display = instance.get_current_directory_to_string().replace("\\\\?\\", "");
        print!("RCli {}>", dir_display);
        io::stdout().flush().unwrap();

        let user_input = std::io::stdin().read_line(&mut input);

        //accept input
        match user_input {
            Ok(_) => {
                parser::parse(input, instance);
            },
            Err(input_error) => {
                //todo! handle error
                eprintln!("INPUT ERROR {}", input_error);
                break 'terminal;
            },
        }
    }
}