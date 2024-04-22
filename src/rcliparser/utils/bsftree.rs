use std::{fmt::{self, Debug}, vec};

/*
Tree format:
            Create 
        File      Directory
FileFlags              DirFlags

Current implementation is too complex.
Each tree has a root of type Node<T>, a parent and a vector of subtrees. 
Each node has data, level, siblings and children.

Nodes can have children and siblings without the Tree knowing. Is this desired(???)
*/

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Tree<T>{
    root: Node<T>,
    parent: Option<Box<Tree<T>>>,
    subtrees: Option<Box<Vec<Tree<T>>>>
}

/*
Tree represents a single core command (like CREATE and COPY etc.) Each command has children that 
represent the available sub trees (meaning the different parameters than can invoke a command) 
like flags and object.
*/
impl<T> Tree<T>{
    pub fn new(head: T) -> Self{
        let root = Node::new(head, 0);

        Tree{
            root: root,
            parent: None,
            subtrees: None
        }
    }

    pub fn add_parent(&mut self, parent: Tree<T>){
        if self.parent.is_none(){
            self.parent = Some(Box::new(parent));
            return;
        }
    }

    pub fn add_subtree(&mut self, subtree: Tree<T>){
        if self.subtrees.is_none(){
            self.subtrees = Some(Box::new(vec![subtree]));
            return;
        }

        let mut self_subtrees = self.subtrees.as_mut().unwrap();
        self_subtrees.push(subtree);
    }
}



#[derive(PartialEq, Eq, Clone)]
pub struct Node<T>{
    data: T,
    level: i32,
    children: Option<Box<Vec<Node<T>>>>,
    siblings: Option<Box<Vec<Node<T>>>>
}

impl<T> Node<T>{
    pub fn new(data: T, level: i32) -> Self{
        Node { 
            data: data,
            level: level,
            children: None,
            siblings: None
        }
    }

    pub fn add_child(&mut self, new_node: Node<T>){
        if self.level < new_node.level{

            match self.children.as_mut(){
                Some(c) => {
                    unsafe{
                        c.push(new_node);
                    }
                },
                None => {
                    self.children = Some(Box::new(vec![new_node]));
                }
            }
            return;
        }
        todo!("throw error");
    }

    pub fn add_sibling(&mut self, new_node: Node<T>){
        if self.level == new_node.level{

            match self.siblings.as_mut(){
                Some(c) => {
                    unsafe{
                        c.push(new_node);
                    }
                },
                None => {
                    self.siblings = Some(Box::new(vec![new_node]));
                }
            }
            return;
        }
        todo!("throw error");
    }
}

impl<T> fmt::Debug for Node<T> where T: Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Note {{ data: {:?}, level: {}, children: {:?}, siblings: {:?}}}", 
            self.data, self.level, self.children, self.siblings)
    }
}

#[cfg(test)]
mod test{
    use super::*;
    use crate::rcliparser::lexical_analyzer::FlagType::TERMINAL;
    use crate::rcliparser::lexical_analyzer::Tokens;

    use crate::rcliparser::lexical_analyzer::TokenCommands::COPY;
    use crate::rcliparser::lexical_analyzer::TokenCommands::LIST;
    use crate::rcliparser::lexical_analyzer::TokenCommands::CREATE;

    use crate::rcliparser::lexical_analyzer::TokenObjects::FILE;

    use crate::rcliparser::lexical_analyzer::TokenFlag::FLAG;

    #[test]
    fn test_node_chain(){
        let mut copy_token = Node::new(Tokens::TokenCommands((COPY)), 0);
        let mut list_token = Node::new(Tokens::TokenCommands((LIST)), 0);
        let mut create_token = Node::new(Tokens::TokenCommands((CREATE)), 0);

        let new_node = Node::new(Tokens::TokenObjects(FILE(("readme.txt".to_string()))), 1);
        copy_token.add_child(new_node.clone());
        list_token.add_child(new_node.clone());
        create_token.add_child(new_node.clone());

        let new_new_node = Node::new(Tokens::TokenFlag(FLAG(TERMINAL, "-d".to_string())), 2);
        copy_token.add_child(new_new_node.clone());
        list_token.add_child(new_new_node.clone());
        create_token.add_child(new_new_node.clone());

        copy_token.add_sibling(list_token.clone());
        copy_token.add_sibling(create_token.clone());
        list_token.add_sibling(create_token.clone());

        assert_eq!(copy_token.clone().children.unwrap().len(), 2);
        assert_eq!(copy_token.clone().siblings.unwrap().len(), 2);

        assert_eq!(list_token.clone().children.unwrap().len(), 2);
        assert_eq!(list_token.clone().siblings.unwrap().len(), 1);

        assert_eq!(create_token.clone().children.unwrap().len(), 2);
    }

    #[test]
    fn test_tree(){
        let mut create_token: Node<Tokens> = Node::new(Tokens::TokenCommands((CREATE)), 0);
        let create_file: Node<Tokens> = Node::new(Tokens::TokenObjects(FILE(("readme.txt".to_string()))), 1);

        let mut tree = Tree::new(create_token);

        let mut sub_tree = Tree::new(create_file);
        sub_tree.add_parent(tree.clone());

        //fails because tree has subtrees
        assert_eq!(sub_tree.parent.clone(), Some(Box::new(tree.clone())));

        tree.add_subtree(sub_tree.clone());

        assert_eq!(tree.subtrees, Some(Box::new(vec![sub_tree])));
    }
    
}