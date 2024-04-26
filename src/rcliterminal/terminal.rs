use std::{env, io};
use std::io::Write;

use super::super::rcliparser::parser;
use super::super::rcliparser::utils::grammar_reader;
use crate::rcliterminal::terminal_singlenton;
use crate::rcliterminal::terminal_singlenton::Terminal;

pub fn start_terminal(){
    //load grammar
    let grammar = grammar_reader::load_grammar();
    //load singlenton
    let instance: &mut Terminal = terminal_singlenton::singlenton(grammar);

    //singlenton loop
    'terminal: loop  {
        println!("============RCLI TERMINAL============");
        let mut input = String::new();

        //display cwd
        print!("{}>", instance.current_directory.lock().unwrap().display());
        io::stdout().flush().unwrap();

        let user_input = std::io::stdin().read_line(&mut input);

        //accept input
        match user_input {
            Ok(_) => {
                println!("{}", input);
                //
                parser::match_parse(input, instance);
            },
            Err(input_error) => {
                //todo! handle error
                break 'terminal;
            },
        }

        println!("=====================================");
    }
}