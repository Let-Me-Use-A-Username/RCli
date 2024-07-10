use crate::token::Token;
use crate::look_tables::Lookup_table;

pub fn tokenize(input_string: String){
    let table = Lookup_table::new();

    //First term is COMMAND. Rule 1.
    let mut first_term = false;
    let mut word = Vec::<char>::new();

    let tokens = Vec::<Token>::new();
}