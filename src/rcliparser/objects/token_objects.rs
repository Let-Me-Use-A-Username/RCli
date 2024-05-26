use super::{data_types::DataType, grammar_objects::{CommandType, FlagType, PipeType}};

///Trait to get a value from a Token.
pub trait GetValue{
    fn get_value(&self) -> &String;
}

pub trait GetType{
    fn get_type(&self) -> Option<&DataType>;
}

///Hieghest hierarchical Token.
#[derive(Clone, Debug)]
pub enum Token{
    InvocationToken(InvocationToken),
    InvocationFlag(InvocationFlag),
    InvocationPair(InvocationPair),
    InvocationPipe(InvocationPipe),

    TokenCommand(TokenCommand),
    TokenObject(TokenObject),
    TokenFlag(TokenFlag),
    TokenPipe(TokenPipe)
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
            Token::TokenPipe(pipe) => {
                return pipe.get_value()
            }
            _ => unreachable!()
        }
    }
}
impl GetType for Token{
    fn get_type(&self) -> Option<&DataType> {
        match self{
            Token::TokenObject(TokenObject::DATAOBJECT(_, t)) => {
                return Some(t)
            },
            _ => None
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TokenObject{
    OBJECT(String),
    DATAOBJECT(String, DataType)
}
impl GetValue for TokenObject{
    fn get_value(&self) -> &String {
        match self{
            TokenObject::OBJECT(obj) => {
                return obj
            },
            TokenObject::DATAOBJECT(obj, _) => {
                return obj
            }
        }
    }
}
impl GetType for TokenObject{
    fn get_type(&self) -> Option<&DataType> {
        match self{
            TokenObject::DATAOBJECT(_, t) => {
                return Some(t)
            },
            _ => None
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
pub enum TokenPipe{
    PIPE(String)
}
impl GetValue for TokenPipe{
    fn get_value(&self) -> &String {
        match self{
            TokenPipe::PIPE(pipe) => {
                return pipe
            },
        }
    }
}


/* 
    Invocation Tokens
*/


///Token used by invoker to invoke a commadn.
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


///Used by invoker to invoke flag objects.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InvocationFlag{
    flag: FlagType
}
impl InvocationFlag {
    pub fn new(flag: FlagType) -> Self{
        return InvocationFlag{ flag:flag }
    }
    pub fn get_type(&self) -> FlagType{
        return self.flag.clone()
    }
}


///Used by invoker to invoke flag-object pairs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InvocationPair{
    flag: FlagType,
    object: TokenObject
}
impl InvocationPair{
    pub fn new(flag: FlagType, object: TokenObject) -> Self{
        return InvocationPair{ flag:flag, object:object }
    }

    pub fn get_type(&self) -> FlagType{
        return self.flag.clone()
    }

    pub fn get_object(&self) -> TokenObject{
        return self.object.clone()
    }

}


//Used by invoker to invoke piping
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InvocationPipe{
    pipe: PipeType
}
impl InvocationPipe{
    pub fn new(pipe: PipeType) -> Self{
        return InvocationPipe{ pipe:pipe }
    }

    pub fn get_type(&self) -> &PipeType{
        return &self.pipe
    }
}