#[warn(non_snake_case)]

#[path="rcliparser/inputreader/input_reader.rs"]
mod input_reader;
// use crate::input_reader::UserInput;
// use crate::input_reader::Peekable;
// use crate::input_reader::Consumable;

#[path="rcliparser/lexer/lexical_analyzer.rs"]
mod lexical_analyzer;

fn main() {
    println!("HJello");
    lexical_analyzer::analyze();
}
