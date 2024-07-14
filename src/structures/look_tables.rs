use std::collections::HashMap;

use crate::structures::token::Token;

use super::commands::command::Command;


/* 
        Token Table
*/

pub struct TokenTable{
    reserved_characters: HashMap<char, Token>,
    token_expansions: HashMap<Vec<Token>, Token>,
}
impl TokenTable{
    pub fn new() -> Self{
        TokenTable { 
            reserved_characters: HashMap::from_iter([
            ('$', Token::DollarSign),
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
        ].into_iter()),

        token_expansions: HashMap::from_iter([
            (vec![Token::Dash, Token::Dash], Token::DoubleDash),
            (vec![Token::Ambersant, Token::Ambersant], Token::And),
            (vec![Token::Pipe, Token::Pipe], Token::Or),
            (vec![Token::Larrow, Token::Rarrow], Token::LRarrow),
            (vec![Token::Rarrow, Token::Rarrow], Token::RRarrow),
        ].into_iter()),

        }
    }

    pub fn get_token(&self, character: char) -> Option<Token>{
        return self.reserved_characters.get(&character).cloned()
    }

    pub fn get_expansion(&self, token_start: Token, token_end: Token) -> Option<Token>{
        let temp_vec: Vec<Token> = vec![token_start, token_end];
        return self.token_expansions.get(&temp_vec).cloned();
    }
}


/* 
        Command Table
*/


pub struct CommandTable{
    reserved_commands: HashMap<&'static str, Command>
}

impl CommandTable{
    pub fn new() -> Self{
        CommandTable { 
            reserved_commands: HashMap::from_iter([
                ("cd", Command::Cd),
                ("pwd", Command::Pwd),
                ("ls", Command::Ls),
                ("mv", Command::Move),
                ("cp", Command::Copy),
                ("rm", Command::Remove),
                ("touch", Command::Touch),
                ("mkdir", Command::MkDir),
                ("find", Command::Find),
                ("zip", Command::Zip),
                ("unzip", Command::Unzip),
                ("echo", Command::Echo),
                ("cat", Command::Read),
                ("grep", Command::Grep),
                ("sed", Command::Sed),
                ("awk", Command::Awk),
                ("sort", Command::Sort),
                ("head", Command::Head),
                ("tail", Command::Tail),
                ("diff", Command::Diff),
                ("export", Command::Export),
                ("alias", Command::Alias),
                ("unalias", Command::Unalias),
                ("shc", Command::Shortcut),
                ("sudo", Command::Sudo),
                ("df", Command::Df),
                ("jobs", Command::Jobs),
                ("kill", Command::Kill),
                ("shutdown", Command::Shutdown),
                ("curl", Command::Curl),
                ("note", Command::Note)
            ].into_iter()) 
        }
        
    }

    pub fn get_command(&self, str_command: String) -> Option<Command>{
        let tuple = self.reserved_commands.get_key_value(&str_command.as_str());
        if tuple.is_some(){
            return Some(tuple.unwrap().1).cloned()
        }
        return None
    }
}