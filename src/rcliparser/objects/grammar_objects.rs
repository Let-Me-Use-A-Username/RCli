use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::token_objects::InvocationCommand;

///Core object of RCli.s
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq)]
pub struct Grammar{
    command_invocations: HashMap<CommandType, Command>,
    pipe_commands: HashMap<PipeType, String>,
    flag_type: HashMap<FlagType, Flags>,
    bnf_grammar: HashMap<BnfType, BnfSyntax>
}

impl Grammar{
    ///Next valid object in BNF Grammar.
    pub fn accepts_next(&self, current: &BnfType, next: &BnfType) -> bool{
        let pair =  &self.bnf_grammar.get_key_value(current);
        if pair.is_some(){
            return pair.unwrap().1.next.contains(next)
        }
        return false
    }

    ///If string input matched a command invocation return it else none.
    pub fn get_command(&self, command: &String) -> Option<InvocationCommand>{
        for (command_type, invocation_command) in &self.command_invocations{
            let match_command =  invocation_command.match_string(command);
            if match_command.is_some(){
                return Some(InvocationCommand::new(command_type.clone(), match_command.unwrap().get_flags()))
            }
        }
        return None
    }

    pub fn match_string_to_command(&self, command: &String) -> Option<()>{
        for (_, invocation_command) in &self.command_invocations{
            let match_command =  invocation_command.match_string(command);
            if match_command.is_some(){
                return Some(());
            }
        }
        return None
    }

    pub fn get_pipe(&self, pipe: &String) -> Option<PipeType>{
        for(pipe_type, pipe_string) in &self.pipe_commands{
            if pipe_string.eq(pipe){
                return Some(pipe_type.clone())
            }
        }
        return None
    }

    ///Iterate available flags to check if given flag exists.
    pub fn get_flag(&self, flag: &String) -> Option<(&FlagType, &Flags)>{
        for (_type, flag_values) in &self.flag_type{
            let flags =  flag_values.get_flag_values();
            if flags.contains(flag){
                return Some((_type, flag_values))
            }
        }
        return None
    }

    ///If flag is followed by an object or is a sole flag. For example -destination aFile.txt
    pub fn flag_accepts_obj(&self, flag: &FlagType) -> bool{
        return self.flag_type.get(&flag).unwrap().has_obj
    }
}

///Command types used to invoke a command.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
pub enum CommandType{
    HOME,
    CWD,
    ECHO,
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
    FIND,
    INVALID
}

///Valid command invocations (String) and valid flag types.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
pub struct Command{
    invocation_name: Vec<String>,
    flags: Vec<FlagType>,
}

impl Command{
    ///Matches a string to a command invocation string.
    pub fn match_string(&self, command: &String) -> Option<Self>{
        for name in &self.invocation_name{
            if name.eq(command){
                return Some(self.clone())
            }
        }
        return None
    }

    ///Returns flag types for a valid command.
    pub fn get_flags(&self) -> Vec<FlagType>{
        return self.flags.clone()
    }
}


///Pipe commands type
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Hash)]
pub enum PipeType{
    PIPE,
    REDIRECT
}



///All available flag types.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
pub enum FlagType{
    RECURSIVE,
    DESTINATION,
    HIDDEN,
    FORCE,
}

///All available invocations for a given flag. Can be followed by an object.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
pub struct Flags{
    flag_values: Vec<String>,
    has_obj: bool
}
impl Flags{
    ///Return flag values (String) for a given type.
    pub fn get_flag_values(&self) -> &Vec<String>{
        return &self.flag_values
    }
}



///Bnf grammar object types.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash, Copy)]
pub enum BnfType{
    START,
    CORE,
    PIPE,
    OBJECT,
    FLAG,
    END
}

///Next object that can exist after current.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
pub struct BnfSyntax{
    pub next: Vec<BnfType>
}