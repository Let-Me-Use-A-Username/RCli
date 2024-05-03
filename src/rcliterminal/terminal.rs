use std::mem::discriminant;
use std::{env, io};
use std::io::Write;

use crate::rcliparser::parser;
use crate::rcliparser::utils::grammar_reader;
use crate::rcliterminal::terminal_singlenton;
use crate::rcliterminal::terminal_singlenton::Terminal;

pub fn start_terminal(){
    //load grammar
    let grammar = grammar_reader::load_grammar();
    //load syntax
    let syntax = grammar_reader::load_command_syntax();
    //load singlenton
    let instance: &mut Terminal = terminal_singlenton::singlenton(grammar, syntax);

    //set the current directory in case the core command is on local dir and full path isnt specified
    instance.set_current_directory(env::current_dir().unwrap());

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
                parser::match_parse(input, instance);
            },
            Err(input_error) => {
                //todo! handle error
                eprintln!("INPUT ERROR {}", input_error);
                break 'terminal;
            },
        }
    }
}