use std::collections::HashMap;

pub struct Tree<T>{
    root: Node<T>,
    command_list: HashMap<Node<T>, Vec<Node<T>>>,
    height: usize
}

impl<T> Tree<T>{
    pub fn new(head: T) -> Tree<T>{
        let root = Node::new(head);
        let command_list = HashMap::<Node<T>, Vec<Node<T>>>::new();
        let height = 0;
        Tree{
            root,
            command_list,
            height
        }
    }

    pub fn insert_under(&mut self, parent: Node<T>, data: T){
        for key in self.command_list.keys(){
            
        }
    }
}

#[derive(PartialEq)]
pub struct Node<T>{
    data: T,
}

impl<T> Node<T>{
    pub fn new(data: T) -> Self{
        Node { 
            data
        }
    }
}

#[cfg(test)]
mod test{
    use super::*;
    use crate::rcliparser::lexical_analyzer::TokenCommands::COPY;

    #[test]
    fn test_node_create(){
        let token_node = Node::new(COPY);
    }

    #[test]
    fn test_tree_create(){
    }
    
}