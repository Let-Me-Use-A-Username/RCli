use serde::{Deserialize, Serialize};

///Struct used by the parser. Carries the information from
///bnf_command objects onto the parser.
#[derive(PartialEq, Debug, Clone, Eq)]
pub struct TokenCommands{
    command_type: TokenCommandType,
    token_flags: Vec<String>
}

impl TokenCommands{
    pub fn new(command_type: TokenCommandType, flags: Vec<String>) -> Self{
        return TokenCommands { command_type: command_type, token_flags: flags }
    }

    pub fn get_type(&self) -> TokenCommandType{
        return self.command_type
    }

    pub fn containt_flag(&self, flag: &String) -> bool{
        let con = self.token_flags.iter().any(|val| val.contains(flag));

        return con
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash, Copy)]
pub enum TokenCommandType{
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
    EXIT,
    INVALID
}

///Enum used by the lexer and parser. Mimics available invocable objects.
#[derive(PartialEq, Debug, Clone, Eq)]
pub enum TokenObjects{
    FILE(String),
    DIRECTORY(String),
    OBJECT(String),
    INVALID
}

///Enum used by the lexer and parser to interpret terminal (--) and
///non terminal (-) commands
#[derive(PartialEq, Debug, Clone, Eq, Copy)]
pub enum FlagType{
    TERMINAL,
    NONTERMINAL
}

///Enum that is used by the lexer to qualify flag order.
#[derive(PartialEq, Debug, Clone, Eq)]
pub enum TokenFlag{
    FLAG(FlagType, String),
    FlagType(FlagType)
}

///Enum used by the lexer to provide a Token stream 
///without caring about what is inside (to a certain degree of course)
#[derive(PartialEq, Debug, Clone, Eq,)]
pub enum Tokens{
    TokenCommand(TokenCommands),
    TokenObjects(TokenObjects),
    TokenFlag(TokenFlag)
}

pub trait GetValue{
    fn get_value(&self) -> String;
}

///Get String value from TokenObject
impl GetValue for TokenObjects{
    fn get_value(&self) -> String{
        match self{
            TokenObjects::FILE(file) => file.to_string(),
            TokenObjects::DIRECTORY(dir) => dir.to_string(),
            TokenObjects::OBJECT(obj) => obj.to_string(),
            TokenObjects::INVALID => "INVALID".to_string()
        }
    }
}

///Get String value from TokenFlag::FLAG object
impl GetValue for TokenFlag{
    fn get_value(&self) -> String{
        match self{
            TokenFlag::FLAG(f_type, f_value) => f_value.to_string(),
            _ => unreachable!()
        }
    }
}


impl TryFrom<Tokens> for TokenObjects{
    type Error = &'static str;  
    ///Trait used to downcast Tokens to TokenObjects
    fn try_from(value: Tokens) -> Result<Self, Self::Error> {
        match value{
            Tokens::TokenObjects(TokenObjects::DIRECTORY(dir)) => {
                Ok(TokenObjects::DIRECTORY(dir))
            },
            Tokens::TokenObjects(TokenObjects::FILE(file)) => {
                Ok(TokenObjects::FILE(file))
            },
            Tokens::TokenObjects(TokenObjects::OBJECT(obj)) => {
                Ok(TokenObjects::OBJECT(obj))
            },
            _ => {
                unreachable!()
            }
        }
    }
}


impl TryFrom<Tokens> for TokenFlag{
    type Error = &'static str;  
    ///Trait used to downcast Tokens to TokenFlags::FLAG
    fn try_from(value: Tokens) -> Result<Self, Self::Error> {
        match value{
            Tokens::TokenFlag(TokenFlag::FLAG(flagtype, flagvalue)) => {
                Ok(TokenFlag::FLAG(flagtype, flagvalue))
            },
            _ => {
                unreachable!()
            }
        }
    }
}
/*

PARSER OBJECTS:
Used to pass to invoker a pair of flag and object or a sole object

*/
///Enum used to parse tokenflag-tokenobject pair for the invoker
#[derive(Clone, Debug)]
pub enum FlagObjectPair{
    PAIR(TokenFlag, TokenObjects),
    SOLE(TokenFlag)
}

pub trait GetTupleValue{
    fn get_value(&self) -> (Option<String>, Option<String>);
}


impl GetTupleValue for FlagObjectPair{
    ///Get String value from TokenObject. First value is flag second is object.
    fn get_value(&self) -> (Option<String>, Option<String>){
        match self{
            FlagObjectPair::PAIR(flag, object) => {
                return (Some(flag.get_value()), Some(object.get_value()))
            },
            FlagObjectPair::SOLE(sole_flag) => {
                return (Some(sole_flag.get_value()), None)
            },
        }
    }
}


impl TryFrom<FlagObjectPair> for TokenFlag{
    type Error = &'static str;  
    ///Trait to downcast pair to flag value
    fn try_from(value: FlagObjectPair) -> Result<Self, Self::Error> {
        match value{
            FlagObjectPair::PAIR(flag, _) => {
                match flag{
                    TokenFlag::FLAG(f_type, f_value) => {
                        Ok(TokenFlag::FLAG(f_type, f_value))
                    },
                    TokenFlag::FlagType(f_type) => {
                        Ok(TokenFlag::FlagType(f_type))
                    },
                }
            },
            FlagObjectPair::SOLE(flag) => {
                Ok(flag)
            }
        }
    }
}


impl TryFrom<FlagObjectPair> for TokenObjects{
    type Error = &'static str;  
    ///Trait to downcast pair to object value
    fn try_from(value: FlagObjectPair) -> Result<Self, Self::Error> {
        match value{
            FlagObjectPair::PAIR(_, obj) => {
                Ok(obj)
            },
            _ => {
                unreachable!()
            }
        }
    }
}