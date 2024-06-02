use std::process::ExitCode;
use std::{env, io};
use std::io::Write;

use crate::rcliparser::parser;
use crate::rcliparser::utils::grammar_reader;
use crate::rcliparser::objects::data_types::Data;
use crate::rcliterminal::terminal_singlenton;
use crate::rcliterminal::terminal_singlenton::Terminal;

pub fn start_terminal() -> ExitCode{
    //load grammar
    let grammar = grammar_reader::load_grammar();
    //current dir
    let current_dir = env::current_dir().unwrap();
    //get home dir
    let home_dir = dirs::home_dir().unwrap_or(current_dir);
    //load singlenton
    let instance: &mut Terminal = terminal_singlenton::singlenton(home_dir.clone(), grammar);

    //set the current directory in case the core command is on local dir and full path isnt specified
    let _ = instance.set_current_directory(home_dir);

    //singlenton loop
    'terminal: loop  {
        println!("============RCLI TERMINAL============\n");
        let mut input = String::new();

        let dir_display = instance.get_current_directory_to_string().replace("\\\\?\\", "");
        print!("RCli {}>", dir_display);
        io::stdout().flush().unwrap();

        let user_input = std::io::stdin().read_line(&mut input);

        //accept input
        match user_input {
            Ok(_) => {
                let operation_result = parser::parse(input, instance);

                match operation_result{
                    Ok(data) => {
                        match data {
                            Data::PathData(path) => {
                                println!("Path: {}", path.display().to_string());
                            },
                            Data::StringData(data) => {
                                println!("{data}");
                            },
                            Data::VecStringData(string_vec) => {
                                string_vec.iter().for_each(|x| println!("{x}"));
                            },
                            Data::DirPathData(path_data) => {
                                path_data.iter().for_each(|x| println!("{:?}", x.display().to_string()))
                            },
                            Data::StatusData(status_code) => {
                                if status_code.eq(&1){
                                    return ExitCode::SUCCESS;
                                }
                            },
                            Data::DataVector(boxed_data) => {
                                let data = *boxed_data;
                                
                                data.iter().for_each(|x| println!("{:?}", x.get_value()));
                            }
                            _ => unreachable!()
                        }
                    },
                    Err(err) => {
                        println!("Error:{}", err.to_string());
                    },
                }
            },
            Err(input_error) => {
                eprintln!("Error: {}", input_error.to_string());
                break 'terminal;
            },
        }
    }
    return ExitCode::FAILURE;
}