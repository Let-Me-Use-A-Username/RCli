use std::{fmt::Display, io::{self, Write}, sync::Mutex};


pub struct Logger{
    stdout: Mutex<io::Stdout>,
    stderr: Mutex<io::Stderr>
}
impl Logger{
    pub fn new() -> Self{
        Logger { 
            stdout: Mutex::new(io::stdout()),
            stderr: Mutex::new(io::stderr())
        }
    }

    ///Standard stdout log.
    pub fn log<T: Display>(&self, message: T){
        let mut stdout = self.stdout.lock().unwrap();
        writeln!(stdout, "{}", message).unwrap();
        stdout.flush().unwrap();
    }

    ///Standard stdout log without new line.
    pub fn lognn<T: Display>(&self, message: T){
        let mut stdout = self.stdout.lock().unwrap();
        write!(stdout, "{}", message).unwrap();
        stdout.flush().unwrap();
    }

    ///Specific function to de-canoncalize windows paths.
    pub fn format_log(&self, message: String){
        let mut stdout = self.stdout.lock().unwrap();
        writeln!(stdout, "{}", message.replace(r"\\", r"\").replace(r"\?\", r"")).unwrap();
        stdout.flush().unwrap();
    }

    ///Standard stderr log.
    pub fn log_err<T: Display>(&self, message: T){
        let mut stderr = self.stderr.lock().unwrap();
        writeln!(stderr, "{}", message).unwrap();
        stderr.flush().unwrap();
    }
}
