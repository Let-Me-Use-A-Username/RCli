use std::{collections::HashMap, fmt};
use serde::{Deserialize, Serialize};

use super::tokens::{TokenCommandType, TokenCommands};

///Object that is translated from json to struct.
///Represents the BNF grammar, meaning the order in which
///you write a command.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct BnfGrammar{
    pub command_type: HashMap<CommandType, Command>
}

impl BnfGrammar{
    pub fn get_hashmap(&self) -> &HashMap<CommandType, Command>{
        return &self.command_type
    }
}

///The struct that contains the `next` input that can be accepted.
///Used in the lexer to verify the order.
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


#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub enum CommandType{
    CORE,
    SUB,
    OBJECT,
    FLAG,
    NONE
}


///Object used to parse the syntax. This is different from
///the grammar because grammar tells us the order something is written
///while syntax tells us if WHAT is written is correct.
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct InvocationCommandSyntax{
    commands: HashMap<TokenCommandType, InvocationCommand>
}

impl InvocationCommandSyntax{  
    ///Returns either the InvocationCommand that contains the name of the core command
    ///or None
    pub fn get_value_from_string(&self, name: &String) -> Option<&InvocationCommand>{
        for (_, value) in &self.commands{
            let return_command = value.get_command(name);
            if return_command.is_some(){
                return return_command
            }
        }
        return None
    }

    pub fn get_token(&self, name: &String)-> Option<TokenCommands>{
        for (key, value) in &self.commands{
            let return_command = value.get_command(name);
            if return_command.is_some(){
                return Some(TokenCommands::new(*key, return_command.unwrap().flags.clone()))
            }
        }
        return None
    }
}

///Object that contains all the valid names and flags of a single command.
///Is mainly used inside the lexer and then the information is passed onto the tokens.
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct InvocationCommand{
    pub invocation_name: Vec<String>,
    pub flags: Vec<String>
}

impl InvocationCommand{
    ///Function that returns Self if it contains the String.
    pub fn get_command(&self, command: &String) -> Option<&Self>{
        for com in &self.invocation_name{
            if com.eq(command){
                return Some(self)
            }
        }
        return None
    }

    ///Function that checks if the name exists
    pub fn name_exists(&self, command: &String) -> bool{
        let con = self.invocation_name.iter().any(|val| val.contains(command));
        
        return con
    }

    ///Function that checks if the flag exists
    pub fn flag_exists(&self, flag: &String) -> bool{
        let con = self.flags.iter().any(|val| val.contains(flag));

        return con
    }
}