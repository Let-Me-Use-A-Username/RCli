use std::{collections::VecDeque, io::Error};

use super::objects::user_input::UserInput;

//Accepts user input and vectorizes
pub fn accept_input(input: String) -> Result<UserInput, Error>{
    let input_parts: Vec<&str> = input.split(' ').collect();

    let size = input_parts.len();

    if size < 1 {
        return Err(Error::new(std::io::ErrorKind::InvalidInput, "Input reader error: No arguments provided."));
    }

    let mut string_parts: VecDeque<String> = VecDeque::new();

    // for part in input_parts.clone(){
    //     string_parts.push_back(part.trim().to_string())
    // }
    let mut object_with_quotes = Vec::<&str>::new();
    for part in input_parts{
        if part.starts_with('"'){
            object_with_quotes.push(part)
        }
        else if part.contains('"'){
            object_with_quotes.push(" ");
            object_with_quotes.push(part);
            
            let object_w_quote = object_with_quotes.concat().replace("\"", "").replace("\r\n", "");
            string_parts.push_back(object_w_quote);
        }
        else{
            string_parts.push_back(part.trim().to_string());
        }   
    }

    let main: String = string_parts[0].to_lowercase();


    return Ok(UserInput {
        vector_input:string_parts, 
        vector_length:size, 
        core_command:main, 
        peek_index:0, 
        consume_index:0,
        analyzed: false});
}


#[cfg(test)]
mod tests {
    use crate::rcliparser::objects::user_input::{Consumable, Peekable};

    use super::*;

    #[test]
    fn test_input(){
        let input = accept_input("create readme.txt -d path".to_string()).ok().unwrap();
        assert_eq!(input.vector_input[0], "create");
        assert_eq!(input.vector_input[1], "readme.txt");
        assert_eq!(input.vector_input[2], "-d");
        assert_eq!(input.vector_input[3], "path");
        assert_eq!(input.vector_length, 4);
        assert_eq!(input.core_command, "create");
    }


    #[test]
    fn test_peek(){
        let mut input = accept_input("create ./Desktop/Some/Dir".to_string()).ok().unwrap();
        assert_eq!(input.peek(0), Some("create".to_string()));
        assert_eq!(input.peek(1), Some("./Desktop/Some/Dir".to_string()));
        assert_eq!(input.peek(2), None);
    }

    #[test]
    fn test_peek_next(){
        let mut input = accept_input("list Desktop/Some/Dir --hidden".to_string()).ok().unwrap();
        assert_eq!(input.peek_next(), Some("list".to_string()));
        assert_eq!(input.peek_index, 1);
        assert_eq!(input.peek_next(), Some("Desktop/Some/Dir".to_string()));
        assert_eq!(input.peek_index, 2);
        assert_eq!(input.peek_next(), Some("--hidden".to_string()));
        assert_eq!(input.peek_index, 3);
        assert_eq!(input.peek_next(), None);
        assert_eq!(input.peek_index, 3);
    }

    #[test]
    fn test_consume(){
        let mut input = accept_input("list --hidden".to_string()).ok().unwrap();
        assert_eq!(input.consume(), Some("list".to_string()));
        assert_eq!(input.consume_index, 0);
        assert_ne!(input.consume_index, input.vector_length);
        assert_eq!(input.consume(), Some("--hidden".to_string()));
        assert_eq!(input.consume_index, 0);
        assert_eq!(input.consume_index, input.vector_length);
        assert_eq!(input.consume(), None);
        assert_eq!(input.consume_index, input.vector_length);
        assert_eq!(input.consume_index, 0);
    }
}

