use std::{process::ExitCode, sync::{Arc, Mutex}};

use crate::{rclilogger::logger::Logger, rcliparser::{objects::data_types::Data, parser}, rcliterminal::terminal::Terminal};


pub struct Shell{
    terminal: Arc<Mutex<Terminal>>,
    logger: Arc<Mutex<Logger>>
}
impl Shell{
    pub fn new() -> Self{
        Shell { 
            terminal: Arc::new(Mutex::new(Terminal::new())), 
            logger: Arc::new(Mutex::new(Logger::new()))
        }
    }

    pub fn run(&self) -> ExitCode{
        //singlenton loop
        let mut terminal_instance = self.terminal.lock().unwrap();
        let logger = self.logger.lock().unwrap();
        'run: loop  {
            let mut input = String::new();
            let dir_display = terminal_instance.get_current_directory().display().to_string().replace(r"\\", r"\").replace(r"\?\", r"");

            logger.log("============RCLI TERMINAL============\n");
            logger.lognn(String::from(format!("RCli {}>", dir_display)));

            let user_input = std::io::stdin().read_line(&mut input);

            //accept input
            match user_input {
                Ok(_) => {
                    let operation_result = parser::parse(input, &mut *terminal_instance);

                    match operation_result{
                        Ok(data) => {
                            match data {
                                Data::PathData(path) => {
                                    let path_to_string = path.display().to_string();
                                    logger.format_log(path_to_string);
                                },
                                Data::StringData(data) => {
                                    logger.log(data);
                                },
                                Data::VecStringData(string_vec) => {
                                    string_vec.iter().for_each(|x| logger.log(x));
                                },
                                Data::DirPathData(path_data) => {
                                    path_data.iter().for_each(|x| {
                                        let path_to_string = x.display().to_string();
                                        logger.format_log(path_to_string);
                                    })
                                },
                                Data::StatusData(status_code) => {
                                    if status_code.eq(&1){
                                        return ExitCode::SUCCESS;
                                    }
                                },
                                Data::DataVector(boxed_data) => {
                                    let data = *boxed_data;
                                    
                                    data.iter().for_each(|x| logger.format_log(x.get_value().unwrap().to_string()));
                                }
                                _ => unreachable!()
                            }
                        },
                        Err(err) => {
                            logger.log_err(err);
                        },
                    }
                },
                Err(input_error) => {
                    logger.log_err(input_error);
                    break 'run;
                },
            }
        }
        return ExitCode::FAILURE;
    }
}