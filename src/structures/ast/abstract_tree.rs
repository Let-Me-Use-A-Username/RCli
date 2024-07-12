use std::process::Command;


pub struct AbstractSyntaxTree{
    root: Node
}

pub enum Node{
    CommandNode(CommandNode),
    OperationNode(OperationNode),
}


pub struct CommandNode{
    command: Command
}

pub struct OperationNode{
    operation: String,
    left: Box<Node>,
    right: Box<Node>,
}