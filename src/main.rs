use std::io::stdin;

mod token;
mod tokenizer;
mod look_tables;

fn main() {
    let mut input = String::new();
    let _ = stdin().read_line(&mut input);
    tokenizer::tokenize(input);
}
