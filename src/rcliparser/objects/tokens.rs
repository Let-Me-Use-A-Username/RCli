use super::bnf_commands::InvocationCommand;

#[derive(PartialEq, Debug, Clone, Eq)]
pub enum TokenCommands{
    CWD,
    TOUCH,
    MKDIR,
    DELETE,
    COPY,
    MOVE,
    READ,
    LIST,
    CD,
    EXIT,
    INVALID
}

#[derive(PartialEq, Debug, Clone, Eq)]
pub enum TokenObjects{
    FILE(String),
    DIRECTORY(String),
    OBJECT(String)
}


#[derive(PartialEq, Debug, Clone, Eq, Copy)]
pub enum FlagType{
    TERMINAL,
    NONTERMINAL
}


#[derive(PartialEq, Debug, Clone, Eq)]
pub enum TokenFlag{
    FLAG(FlagType, String),
    FlagType(FlagType)
}


#[derive(PartialEq, Debug, Clone, Eq,)]
pub enum Tokens{
    TokenCommands(TokenCommands),
    TokenObjects(TokenObjects),
    TokenFlag(TokenFlag)
}

pub trait GetTokenFromString{
    fn get_token_command(invocation_command: InvocationCommand) -> Option<TokenCommands>;
}

impl GetTokenFromString for Tokens {
    fn get_token_command(invocation_command: InvocationCommand) -> Option<TokenCommands> {
        let mut iterator = invocation_command.invocation_name.into_iter();
        
        loop{
            match iterator.next().unwrap().to_lowercase().as_str(){
                "cwd" => {
                    return Some(TokenCommands::CWD)
                }
                "touch" => {
                    return Some(TokenCommands::TOUCH)
                },
                "mkdir" => {
                    return Some(TokenCommands::MKDIR)
                },
                "delete" | "del" => {
                    return Some(TokenCommands::DELETE)
                },
                "copy" | "cp" => {
                    return Some(TokenCommands::COPY)
                },
                "move" | "mv" => {
                    return Some(TokenCommands::MOVE)
                },
                "read" => {
                    return Some(TokenCommands::READ)
                },
                "list" | "ls" => {
                    return Some(TokenCommands::LIST)
                },
                "cd" => {
                    return Some(TokenCommands::CD)
                },
                "exit" => {
                    return Some(TokenCommands::EXIT)
                }
                _ => {
                    return Some(TokenCommands::INVALID);
                }
            }
        }
    }
}

pub trait GetValue{
    fn get_value(&self) -> String;
}

impl GetValue for TokenObjects{
    fn get_value(&self) -> String{
        match self{
            TokenObjects::FILE(file) => file.to_string(),
            TokenObjects::DIRECTORY(dir) => dir.to_string(),
            TokenObjects::OBJECT(obj) => obj.to_string(),
        }
    }
}

impl GetValue for TokenFlag{
    fn get_value(&self) -> String{
        match self{
            TokenFlag::FLAG(f_type, f_value) => f_value.to_string(),
            _ => unreachable!()
        }
    }
}

//Trait to downcast tokens to tokencommand enum in order to extract string value
impl TryFrom<Tokens> for TokenCommands{
    type Error = &'static str;  

    fn try_from(value: Tokens) -> Result<Self, Self::Error> {
        match value{
            Tokens::TokenCommands(TokenCommands::CWD) => {
                Ok(TokenCommands::CWD)
            },
            Tokens::TokenCommands(TokenCommands::TOUCH) => {
                Ok(TokenCommands::TOUCH)
            },
            Tokens::TokenCommands(TokenCommands::MKDIR) => {
                Ok(TokenCommands::MKDIR)
            },
            Tokens::TokenCommands(TokenCommands::DELETE) => {
                Ok(TokenCommands::DELETE)
            },
            Tokens::TokenCommands(TokenCommands::COPY) => {
                Ok(TokenCommands::COPY)
            },
            Tokens::TokenCommands(TokenCommands::MOVE) => {
                Ok(TokenCommands::MOVE)
            },
            Tokens::TokenCommands(TokenCommands::READ) => {
                Ok(TokenCommands::READ)
            },
            Tokens::TokenCommands(TokenCommands::LIST) => {
                Ok(TokenCommands::LIST)
            },
            Tokens::TokenCommands(TokenCommands::CD) => {
                Ok(TokenCommands::CD)
            },
            Tokens::TokenCommands(TokenCommands::EXIT) => {
                Ok(TokenCommands::EXIT)
            },
            Tokens::TokenCommands(TokenCommands::INVALID) => {
                Ok(TokenCommands::INVALID)
            },
            _ => {
                unreachable!()
            }
        }
    }
}

//Trait to downcast tokens to tokenobject enum in order to extract string value
impl TryFrom<Tokens> for TokenObjects{
    type Error = &'static str;  

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

//Trait to downcast tokens to tokenflag enum in order to extract string value
impl TryFrom<Tokens> for TokenFlag{
    type Error = &'static str;  

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

#[derive(Clone, Debug)]
pub enum FlagObjectPair{
    PAIR(TokenFlag, TokenObjects),
    SOLE(TokenFlag)
}

//Trait to downcast pair to tokenflag enum in order to extract string value
impl TryFrom<FlagObjectPair> for TokenFlag{
    type Error = &'static str;  

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
            },
            _ => {
                unreachable!()
            }
        }
    }
}

//Trait to downcast pair to tokenobject enum in order to extract string value
impl TryFrom<FlagObjectPair> for TokenObjects{
    type Error = &'static str;  

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