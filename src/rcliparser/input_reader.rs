use std::collections::VecDeque;

use super::objects::user_input::UserInput;

//Accepts user input and vectorizes
pub fn accept_input(input: String) -> UserInput{
    let input_parts: Vec<&str> = input.split(' ').collect();

    let size = input_parts.len();

    if size < 1 {
        panic!("ERROR! No arguments provided")
    }

    let mut string_parts: VecDeque<String> = VecDeque::new();

    for part in input_parts.clone(){
        string_parts.push_back(part.trim().to_string())
    }


    let main: String = string_parts[0].to_lowercase();

    let rest: VecDeque<String> = VecDeque::from_iter(string_parts.split_off(1));

    return UserInput {
        vector_input:string_parts, 
        vector_length:size, 
        core_command:main, 
        rest_commands:rest, 
        peek_index:0, 
        consume_index:0,
        analyzed: false};
}


#[cfg(test)]
mod tests {
    use crate::rcliparser::objects::user_input::{Consumable, Peekable};

    use super::*;

    #[test]
    fn test_input(){
        let input = accept_input("create readme.txt -d path".to_string());
        assert_eq!(input.vector_input[0], "create");
        assert_eq!(input.vector_input[1], "readme.txt");
        assert_eq!(input.vector_length, 4);
        assert_eq!(input.core_command, "create");
        assert_eq!(input.rest_commands[0], "readme.txt");
        assert_eq!(input.rest_commands[1], "-d");
        assert_eq!(input.rest_commands[2], "path");
        assert_eq!(input.peek_index, 0);
        assert_eq!(input.consume_index, 0);
    }


    #[test]
    fn test_peek(){
        let mut input = accept_input("create ./Desktop/Some/Dir".to_string());
        assert_eq!(input.peek(0), Some("create".to_string()));
        assert_eq!(input.peek(1), Some("./Desktop/Some/Dir".to_string()));
        assert_eq!(input.peek(2), None);
    }

    #[test]
    fn test_peek_next(){
        let mut input = accept_input("list Desktop/Some/Dir --hidden".to_string());
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
        let mut input = accept_input("list --hidden".to_string());
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

