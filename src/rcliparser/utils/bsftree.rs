use std::{collections::HashMap, os::windows::raw::SOCKET};
use crate::rcliparser::lexical_analyzer::Tokens;

use rand::Rng;

pub struct Tree<T>{
    root: Node<T>,
    rows: HashMap<i32, Vec<Node<T>>>, 
    height: usize,
    level: usize,
    size: usize
}

impl<T> Tree<T>{
    pub fn new(head: Option<Tokens>){
        match &head {
            Some(node) => {
                let root = Node::new(&node);
                let rows = HashMap::from([
                    (0, vec![&head])
                ]);
                let height = 0;
                let level = 0;
                let size = 1;
            },
            None => {
                todo!("throw error");
            }
        }
        
    }
}

pub struct Node<T>{
    node_type: NodeType,
    data: Option<T>,
    children: Option<Vec<T>>
}

impl<T> Node<T>{
    pub fn new(data: T) -> Self{
        let data = Some(data);
        let children = Some(Vec::<T>::new());
        let datatype = NodeType::LeafNode;
        Node { 
            node_type: (datatype), 
            data: (data), 
            children: (children) }
    }
}

enum NodeType{
    LeafNode,
    StructNode
}

#[cfg(test)]
mod test{
    use super::*;
    use crate::rcliparser::lexical_analyzer::TokenCommands::COPY;

    #[test]
    fn test_node_create(){
        let node = Node::new("some data");
        let node2 = Node::new(Tokens::TokenCommands(COPY));
    }

    #[test]
    fn test_tree_create(){
    }
    
}