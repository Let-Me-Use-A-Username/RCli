use std::fmt;

#[derive(PartialEq)]
pub struct UserInput{
    pub vector_input: Vec<String>,
    vector_length: usize,
    pub core_command: String,
    pub rest_commands: Vec<String>,
    pub peek_index: usize,
    pub consume_index: usize,
    pub analyzed: bool
}

pub trait Peekable {
    fn peek(&mut self, index: usize) -> Result<String, &'static str>;
    fn peek_next(&mut self) -> Result<String, &'static str>;
}

impl Peekable for UserInput{
    //Peek at index
    fn peek(&mut self, index: usize) -> Result<String, &'static str>{
        if index < self.vector_length{
            let peek_item = self.vector_input[index].clone();

            if !self.is_eof(&peek_item){
                return Ok(peek_item)
            }
        }
        self.analyzed = true;
        return Err("?");
    }

    //Peeks next character
    fn peek_next(&mut self) -> Result<String, &'static str>{
        let obj_index = self.peek_index;

        let res = self.peek(obj_index);
        if res.is_ok(){
            self.peek_index += 1;
            return res;
        }
        return Err("?");
    }
}

pub trait Consumable {
    fn consume(&mut self) -> Result<String, &'static str>;
    fn is_eof(&self, character: &String) -> bool;
}

impl Consumable for UserInput{
    //Consumes one command at a time. Does not remove item to optimize performance.
    fn consume(&mut self) -> Result<String, &'static str>{
        let con_index = self.consume_index;
        let vec_length = self.vector_length;

        if con_index < vec_length{
            let item = self.vector_input[self.consume_index].clone();
            if !self.is_eof(&item) {
                self.consume_index += 1;
                return Ok(item);
            }
        }
        self.analyzed = true;
        return Err("?");
    }

    //Checks for EOF
    fn is_eof(&self, character: &String) -> bool{
        if character == "?"{
            return true;
        }
        return false;
    }
}


impl fmt::Debug for UserInput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UserInput {{ vector_input: {:?}, vector_length: {}, core_command: {}, sub_commands: {:?}, peek_index: {}, consume_index: {} }}", 
            self.vector_input, self.vector_length, self.core_command, self.rest_commands, self.peek_index, self.consume_index)
    }
}



//Accepts user input and vectorizes
pub fn accept_input(input: &str) -> UserInput{
    let mut input_parts: Vec<&str> = input.split(' ').collect();

    //EOF character is "?"
    input_parts.push("?");

    let mut string_parts: Vec<String> = Vec::new();

    for part in input_parts.clone(){
        string_parts.push(part.trim().to_string())
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
        rest_commands:rest, 
        peek_index:0, 
        consume_index:0,
        analyzed: false};
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input(){
        let input = accept_input("create readme.txt");
        assert_eq!(input.vector_input[0], "create");
        assert_eq!(input.vector_input[1], "readme.txt");
        assert_eq!(input.vector_input[2], "?");
        assert_eq!(input.vector_length, 3);
        assert_eq!(input.core_command, "create");
        assert_eq!(input.rest_commands[0], "readme.txt");
        assert_eq!(input.peek_index, 0);
        assert_eq!(input.consume_index, 0);
    }


    #[test]
    fn test_peek(){
        let mut input = accept_input("create ./Desktop/Some/Dir");
        assert_eq!(input.peek(0), Ok("create".to_string()));
        assert_eq!(input.peek(1), Ok("./Desktop/Some/Dir".to_string()));
        assert_eq!(input.peek(2), Err("?"));
    }

    #[test]
    fn test_peek_next(){
        let mut input = accept_input("list Desktop/Some/Dir --hidden");
        assert_eq!(input.peek_next(), Ok("list".to_string()));
        assert_eq!(input.peek_index, 1);
        assert_eq!(input.peek_next(), Ok("Desktop/Some/Dir".to_string()));
        assert_eq!(input.peek_index, 2);
        assert_eq!(input.peek_next(), Ok("--hidden".to_string()));
        assert_eq!(input.peek_index, 3);
        assert_eq!(input.peek_next(), Err("?"));
        assert_eq!(input.peek_index, 3);
    }

    #[test]
    fn test_consume(){
        let mut input = accept_input("list --hidden");
        assert_eq!(input.consume(), Ok("list".to_string()));
        assert_eq!(input.consume_index, 1);
        assert_eq!(input.consume(), Ok("--hidden".to_string()));
        assert_eq!(input.consume_index, 2);
        assert_eq!(input.consume(), Err("?"));
        assert_eq!(input.consume_index, 2);
    }
}

