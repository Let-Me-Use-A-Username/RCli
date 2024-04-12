#[warn(non_snake_case)]

#[path="rcliparser/inputreader/input_reader.rs"]
mod input_reader;

#[path="rcliparser/lexer/lexical_analyzer.rs"]
mod lexical_analyzer;

fn main() {
    println!("HJello");
    let mut input = input_reader::accept_input("create readme.txt");
    lexical_analyzer::analyze(&mut input);

    let mut input2 = input_reader::accept_input("create ./Desktop/Some/Dir");
    lexical_analyzer::analyze(&mut input2);
}
