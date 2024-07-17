use crate::rcliparser::lexer::tokenizer;

/* 
    # Note RULE 1: First term is always a reserved or alias command
    # Note RULE 2: A pipe can only exist if both 'AND' and 'OR' aren't included in the pipe sequence.
    # Note RULE 3: If WORD matches a command it is wrapped with quotes.
    # Note RULE 4: REGEX_WORD can not contain whitespace. 
    # Note RULE 5: Brace expansions must be space delimitered.
*/
pub fn parse(input_string: String){
    //Rule 1.
    let first_word = true;
    //Intermediate token stream 
    //Is guaranteed to be Ok()
    let mut token_stream = tokenizer::tokenize(input_string).unwrap();
    let mut token_iter = token_stream.iter();
    
    //Todo : 1)Parse Word to Command.
    //Todo : 2)Parse Token slice as some Command struct.
    //Todo : 2.1) Parse flags
    //Idea 1, pass structs to nodes.
    // Idea : Match Word to CommandType (currently command), collect all parameters and options
    // Idea : then match the CommandType to structs, create a struct and continue

    //Idea 2 create node types and pass to AST
    // Idea : Parser will create nodes that can either be a command (commandtype: CommandType, parameter: Vec<Token>, options: Vec<Flag>)
    // Idea : or an operation. Then the commandtype will be matched at the engine, and the operation will also be handled there.
    'parse: loop{
        match token_iter.next(){
            Some(token) => {

            },
            None => break 'parse,
        }
    }
    //Todo : 3)Create AST.
    //Todo : Expansion happens when.. ?
}