use std::collections::VecDeque;

use crate::structures::token::Token;
use crate::structures::look_tables::TokenTable;

///First stage preprocessing. Parse string into tokens.
pub fn tokenize(input_string: String) -> VecDeque<Token>{
    let table = TokenTable::new();

    let mut word = Vec::<char>::new();
    let mut tokens = VecDeque::<Token>::new();
    let mut char_iter = input_string.chars();

    'tokenize: loop{
        //Iterate characters
        match char_iter.next(){
            Some(character) => {
                //Match character against Token Lookup table
                match table.get_token(character){
                    //Token found
                    Some(token) => {
                        let last_token = tokens.back();
                        //Every time the last token is checked to see if it starts an expansion
                        if tokens.back().is_some_and(|x| {
                            match x {
                                Token::Dash => return true,
                                Token::Ambersant => return true,
                                Token::Pipe => return true,
                                Token::Larrow => return true,
                                Token::Rarrow => return true,
                                _ => return false,
                            }
                            })
                                {
                                //if an expansions is returned, the last item is popped and then new inserted
                                match table.get_expansion(last_token.unwrap().clone(), token.clone()){
                                    Some(exp) => {
                                        tokens.pop_back();
                                        tokens.push_back(exp);
                                    },
                                    //else the token is pushed normally
                                    None => {
                                        tokens.push_back(token)
                                    },
                                }
                                continue;
                        }
                        // FIXME: Possible problem where word isn't inserted after expansion.
                        //If word isnt empty push it as a word
                        if !word.is_empty(){
                            tokens.push_back(Token::Word(word.iter().collect()));
                            word.clear();
                        }
                        //Push token if expansion doesn't exist
                        tokens.push_back(token);
  
                    },
                    //Non token found. Alphanumeric or special character not reserved found
                    None => {
                        word.push(character);
                        continue;
                    }
                }
            },
            None => break 'tokenize,
        }
    }

    return tokens;
}

//Second stage preprocessing. Resolves Words into more specific type.
pub fn resolve_words(tokens: VecDeque<Token>){
    let mut output: VecDeque<Token> = VecDeque::new();
    let mut token_iter = tokens.iter();

    'word_parse: loop{
        //Parse tokens
        match token_iter.next(){
            Some(token) => {
                //Get next token
                let next_item = token_iter.next();
                let mut word_context: String = String::new();
                let mut is_regex = false;

                if next_item.is_some_and(|x| match x {
                    //If next token is Something and also a Word()
                    Token::Word(word) => {
                        word_context = word.to_string();
                        //If word contains regex flag it
                        if contains_regex(&word_context) {
                            is_regex = true;
                        }
                        return true
                    },
                    _ => return false,
                }){
                    //and current word is an expansion form
                    match token{
                        Token::DollarSign => output.push_back(Token::Variable(word_context)),
                        Token::Dash => output.push_back(Token::Shortflag(word_context)),
                        Token::DoubleDash => output.push_back(Token::Longflag(word_context)),
                        Token::Lbrace => {
                            let brace_expansion = token_iter.clone().take_while(|x| {
                                x.ne(&&Token::Rbrace)
                            });

                            let mut brace_statement = String::from(word_context);
                            //Extract words from brace
                            brace_expansion.for_each(|x| {
                                match x{
                                    Token::Word(word) => {
                                        brace_statement.push_str(word);
                                    },
                                    _ => todo!("Return error")
                                }
                            });

                            output.push_back(Token::Brace(brace_statement));
                        },
                        _ => {
                            //found word but not expantion
                            output.push_back(token.to_owned());

                            match is_regex{
                                true => {
                                    output.push_back(Token::Regex(word_context));
                                },
                                false => {
                                    output.push_back(Token::Word(word_context));
                                }
                            }

                        }
                    }
                }

            },
            None => break 'word_parse,
        }
    }
}

pub fn contains_regex(word: &String) -> bool{
    let regex_chars = vec!['?', '*', '+', '-'];
    let mut contains = false;

    word.chars().for_each(|x| {
        if regex_chars.contains(&x){
            contains = true
        }
    });

    return contains
}