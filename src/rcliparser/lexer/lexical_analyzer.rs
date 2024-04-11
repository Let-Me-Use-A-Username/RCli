#[path="../utils/grammar_reader.rs"]
mod grammar_reader;
use grammar_reader::Command;
use grammar_reader::CommandType;

#[path="../inputreader/input_reader.rs"]
mod input_reader;
use crate::input_reader::UserInput;
use crate::input_reader::Peekable;
use crate::input_reader::Consumable;


pub fn analyze(input: &mut UserInput){
    let commands : Vec<Command> = grammar_reader::load_grammar();
}

fn validate(input: &mut UserInput, command: Command) -> Result<bool, bool>{
    let part = input.peek_next().unwrap();
    match command.command_type{
        CommandType::Core=> {
            if command.command.iter().any(|com| com == &input.core_command){
                let res = input.consume().unwrap();
                
            }
            return Ok(true);
        },
        CommandType::Sub=> {
            return Ok(true);
        },
        CommandType::Object=> {
            return Ok(true);
        },
        CommandType::Flag=> {
            return Ok(true);
        },
        _=> {
            return Err(false);
        }
    }
}