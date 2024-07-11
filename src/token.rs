
#[derive(Clone)]
pub enum Token{
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
    //Invalid
    Invalid,
    //Words
    Word(String),   //Any word
    Command(String),    //First word of every command object. See RULE 1.
    Variable(String),   //Any word like $<word>
    Shortflag(char),    // -<char>
    Longflag(String),   // --<word>
    Regex(String),      //Any word containing *, ?, ^, ., (+, -, in certain cases)
}