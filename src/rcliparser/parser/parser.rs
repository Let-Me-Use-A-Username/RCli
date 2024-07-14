use std::collections::VecDeque;

use crate::{rcliparser::lexer::tokenizer, structures::token::Token};

/* 
    # Note RULE 1: First term is always a reserved or alias command
    # Note RULE 2: A pipe can only exist if both 'AND' and 'OR' aren't included in the pipe sequence.
    # Note RULE 3: If WORD matches a command it is wrapped with quotes.
    # Note RULE 4: REGEX_WORD can not contain whitespace. 
    # Note RULE 5: Brace expansions is the only way to have multiple inputs in a command. I.e. list {dir1  dir2  dir3}
*/
pub fn parse(input_string: String){
    //Rule 1.
    let first_word = true;
    //Intermediate token stream 
    let mut intermediate_stream = tokenizer::tokenize(input_string).into_iter().peekable();
    
    //Todo : 1)Parse Word to Command, Variable etc. 
    //Todo : 2)Parse Token slice as some Command struct.
    //Todo : 2.1) Parse flags
    //Todo : 3)Create AST.
    //Todo : Expansion happens when.. ?

    // Note : Commands will return Status struct. With the status code we wil implemenet && and ||
    // Idea : Command will have a Vec<Flag> (HashSet?) that shows what flags they accept.
    // Idea : Instead of Command structs, create a builder that creates Commands
}