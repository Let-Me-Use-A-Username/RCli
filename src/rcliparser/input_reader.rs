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

    let mut object_with_quotes = Vec::<&str>::new();
    let mut quote_object_found = false;

    //For string in input 
    for part in input_parts{
        //if input part has double quote append to vector
        if part.starts_with('"') | part.starts_with('\"'){
            object_with_quotes.push(part);
            quote_object_found = true;
            continue;
        }

        //while we haven't found the end part of the quote keep adding
        if quote_object_found{
            object_with_quotes.push(" ");
            object_with_quotes.push(part);
            
            if part.contains('"') | part.contains('\"'){
                let final_object = object_with_quotes.concat().replace("\"", "").replace("\r\n", "");
                string_parts.push_back(final_object);
                //when found stop adding
                quote_object_found = false;
                object_with_quotes.clear();
            }
            
        }
        else{
            string_parts.push_back(part.trim().to_string());
        }
    }

    return Ok(UserInput {
        vector_input:string_parts, 
        vector_length:size, 
        peek_index:0, 
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
        assert_eq!(input.consume(), Some("--hidden".to_string()));
        assert_eq!(input.consume(), None);
    }
}

