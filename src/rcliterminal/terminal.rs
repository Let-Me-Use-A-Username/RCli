
use std::{env, io};
use std::io::Write;

use super::super::rcliparser::parser::match_parse;

pub fn start_terminal(){

    'terminal: loop  {
        println!("============RCLI TERMINAL============");
        println!("=====================================");
        let mut input = String::new();

        //display cwd
        print!("{}>", env::current_dir().unwrap().display().to_string());
        io::stdout().flush().unwrap();

        let user_input = std::io::stdin().read_line(&mut input);

        //accept input
        match user_input {
            Ok(_) => {
                println!("{}", input);
                match_parse(input);
            },
            Err(input_error) => todo!(),
        }

        

        println!("=====================================");
    }
}