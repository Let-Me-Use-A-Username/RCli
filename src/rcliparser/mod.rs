pub mod utils {
    pub mod grammar_reader;
    pub mod bsftree;
    pub mod dotparser;
}

pub mod objects{
    pub mod tokens;
    pub mod user_input;
}

mod input_reader;
mod lexical_analyzer;
pub mod parser;
mod invoker;
