pub mod utils {
    pub mod grammar_reader;
    pub mod bsftree;
    pub mod windows_file_attributes;
}

pub mod objects{
    pub mod user_input;
    pub mod data_types;
    pub mod grammar_objects;
    pub mod token_objects;
}

mod input_reader;
mod lexical_analyzer;
pub mod parser;
mod invoker;
