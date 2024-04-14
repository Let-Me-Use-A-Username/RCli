use std::collections::HashMap;

use rand::Rng;

pub struct Tree<T>{
    id: String,
    root: Node<T>,
    rows: HashMap<i32, Vec<Node<T>>>,
    nodes: Vec<Node<T>>
}

impl<T> Tree<T>{
    pub fn new(head: Node<T>) -> Tree<T>{
        let id: String = rand::thread_rng().gen::<u32>().to_string();
        let root: Node<T> = head;
        let mut rows = HashMap::new();
        rows.insert(0, vec![root]);
        let nodes: Vec<Node<T>> = vec![root];
        Tree {
            id: id,
            root: root,
            rows: rows,
            nodes: nodes
        }
    }
}


struct Node<T>{
    id: String, 
    children: Option<Vec<Node<T>>>,
    data: Option<T>,
}

impl<T> Node<T>{
    pub fn new(head: T, children: Option<Vec<Node<T>>>) -> Self{
        let id: String = rand::thread_rng().gen::<u32>().to_string();
        match children{
            Some(Vec<Node<T>>) => {

            },
            None() => {

            }
        }
        let children: Option<Vec<Node<T>>> = children;
        let data = Some(head).or(None);
        Node {
            id: id,
            children: children,
            data: data
        }
    }
}



#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_tree(){
        parse("create readme.txt".to_string());
        parse("create ./path/to/readme.txt".to_string());
        parse("list path/to/file".to_string());
        parse("list path/to/file --hidden".to_string());
        parse("list --hidden".to_string());
    }
    
}