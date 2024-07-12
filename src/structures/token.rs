use std::fmt::Display;


#[derive(Clone)]
pub enum Token{
    //Variables
    DollarSign,     //$
    //Quotes
    Single,         //'
    Double,         //"
    //Parameters
    Dash,           //-
    DoubleDash,     //--
    //Redirections
    Larrow,         //<
    Rarrow,         //>
    RRarrow,        //>>
    LRarrow,        //<>
    //Pipe
    Pipe,           // |
    //Logical Operators
    And,            //&&
    Or,             //||
    Not,            // !
    //OPSeparators
    Ambersant,      //&
    GreekQues,      //;
    //Separators
    Whitespace,
    //Expansions
    Lbrace,         //{
    Rbrace,         //}
    //Groups
    Lpar,           //(
    Rpar,           //)
    //Words
    Word(String),   //Any word
    Command(String),    //First word of every command object. See RULE 1.
    Variable(String),   //Any word like $<word>
    Shortflag(String),    // -<char>
    Longflag(String),   // --<word>
    Regex(String),      //Any word containing *, ?, ^, ., (+, -, in certain cases)
    Brace(String)       //Any word starting and ending with {word, word}
}

impl Token{
    pub fn get_value(&self) -> Option<impl Display+ '_>{
        match self{
            Token::Word(string) => {return Some(string)},
            Token::Command(string) => {return Some(string)},
            Token::Variable(string) => {return Some(string)},
            Token::Shortflag(string) => {return Some(string)},
            Token::Longflag(string) => {return Some(string)},
            Token::Regex(string) => {return Some(string)},
            Token::Brace(string) => {return Some(string)},
            _ => return None
        }
    }
}