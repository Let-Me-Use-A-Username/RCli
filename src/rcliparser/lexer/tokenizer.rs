use std::collections::VecDeque;
use std::io::Error;

use crate::structures::look_tables::TokenTable;
use crate::structures::token::Token;

/*
    Lexer module.
    The lexical analysis comprises of a two stage preprocessing.

    The first stage resolves 1) characters into Tokens, and 2) Token combinations into more complex Tokens.
    This is done by checking the last item of the array and removing it if it forms an expansion.

    The second stage resolves Word Tokens into a more specific Token type.
    This is done by checking two Tokens at a time. If the second Token is a Word and the first is a Word
    expansion form then we append the expanded Token.

    Time complexity:
    tokenize:
        loop N length array : O(N)
        check against lookup table: O(1)
            check if last token is an expansion: O(n/5) + O(1) VecDeque = O(n/5)
    = O(N) + O(n/5) = O(N)

    resolve_words:
        loop N length token array: O(M)
        check if token is an expansion: O(n/5) 
    = O(M) + O(n/5) = O(M)

    Total complexity = O(N) + O(M) where N character vector length and M token vector length

*/

///First stage preprocessing. Parse string into tokens.
pub fn tokenize(input_string: String) -> Result<VecDeque<Token>, Error> {
    let table = TokenTable::new();

    let mut word = Vec::<char>::new();
    let mut tokens = VecDeque::<Token>::new();
    let mut char_iter = input_string.chars();

    'tokenize: loop {
        //Iterate characters
        match char_iter.next() {
            Some(character) => {
                //Match character against Token Lookup tablexx
                match table.get_token(character) {
                    //Token found
                    Some(token) => {
                        let last_token = tokens.back();
                        //Every time the last token is checked to see if it starts an expansion
                        if last_token.is_some_and(|x| match x {
                            Token::Dash => return true,
                            Token::Ambersant => return true,
                            Token::Pipe => return true,
                            Token::Larrow => return true,
                            Token::Rarrow => return true,
                            _ => return false,
                        }) {
                            //if an expansions is returned, the last item is popped and then new inserted
                            match table.get_expansion(last_token.unwrap().clone(), token.clone()) {
                                Some(exp) => {
                                    tokens.pop_back();
                                    tokens.push_back(exp);
                                }
                                //else the token is pushed normally
                                None => {
                                    if !word.is_empty() {
                                        tokens.push_back(Token::Word(word.iter().collect()));
                                        word.clear();
                                    }
                                    tokens.push_back(token.clone())
                                }
                            }
                            continue;
                        }
                        //If word isnt empty push it as a word
                        if !word.is_empty() {
                            tokens.push_back(Token::Word(word.iter().collect()));
                            word.clear();
                        }
                        //Push token if expansion didn't occur
                        tokens.push_back(token);
                    }
                    //Non token found. Alphanumeric or special character not reserved found
                    None => {
                        word.push(character);
                        continue;
                    }
                }
            }
            None => {
                // Remove trailing new line
                if !word.is_empty() {
                    let inserted_word = word.iter().filter(|x| x.ne(&&'\r') && x.ne(&&'\n'));
                    tokens.push_back(Token::Word(inserted_word.collect()));
                }
                break 'tokenize;
            }
        }
    }

    return Ok(resolve_words(tokens));
}

//Second stage preprocessing. Resolves Words into more specific type.
pub fn resolve_words(tokens: VecDeque<Token>) -> VecDeque<Token> {
    let mut output: VecDeque<Token> = VecDeque::new();
    let mut token_iter = tokens.iter().peekable();

    'word_parse: loop {
        //Parse tokens
        match token_iter.next() {
            Some(token) => {
                //Get next token
                let next_item = token_iter.peek();
                let mut word_context: String = String::new();
                let mut is_regex = false;

                //If next token is a Word()
                if next_item.is_some_and(|x| match x {
                    Token::Word(word) => {
                        word_context = word.to_string();
                        //If word contains regex flag it
                        if contains_regex(&word_context) {
                            is_regex = true;
                        }
                        return true;
                    }
                    _ => return false,
                }) {
                    //Current word is an expansion form
                    //Advance iter to remove the next token that will be consumed in the expansion
                    token_iter.next();

                    match token {
                        //Push variable
                        Token::DollarSign => output.push_back(Token::Variable(word_context)),
                        //Push shortflag
                        Token::Dash => output.push_back(Token::Shortflag(word_context)),
                        //Push longflag
                        Token::DoubleDash => output.push_back(Token::Longflag(word_context)),
                        //Push brace expression
                        Token::Double => {
                            let quote_expansion =
                                token_iter.clone().take_while(|x| x.ne(&&Token::Double));

                            let mut quote_statement = String::from(word_context);
                            //Extract words from brace
                            quote_expansion.for_each(|x| {
                                //Advance token iterator to consume the brace statement tokens
                                token_iter.next();
                                match x {
                                    Token::Word(word) => {
                                        quote_statement.push_str(word);
                                    }
                                    Token::Whitespace => {
                                        quote_statement.push_str(" ");
                                    }
                                    _ => (),
                                }
                            });

                            output.push_back(Token::Quote(quote_statement));
                            //consume Rbrace
                            token_iter.next();
                        }
                        //Push brace expression
                        Token::Lbrace => {
                            let brace_expansion =
                                token_iter.clone().take_while(|x| x.ne(&&Token::Rbrace));

                            let mut brace_statement = String::from(word_context);
                            //Extract words from brace
                            brace_expansion.for_each(|x| {
                                //Advance token iterator to consume the brace statement tokens
                                token_iter.next();
                                match x {
                                    Token::Word(word) => {
                                        brace_statement.push_str(word);
                                    }
                                    Token::Whitespace => {
                                        brace_statement.push_str(" ");
                                    }
                                    _ => (),
                                }
                            });

                            output.push_back(Token::Brace(brace_statement));
                            //consume Rbrace
                            token_iter.next();
                        }
                        _ => {
                            //found word but not expansion
                            output.push_back(token.to_owned());

                            match is_regex {
                                true => {
                                    output.push_back(Token::Regex(word_context));
                                }
                                false => {
                                    output.push_back(Token::Word(word_context));
                                }
                            }
                        }
                    }
                }
                //else next item isn't a word. So push items normally
                else {
                    output.push_back(token.clone());
                }
            }
            None => break 'word_parse,
        }
    }

    return output;
}

pub fn contains_regex(word: &String) -> bool {
    let regex_chars = vec!['?', '*', '+', '-'];
    let mut contains = false;

    word.chars().for_each(|x| {
        if regex_chars.contains(&x) {
            contains = true
        }
    });

    return contains;
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_general_tokenization() {
        let input =
            String::from("ls Desktop -R --any | grep *.txt -l -m > txtfiles || find txtfiles -R");
        let output = tokenize(input);
        let matched: Vec<Token> = vec![
            Token::Word("ls".to_string()),
            Token::Whitespace,
            Token::Word("Desktop".to_string()),
            Token::Whitespace,
            Token::Shortflag("R".to_string()),
            Token::Whitespace,
            Token::Longflag("any".to_string()),
            Token::Whitespace,
            Token::Pipe,
            Token::Whitespace,
            Token::Word("grep".to_string()),
            Token::Whitespace,
            Token::Regex("*.txt".to_string()),
            Token::Whitespace,
            Token::Shortflag("l".to_string()),
            Token::Whitespace,
            Token::Shortflag("m".to_string()),
            Token::Whitespace,
            Token::Rarrow,
            Token::Whitespace,
            Token::Word("txtfiles".to_string()),
            Token::Whitespace,
            Token::Or,
            Token::Whitespace,
            Token::Word("find".to_string()),
            Token::Whitespace,
            Token::Word("txtfiles".to_string()),
            Token::Whitespace,
            Token::Shortflag("R".to_string()),
        ];
        assert_eq!(output.unwrap(), matched)
    }

    #[test]
    fn test_tokenize_multiple_flags() {
        let input = String::from("ls Desktop -l -m -R -n --any");
        let output = tokenize(input);
        let matched: Vec<Token> = vec![
            Token::Word("ls".to_string()),
            Token::Whitespace,
            Token::Word("Desktop".to_string()),
            Token::Whitespace,
            Token::Shortflag("l".to_string()),
            Token::Whitespace,
            Token::Shortflag("m".to_string()),
            Token::Whitespace,
            Token::Shortflag("R".to_string()),
            Token::Whitespace,
            Token::Shortflag("n".to_string()),
            Token::Whitespace,
            Token::Longflag("any".to_string()),
        ];
        assert_eq!(output.unwrap(), matched)
    }

    #[test]
    fn test_tokenize_brace() {
        let input = String::from("ls C:/{Desktop  Videos  Pictures}");
        let output = tokenize(input);
        let matched: Vec<Token> = vec![
            Token::Word("ls".to_string()),
            Token::Whitespace,
            Token::Word("C:/".to_string()),
            Token::Brace("Desktop  Videos  Pictures".to_string()),
        ];
        assert_eq!(output.unwrap(), matched)
    }

    #[test]
    fn test_tokenize_false_brace_expression() {
        let input = String::from("ls C:/{Desktop  Videos  Pictures");
        let output = tokenize(input);
        let matched: Vec<Token> = vec![
            Token::Word("ls".to_string()),
            Token::Whitespace,
            Token::Word("C:/".to_string()),
            Token::Brace("Desktop  Videos  Pictures".to_string()),
        ];
        assert_eq!(output.unwrap(), matched)
    }

    
    #[test]
    fn test_tokenize_variable() {
        let input = String::from("ls $HOME/Desktop");
        let output = tokenize(input);
        let matched: Vec<Token> = vec![
            Token::Word("ls".to_string()),
            Token::Whitespace,
            Token::Variable("HOME/Desktop".to_string())
        ];
        assert_eq!(output.unwrap(), matched)
    }

    #[test]
    fn test_tokenize_new_line() {
        let input = String::from("\r\n");
        let output = tokenize(input);
        let matched: Vec<Token> = vec![
            Token::Word("".to_string()),
        ];
        assert_eq!(output.unwrap(), matched)
    }

    #[test]
    fn test_tokenize_quotes() {
        let input = String::from("echo \"a simple string\"");
        let output = tokenize(input);
        let matched: Vec<Token> = vec![
            Token::Word("echo".to_string()),
            Token::Whitespace, 
            Token::Quote("a simple string".to_string())
        ];
        assert_eq!(output.unwrap(), matched)
    }
}
