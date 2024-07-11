use std::collections::HashMap;

use crate::token::Token;



pub struct LookupTable{
    reserved_characters: HashMap<char, Token>
}

impl LookupTable{
    pub fn new() -> Self{
        LookupTable { reserved_characters: HashMap::from_iter([
            ('\'', Token::Single),
            ('\"', Token::Double),
            ('-', Token::Dash),
            ('<', Token::Larrow),
            ('>', Token::Rarrow),
            ('|', Token::Pipe),
            ('!', Token::Not),
            ('&', Token::Ambersant),
            (';', Token::GreekQues),
            ('{', Token::Lbrace),
            ('}', Token::Rbrace),
            ('(', Token::Lpar),
            (')', Token::Rpar),
            (' ', Token::Whitespace)
        ].into_iter()) }
    }

    pub fn is_reserved(&self, character: char) -> bool{
        return self.reserved_characters.contains_key(&character)
    }

    pub fn get_token(&self, character: char) -> Option<Token>{
        return self.reserved_characters.get(&character).cloned()
    }
}