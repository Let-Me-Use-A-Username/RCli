use std::io::stdin;

mod structures;
mod rcliparser;

fn main() {
    let mut input = String::new();
    let _ = stdin().read_line(&mut input);
    rcliparser::lexer::tokenizer::tokenize(input);
}
