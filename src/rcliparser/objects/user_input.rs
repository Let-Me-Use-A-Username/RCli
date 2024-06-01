use std::{collections::VecDeque, fmt};

#[derive(PartialEq)]
pub struct UserInput{
    pub vector_input: VecDeque<String>,
    pub vector_length: usize,
    pub peek_index: usize,
    pub analyzed: bool
}

pub trait Peekable {
    fn peek(&mut self, index: usize) -> Option<String>;
    fn peek_next(&mut self) -> Option<String>;
}

impl Peekable for UserInput{
    //Peek at index
    fn peek(&mut self, index: usize) -> Option<String>{
        if index < self.vector_length{
            let peek_item = self.vector_input[index].clone();

            return Some(peek_item)
        }
        self.analyzed = true;
        return None
    }

    //Peeks next character
    fn peek_next(&mut self) -> Option<String>{
        let obj_index = self.peek_index;

        let res = self.peek(obj_index);
        if res.is_some(){
            self.peek_index += 1;
            return res
        }
        return None
    }
}

pub trait Consumable {
    fn consume(&mut self) -> Option<String>;
}

impl Consumable for UserInput{
    //Consumes one command at a time. Does not remove item to optimize performance.
    fn consume(&mut self) -> Option<String>{
        let con_index = 0;
        let vec_length = self.vector_length;

        if con_index < vec_length{
            
            match self.vector_input.pop_front() {
                Some(some_item) => {
                    self.vector_length -= 1;
                    return Some(some_item)
                },
                None => {
                    self.analyzed = true;
                    return None
                },
            };
        }
        self.analyzed = true;
        return None
    }
}


impl fmt::Debug for UserInput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UserInput {{ vector_input: {:?}, vector_length: {}, peek_index: {}}}", 
            self.vector_input, self.vector_length, self.peek_index)
    }
}