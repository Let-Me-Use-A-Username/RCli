#[path="../utils/grammar_reader.rs"]
mod grammar_reader;
use grammar_reader::Command;

pub fn analyze(){
    let commands : Vec<Command> = grammar_reader::load_grammar();
    for command in commands{
        println!("{:?}", command);
    }
}

fn validate(){
    
}