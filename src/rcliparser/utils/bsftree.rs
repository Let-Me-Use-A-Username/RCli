use std::collections::HashMap;
use crate::rcliparser::lexical_analyzer::Tokens;

use rand::Rng;

pub struct Tree{
    id: String,
    root: Node,
    rows: HashMap<i32, Vec<Node>>,
    height: i32,
    level: i32,
    size: usize
}

impl Tree{
    pub fn new(head: Tokens){

    }
    
    // fn insert(&mut self, position: i32, node: Node, parent: Node){
    // }
    
    // fn get_node(&self, data: Option<T>) -> Option<T> {
    //     todo!("todo");
    // }

    // fn get_row(&self, index: usize) -> Option<Vec<T>> {
    //     todo!("todo");
    // }

    // fn get_children(&self, data: Option<T>) -> Option<Vec<T>> {
    //     todo!("todo");
    // }
}

struct Node{
    node_type: NodeType,
    data: Option<Tokens>
}

enum NodeType{
    LeafNode,
    StructNode
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_tree_create(){
    }
    
}