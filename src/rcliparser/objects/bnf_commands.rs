use std::{collections::HashMap, fmt};
use serde::{Deserialize, Serialize};

/*
Objects used to parse bnf grammar.
*/
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct BnfGrammar{
    pub command_type: HashMap<String, Command>
}

impl BnfGrammar{
    pub fn get_hashmap(&self) -> &HashMap<String, Command>{
        return &self.command_type
    }
}


#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub enum CommandType{
    Core,
    Sub,
    Object,
    Flag,
    INVALID,
    None
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct Command{
    pub next: Vec<CommandType>
}

impl fmt::Debug for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Command {{ next: {:?}}}", 
            self.next)
    }
}

/*
Objects used to parse command syntax.
*/

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct InvocationCommandSyntax{
    commands: HashMap<InvocationName, InvocationCommand>
}

impl InvocationCommandSyntax{
    pub fn get_hashmap(&self) -> &HashMap<InvocationName, InvocationCommand>{
        return &self.commands
    }

    pub fn get_value(&self, name: &InvocationName) -> Option<&InvocationCommand>{
        if self.commands.contains_key(name){
            return self.commands.get(name)
        }
        else{
            return None
        }
    }

    pub fn get_all_values(&self) -> Vec<InvocationCommand>{
        let mut command_syntax = Vec::<InvocationCommand>::new();

        for (_, value) in &self.commands{
            command_syntax.push(value.clone());
        }
        return command_syntax
    }
}
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Hash)]
pub enum InvocationName{
    CWD,
    TOUCH,
    MKDIR,
    REMOVE,
    COPY,
    MOVE,
    READ,
    LIST,
    CD,
    EXIT
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