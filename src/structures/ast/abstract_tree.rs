use core::fmt;
use std::{borrow::BorrowMut, collections::VecDeque, ops::Deref};

use serde::Serializer;

use crate::structures::{commands::{command::Command, flag::Flag}, token::Token};

pub struct AbstractSyntaxTree{
    root: Node,
    current: Node
}
impl AbstractSyntaxTree{
    pub fn new(root: Node) -> AbstractSyntaxTree{
        AbstractSyntaxTree { root: root.clone() , current: root}
    }

    pub fn root(&mut self) -> &mut Node{
        return self.root.borrow_mut();
    }

    pub fn switch_root(&mut self, new_root: &mut Node){
        new_root.add_left(&self.root);
        self.root = new_root.clone();
    }

    pub fn current(&mut self) -> &mut Node{
        return self.current.borrow_mut();
    }

    pub fn switch_current(&mut self, new_current: Node){
        self.current = new_current;
    }
}

impl fmt::Debug for AbstractSyntaxTree{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut layer = 0;
        let mut counter = 0;
        let mut threshold = 2;
        let mut nodes: VecDeque<Option<Box<Node>>> = vec![self.root.left.clone(), self.root.right.clone()].into();
        let mut status = f.write_fmt(format_args!("L{}, nodes: {:?}\n", layer, self.root));
        layer += 1;
        
        while !nodes.is_empty(){
            let current = nodes.pop_front();

            if current.clone().is_some_and(|x| x.is_some()){
                let current_node = current.unwrap().unwrap();
                status = f.write_fmt(format_args!("L{}, nodes: {:?}\n", layer, current_node.deref()));
                nodes.push_back(current_node.left);
                nodes.push_back(current_node.right);
            }
            counter += 1;

            if counter == threshold{
                threshold *= 2;
                counter = 0;
                layer += 1;
            }
        }

        return status;
    }
}

#[derive(Clone)]
pub struct Node{
    node_type: NodeType,
    parent: Option<Box<Node>>,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    data: NodeData
}
impl Node{
    pub fn new(n_type: NodeType, data: NodeData) -> Node{
        Node { node_type: n_type, parent: None, left: None, right: None, data: data }
    }

    pub fn n_type(&self) -> &NodeType{
        return &self.node_type
    }

    pub fn left(&self) -> &Option<Box<Node>>{
        return &self.left;
    }

    pub fn right(&self) -> &Option<Box<Node>>{
        return &self.right;
    }

    pub fn parent(&self) -> &Option<Box<Node>>{
        return &self.parent;
    }

    pub fn data(&self) -> &NodeData{
        return &self.data
    }

    pub fn add_left(&mut self, node: &Node){
        self.left = Some(Box::new(node.to_owned()));
    }

    pub fn add_right(&mut self, node: &Node){
        self.right = Some(Box::new(node.to_owned()));
    }

    pub fn add_parent(&mut self, node: &Node){
        self.parent = Some(Box::new(node.to_owned()));
    }
}

impl fmt::Debug for Node{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node").field("node_type", &self.node_type).field("left", &self.left).field("right", &self.right).finish()
    }
}

#[derive(Clone, Debug)]
pub enum NodeType{
    Command,
    Parameter,
    Flag,
    Operator,
}

#[derive(Clone, Debug)]
pub enum NodeData{
    Command(Command),
    Token(Token),
    Flag(Flag)
}


#[cfg(test)]
mod tests {
    use std::borrow::BorrowMut;

    use crate::structures::{commands::{command::Command, flag::Flag}, token::Token};

    use super::*;

    #[test]
    fn test_node(){
        // Review: Remove some complexity. Possibly by making the tree more automated and  interacting directly with the tree.
        //ls a/dir --force ; grep *.txt > file
        let root = Node::new(NodeType::Command, NodeData::Command(Command::Ls));
        let mut tree = AbstractSyntaxTree::new(root.clone());

        //ls target
        let mut target = Node::new(NodeType::Parameter, NodeData::Token(Token::Word("a/dir".to_string())));
        tree.root().add_left(&target);
        target.add_parent(tree.root());

        //ls flag
        let mut flag = Node::new(NodeType::Flag, NodeData::Flag(Flag::Force));
        tree.root().add_right(&flag);
        flag.add_parent(tree.root());

        //sequence
        let mut op: Node = Node::new(NodeType::Operator, NodeData::Token(Token::GreekQues));
        tree.switch_root(op.borrow_mut());

        //grep command
        let mut grep: Node = Node::new(NodeType::Command, NodeData::Command(Command::Grep));
        grep.add_parent(&tree.root());

        //grep regex
        let mut regex: Node = Node::new(NodeType::Parameter, NodeData::Token(Token::Regex("*.txt".to_string())));
        grep.add_left(&regex);
        regex.add_parent(&grep);
        tree.root().add_right(&grep);

        println!("{:?}", tree); 
    }
}