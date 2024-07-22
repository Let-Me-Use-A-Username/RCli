pub struct AbstractSyntaxTree<T>{
    root: Node<T>,
    current: Node<T>
}
impl<T: std::clone::Clone> AbstractSyntaxTree<T>{
    pub fn new(root: Node<T>) -> AbstractSyntaxTree<T>{
        AbstractSyntaxTree { root: root.clone() , current: root}
    }

    pub fn root(&self) -> &Node<T>{
        return &self.root;
    }

    pub fn switch_root(&mut self, mut new_root: Node<T>){
        let old_root = &self.root;
        new_root.add_left(old_root.clone());
        self.root = new_root;
    }

    pub fn current(&self) -> &Node<T>{
        return &self.current;
    }

    pub fn switch_current(&mut self, mut new_current: Node<T>){
        let old_current = &self.current;
        new_current.add_left(old_current.clone());
        self.root = new_current;
    }
}

#[derive(Clone)]
pub struct Node<T>{
    node_type: NodeType,
    parent: Option<Box<Node<T>>>,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
    data: T
}
impl<T> Node<T>{
    pub fn new(n_type: NodeType, parent: Option<Node<T>>, data: T) -> Node<T>{
        let mut par = None;

        if parent.is_some(){
            par = Some(Box::new(parent.unwrap()));
        }

        Node { node_type: n_type, parent: par, left: None, right: None, data: data }
    }

    pub fn left(&self) -> &Option<Box<Node<T>>>{
        return &self.left;
    }

    pub fn right(&self) -> &Option<Box<Node<T>>>{
        return &self.right;
    }

    pub fn parent(&self) -> &Option<Box<Node<T>>>{
        return &self.parent;
    }

    pub fn n_type(&self) -> &NodeType{
        return &self.node_type
    }

    pub fn add_left(&mut self, node: Node<T>){
        self.left = Some(Box::new(node));
    }

    pub fn add_right(&mut self, node: Node<T>){
        self.right = Some(Box::new(node));
    }
}

#[derive(Clone)]
pub enum NodeType{
    Command,
    Parameter,
    Flag,
    Operator,
}


#[cfg(test)]
mod tests {
    use crate::structures::{commands::{command::Command, flag::Flag}, token::Token};

    use super::*;

    fn test_node(){
        //FIXME : make data be Command, Token, Flag or Operator
        //ls operation
        let root = Node::new(NodeType::Command, None, Command::Ls);
        let target = Node::new(NodeType::Parameter, Some(root), Token::Word("a/dir".to_string()));
        let flag = Node::new(NodeType::Flag, Some(root), Flag::Force);
        root.add_left(target);
        root.add_right(flag);

        let op = Node::new(NodeType::Operator, None, Token::Ambersant);
    }
}