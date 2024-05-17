use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::token_objects::InvocationToken;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq)]
pub struct Grammar{
    command_invocations: HashMap<CommandType, Command>,
    flag_type: HashMap<FlagType, Flags>,
    bnf_grammar: HashMap<BnfType, BnfSyntax>
}

impl Grammar{
    pub fn get_invocation_commands(&self) -> HashMap<CommandType, Command>{
        return self.command_invocations
    }

    pub fn get_flag_types(&self) -> HashMap<FlagType, Flags>{
        return self.flag_type
    }

    pub fn get_bnf_grammar(&self) -> HashMap<BnfType, BnfSyntax>{
        return self.bnf_grammar
    }

    pub fn get_command(&self, command: &String) -> Option<InvocationToken>{
        for (command_type, invocation_command) in self.command_invocations{
            let match_command =  invocation_command.match_string(command);
            if match_command.is_some(){
                return Some(InvocationToken::new(command_type, match_command.unwrap().get_flags()))
            }
        }
        return None
    }

    pub fn get_flag(&self, flag: &String) -> Option<(FlagType, Flags)>{
        for (_type, flag_values) in self.flag_type{
            let flags =  flag_values.get_flag_values();
            if flags.contains(flag){
                return Some((_type, flag_values))
            }
        }
        return None
    }

    pub fn flag_accepts_obj(&self, flag: FlagType) -> bool{
        return self.flag_type.get(&flag).unwrap().has_obj
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
pub enum CommandType{
    HOME,
    CWD,
    TOUCH,
    MKDIR,
    REMOVE,
    COPY,
    MOVE,
    READ,
    LIST,
    CD,
    GREP,
    EXIT,
    INVALID
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
pub struct Command{
    invocation_name: Vec<String>,
    flags: Vec<FlagType>,
}

impl Command{
    pub fn match_string(&self, command: &String) -> Option<Self>{
        for name in self.invocation_name{
            if name.eq(command){
                return Some(*self)
            }
        }
        return None
    }

    pub fn get_flags(&self) -> Vec<FlagType>{
        return self.flags
    }
}




#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
pub enum FlagType{
    RECURSIVE,
    DESTINATION,
    FORCE,
    HIDDEN,
    REGEX
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
pub struct Flags{
    flag_values: Vec<String>,
    has_obj: bool
}
impl Flags{
    pub fn get_flag_values(&self) -> Vec<String>{
        return self.flag_values
    }
}




#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
pub enum BnfType{
    CORE,
    SUB,
    OBJECT,
    FLAG,
    NONE
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
pub struct BnfSyntax{
    pub next: Vec<BnfType>
}