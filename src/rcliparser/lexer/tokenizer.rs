use std::collections::VecDeque;

use crate::structures::token::Token;
use crate::structures::look_tables::LookupTable;

///First stage tokenization. Parse string into tokens.
pub fn tokenize(input_string: String) -> VecDeque<Token>{
    let table = LookupTable::new();

    let mut word = Vec::<char>::new();
    let mut tokens = VecDeque::<Token>::new();
    let mut char_iter = input_string.chars();

    'first_stage: loop{
        //Iterate characters
        match char_iter.next(){
            Some(character) => {
                //Match character
                match table.get_token(character){
                    //Reserved character
                    Some(token) => {
                        if !word.is_empty(){
                            tokens.push_back(Token::Word(word.iter().collect()));
                            word.clear();
                        }
                        tokens.push_back(token);
                    },
                    //Alphanumeric
                    None => {
                        word.push(character);
                        continue;
                    }
                }
            },
            None => break 'first_stage,
        }
    }

    return create_intermediate_stream(tokens);
}

///Second stage tokenization. Parse multi-tokens as tokens
pub fn create_intermediate_stream(token_stream: VecDeque<Token>) -> VecDeque<Token>{
    let mut intermediate_tokens = VecDeque::<Token>::new();
    let mut token_iterator = token_stream.iter().peekable();

    'stream: loop{
        let token_pair = (token_iterator.next(), token_iterator.peek().cloned());
        
        match token_pair{
            (Some(token), None) => {
                intermediate_tokens.push_back(token.to_owned())
            },
            (Some(l_token), Some(r_token)) => {
                match (l_token, r_token){
                    (Token::Dash, Token::Dash) => {
                        intermediate_tokens.push_back(Token::DoubleDash);
                    },
                    (Token::Rarrow, Token::Rarrow) => {
                        intermediate_tokens.push_back(Token::RRarrow);
                    },
                    (Token::Larrow, Token::Rarrow) => {
                        intermediate_tokens.push_back(Token::LRarrow);
                    },
                    (Token::Ambersant, Token::Ambersant) => {
                        intermediate_tokens.push_back(Token::And);
                    },
                    (Token::Pipe, Token::Pipe) => {
                        intermediate_tokens.push_back(Token::Or);
                    },
                    _ => {
                        intermediate_tokens.push_back(l_token.to_owned());
                        intermediate_tokens.push_back(r_token.to_owned());
                    }
                }
            },
            _ => {
                break 'stream
            }
        }
    }

    return intermediate_tokens;
}