#[path="../utils/grammar_reader.rs"]
mod grammar_reader;
use grammar_reader::Command;

#[path="..//inputreader/input_reader.rs"]
mod input_reader;
use crate::input_reader::UserInput;
// use crate::input_reader::Peekable;
// use crate::input_reader::Consumable;



pub fn analyze(input: UserInput){
    let commands : Vec<Command> = grammar_reader::load_grammar();
    
    //let core: Command = commands.get(0).unwrap();
    for command in commands{
        println!("{:?}", command);
    }
}

fn validate(){
    
}