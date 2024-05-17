use super::grammar_objects::{CommandType, FlagType};

pub trait GetValue{
    fn get_value(&self) -> &String;
}

#[derive(Clone, Debug)]
pub enum Token{
    InvocationToken(InvocationToken),
    InvocationPair(InvocationPair),

    TokenCommand(TokenCommand),
    TokenObject(TokenObject),
    TokenFlag(TokenFlag)
}

impl GetValue for Token{
    fn get_value(&self) -> &String{
        match self{
            Token::TokenCommand(core) => {
                return core.get_value()
            },
            Token::TokenObject(obj) => {
                return obj.get_value()
            },
            Token::TokenFlag(flag) => {
                return flag.get_value()
            },
            _ => unreachable!()
        }
    }
}


#[derive(Clone, Debug)]
pub enum TokenCommand{
    COMMAND(String)
}
impl GetValue for TokenCommand{
    fn get_value(&self) -> &String {
        match self{
            TokenCommand::COMMAND(command) => {
                return command
            },
        }
    }
}

#[derive(Clone, Debug)]
pub enum TokenObject{
    OBJECT(String)
}
impl GetValue for TokenObject{
    fn get_value(&self) -> &String {
        match self{
            TokenObject::OBJECT(obj) => {
                return obj
            },
        }
    }
}

#[derive(Clone, Debug)]
pub enum TokenFlag{
    FLAG(String)
}
impl GetValue for TokenFlag{
    fn get_value(&self) -> &String {
        match self{
            TokenFlag::FLAG(flag) => {
                return flag
            },
        }
    }
}

#[derive(Clone, Debug)]
pub struct InvocationToken{
    command_type: CommandType,
    flags: Vec<FlagType>
}
impl InvocationToken{
    pub fn new(command_type: CommandType, flags: Vec<FlagType>) -> Self{
        return InvocationToken { command_type: command_type, flags: flags }
    }

    pub fn get_type(&self) -> CommandType{
        return self.command_type.clone()
    }

    pub fn get_flags(&self) -> Vec<FlagType>{
        return self.flags.clone()
    }
}

#[derive(Clone, Debug)]
pub struct InvocationPair{
    flag: FlagType,
    object: TokenObject
}
impl InvocationPair{
    pub fn new(flag: FlagType, object: TokenObject) -> Self{
        return InvocationPair{ flag:flag, object:object }
    }
}