use crate::token::Token;
use crate::look_tables::LookupTable;

pub fn tokenize(input_string: String){
    let table = LookupTable::new();
    let mut first_term = true;
    let mut word = Vec::<char>::new();
    let mut tokens = Vec::<Token>::new();
    let mut char_iter = input_string.chars();

    'tokenize: loop{
        //Iterate characters
        match char_iter.next(){
            Some(character) => {
                //Match character
                match table.get_token(character){
                    //Reserved character
                    Some(token) => {
                    },
                    //Alphanumeric
                    None => {
                        word.push(character);
                        continue;
                    }
                }
            },
            None => break 'tokenize,
        }
    }
}