use std::io::Error;

use super::objects::user_input::UserInput;

//Accepts user input and vectorizes
pub fn accept_input(input: String) -> Result<UserInput, Error>{

    let mut words = Vec::<String>::new();
    let mut iterator = input.chars();

    let mut word = Vec::<char>::new();

    let mut quoted_word = Vec::<char>::new();
    let mut found_quotes = false;

    //for char in part
    'chars: loop{
        match iterator.next(){
            Some('\"') | Some('\'')=> {
                found_quotes = !found_quotes;

                quoted_word.push('\"');

                if !found_quotes{
                    let out = quoted_word.iter().collect();
                    quoted_word.clear();

                    words.push(out)
                }
            },
            Some('\r') => {
                continue;
            },
            Some('\n') => {
                let out = word.iter().collect();
                word.clear();

                words.push(out)
            }
            Some(' ') => {
                if found_quotes{
                    quoted_word.push(' ');
                    continue;
                }
                let out = word.iter().collect();
                word.clear();

                words.push(out)
            },
            Some(character) => {
                if found_quotes{
                    quoted_word.push(character);
                    continue;
                }
                word.push(character);
            },
            None => break 'chars,
        }
    }

    let size = words.len();

    return Ok(UserInput {
        vector_input:words.into(), 
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

