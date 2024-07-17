pub struct AbstractSyntaxTree{
    root: Node,
    current: Node
}

pub struct Node{
    node_type: NodeType,
    parent: Box<Node>,
    left: Box<Node>,
    right: Box<Node>,
}

pub enum NodeType{
    Command,
    Operation,
}