use std::fmt;

pub struct UserInput{
    pub vector_input: Vec<String>,
    vector_length: usize,
    pub core_command: String,
    pub sub_commands: Vec<String>,
    pub peek_index: usize,
    pub consume_index: usize
}

pub trait Peekable {
    fn peek(&self, index: usize) -> Result<String, &'static str>;
    fn peek_next(&mut self) -> Result<String, &'static str>;
}

impl Peekable for UserInput{
    //Peek at index
    fn peek(&self, index: usize) -> Result<String, &'static str>{
        if index < self.vector_length{
            return Ok(self.vector_input[index].clone())
        }
        return Err("ERROR: Unable to peek.");
    }

    //Peeks next character
    fn peek_next(&mut self) -> Result<String, &'static str>{
        let obj_index = self.peek_index;

        let res = self.peek(obj_index);
        if res.is_ok(){
            self.peek_index += 1;
            return res;
        }
        return Err("ERROR: Unable to peek next");
    }
}

pub trait Consumable {
    fn consume(&mut self) -> Result<String, &'static str>;
    fn is_eof(&self, character: String) -> bool;
}

impl Consumable for UserInput{
    //Consumes one command at a time. Does not remove item to optimize performance.
    fn consume(&mut self) -> Result<String, &'static str>{
        let con_index = self.consume_index;
        let vec_length = self.vector_length;

        if con_index < vec_length{
            let item = self.vector_input[self.consume_index].clone();
            self.consume_index += 1;
            return Ok(item);
        }
        return Err("ERROR: Unable to consume");
    }

    //Checks for EOF
    fn is_eof(&self, character: String) -> bool{
        if character == "?"{
            return true;
        }
        return false;
    }
}


impl fmt::Debug for UserInput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UserInput {{ vector_input: {:?}, vector_length: {}, core_command: {}, sub_commands: {:?}, peek_index: {}, consume_index: {} }}", 
            self.vector_input, self.vector_length, self.core_command, self.sub_commands, self.peek_index, self.consume_index)
    }
}



//Accepts user input and vectorizes
pub fn accept_input(input: &str) -> UserInput{
    let mut input_parts: Vec<&str> = input.split(' ').collect();

    //EOF character is "?"
    input_parts.push("?");

    let mut string_parts: Vec<String> = Vec::new();

    for part in input_parts.clone(){
        string_parts.push(part.to_string())
    }
    
    let size = input_parts.len();

    if size < 1 {
        panic!("ERROR! No arguments provided")
    }

    let main: String = string_parts[0].clone();
    let rest: Vec<String> = string_parts[1..].to_vec();


    return UserInput {
        vector_input:string_parts, 
        vector_length:size, 
        core_command:main, 
        sub_commands:rest, 
        peek_index:0, 
        consume_index:0};
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input(){
        let input = accept_input("This is a string message");
        assert_eq!(input.vector_input[0], "This");
        assert_eq!(input.vector_input[1], "is");
        assert_eq!(input.vector_input[2], "a");
        assert_eq!(input.vector_input[3], "string");
        assert_eq!(input.vector_input[4], "message");
        assert_eq!(input.vector_input[5], "?");
        assert_eq!(input.vector_length, 6);
        assert_eq!(input.core_command, "This");
        assert_eq!(input.sub_commands[0], "is");
        assert_eq!(input.sub_commands[1], "a");
        assert_eq!(input.sub_commands[2], "string");
        assert_eq!(input.sub_commands[3], "message");
        assert_eq!(input.peek_index, 0);
        assert_eq!(input.consume_index, 0);
    }


    #[test]
    fn test_peek(){
        let input = accept_input("This is a string message");
        assert_eq!(input.peek(0), Ok("This".to_string()));
        assert_eq!(input.peek(1), Ok("is".to_string()));
        assert_eq!(input.peek(2), Ok("a".to_string()));
        assert_eq!(input.peek(3), Ok("string".to_string()));
        assert_eq!(input.peek(4), Ok("message".to_string()));
        assert_eq!(input.peek(5), Ok("?".to_string()));
        assert_eq!(input.peek(6), Err("ERROR: Unable to peek."));
    }

    #[test]
    fn test_consume(){
        let mut input = accept_input("This is a string message");
        assert_eq!(input.consume(), Ok("This".to_string()));
        assert_eq!(input.consume(), Ok("is".to_string()));
        assert_eq!(input.consume(), Ok("a".to_string()));
        assert_eq!(input.consume(), Ok("string".to_string()));
        assert_eq!(input.consume(), Ok("message".to_string()));
        assert_eq!(input.peek(5), Ok("?".to_string()));
        assert_eq!(input.peek(6), Err("ERROR: Unable to peek."));
    }
}

