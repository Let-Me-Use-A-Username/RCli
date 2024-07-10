

pub enum Token{
    //Parameters
    Dash,           //-
    DoubleDash,     //--
    //Redirections
    Larrow,         //<
    Rarrow,         //>
    //OPSeparators
    Ambersant,      //&
    GreekQues,      //;
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
    Shortflag(char),    // -<char>
    Longflag(String),   // --<word>
    Regex(String),      //Any word containing *, ?, ^, ., (+, -, in certain cases)
}