pub mod utils {
    pub mod grammar_reader;
    pub mod bsftree;
    pub mod dotparser;
    pub mod windows_file_attributes;
}

pub mod objects{
    pub mod tokens;
    pub mod user_input;
    pub mod bnf_commands;
    pub mod data_types;
}

mod input_reader;
mod lexical_analyzer;
pub mod parser;
mod invoker;
