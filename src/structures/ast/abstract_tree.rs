use core::fmt;
use std::{borrow::Borrow, cell::RefCell, collections::VecDeque, rc::{Rc, Weak}};

use crate::structures::{commands::{command::Command, flag::Flag}, token::Token};

#[derive(Debug)]
pub struct AbstractSyntaxTree{
    root: Rc<RefCell<Node>>,
    current: Rc<RefCell<Node>>
}
impl AbstractSyntaxTree{
    pub fn new(root: Node) -> AbstractSyntaxTree{
        AbstractSyntaxTree { 
            root: Rc::new(RefCell::new(root.to_owned())), 
            current: Rc::new(RefCell::new(root.to_owned()))
        }
    }

    fn swap_current(&mut self, node: Node){
        self.current = Rc::new(RefCell::new(node));
    }

    pub fn ascend(&mut self){
        match &self.current.borrow_mut().get_parent().upgrade(){
            Some(parent) => {
                self.swap_current(parent.into_inner().unwrap());
            },
            None => todo!(),
        }
    }

    pub fn descend_left(&mut self){
        let left =  &self.current.into_inner().get_left();

        match left.into_inner(){
            Some(value) => self.swap_current(value),
            None => todo!(),
        }
    }

    pub fn add_command(&self, command: Node){
        self.current.borrow_mut().add_left(command);
    }

    pub fn add_parameter(&self, parameter: Node){
        self.current.borrow_mut().add_left(parameter);
    }

    pub fn add_flag(&self, flag: Node){
        self.current.borrow_mut().add_right(flag);
    }

    pub fn add_operator(&self, operator: Node){
        self.current.borrow_mut().add_right(operator);
    }
}

// impl fmt::Debug for AbstractSyntaxTree{
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let mut layer = 0;
//         let mut counter = 0;
//         let mut threshold = 2;
//         let binding = <RefCell<Node> as Clone>::clone(&self.root).into_inner();
//         let mut nodes: VecDeque<&Option<Rc<RefCell<Node>>>> = vec![binding.get_left(), binding.get_right()].into();
//         let mut status = f.write_fmt(format_args!("L{}, nodes: {:?}\n", layer, self.root));
//         layer += 1;
        
//         while !nodes.is_empty(){
//             match nodes.pop_front().unwrap() {
//                  Some(value) => {
//                     status = f.write_fmt(format_args!("L{}, nodes: {:?}\n", layer, value));
            
//                     nodes.push_back(value.into_inner().get_left());
//                     nodes.push_back(value.into_inner().get_right())
//                  },
//                  None => {
//                     break;
//                  },
//             };

            
//             counter += 1;

//             if counter == threshold{
//                 threshold *= 2;
//                 counter = 0;
//                 layer += 1;
//             }
//         }

//         return status;
//     }
// }

#[derive(Clone)]
pub struct Node{
    parent: Weak<RefCell<Option<Node>>>,
    left: Rc<RefCell<Option<Node>>>,
    right: Rc<RefCell<Option<Node>>>,
    node_type: NodeType,
    data: NodeData
}
impl Node{
    pub fn new(n_type: NodeType, parent: Option<Node>, data: NodeData) -> Node{
        Node { node_type: n_type, parent: Rc::downgrade(&Rc::new(RefCell::new(parent))), left: Rc::new(RefCell::new(None)), right: Rc::new(RefCell::new(None)), data: data }
    }

    pub fn get_type(&self) -> &NodeType{
        return &self.node_type
    }

    pub fn get_left(&self) -> Rc<RefCell<Option<Node>>>{
        return Rc::clone(&self.left)
    }

    pub fn get_right(&self) -> Rc<RefCell<Option<Node>>>{
        return Rc::clone(&self.right);
    }

    pub fn get_parent(&self) -> Weak<RefCell<Option<Node>>>{
        return Weak::clone(&self.parent);
    }

    pub fn get_data(&self) -> &NodeData{
        return &self.data
    }

    pub fn add_left(&mut self, node: Node) -> Rc<RefCell<Option<Node>>>{
        self.left = Rc::new(RefCell::new(Some(node)));
        return Rc::clone(&self.left)
    }

    pub fn add_right(&mut self, node: Node) -> Rc<RefCell<Option<Node>>>{
        self.right = Rc::new(RefCell::new(Some(node.to_owned())));
        return Rc::clone(&self.right);
    }

    pub fn reparent(&mut self, node: Node) -> Weak<RefCell<Option<Node>>>{
        self.parent = Rc::downgrade(&Rc::new(RefCell::new(Some(node))));
        return Weak::clone(&self.parent)
    }
}

impl fmt::Debug for Node{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //f.debug_struct("Node").field("node_type", &self.node_type).field("left", &self.left).field("right", &self.right).finish()
        f.debug_struct("Node").field("data", &self.data).field("left", &self.left).field("right", &self.right).field("parent", &self.parent).finish()
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
mod tests {use crate::structures::{commands::{command::Command, flag::Flag}, token::Token};

    use super::*;

    #[test]
    fn test_tree_abstractions(){
        //ls a/dir --force ; grep *.txt > file
    }
}