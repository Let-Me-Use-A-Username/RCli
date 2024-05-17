use super::grammar_objects::{CommandType, FlagType};

pub trait GetValue{
    fn get_value(&self) -> &String;
}

pub enum Token{
    InvocationToken(InvocationToken),

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


pub struct InvocationToken{
    command_type: CommandType,
    flags: Vec<FlagType>
}

impl InvocationToken{
    pub fn new(command_type: CommandType, flags: Vec<FlagType>) -> Self{
        return InvocationToken { command_type: command_type, flags: flags }
    }

    pub fn get_type(&self) -> CommandType{
        return self.command_type
    }

    pub fn get_flags(&self) -> Vec<FlagType>{
        return self.flags
    }
}