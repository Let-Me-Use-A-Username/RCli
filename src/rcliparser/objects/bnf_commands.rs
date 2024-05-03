use std::{collections::HashMap, fmt, iter};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub enum CommandType{
    Core,
    Sub,
    Object,
    Flag,
    INVALID
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct Command{
    pub command: Vec<String>,
    pub next: Vec<CommandType>,
    pub is_terminal: bool
}

impl fmt::Debug for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Command {{ commands: {:?}, next: {:?}, is_terminal: {:?}}}", 
            self.command, self.next, self.is_terminal)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct InvocationCommandSyntax{
    commands: HashMap<String, InvocationCommand>
}

impl InvocationCommandSyntax{
    pub fn get_hashmap(&self) -> &HashMap<String, InvocationCommand>{
        return &self.commands
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct InvocationCommand{
    pub invocation_name: Vec<String>,
    flags: Vec<String>
}

impl InvocationCommand{

    pub fn match_name_iter(&self, command: &String) -> bool{
        let con = self.invocation_name.iter().any(|val| val.contains(command));
        
        return con
    }

    pub fn match_flag_iter(&self, flag: &String) -> bool{
        let con = self.flags.iter().any(|val| val.contains(flag));

        return con
    }
}