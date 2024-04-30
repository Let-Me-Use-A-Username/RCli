#[derive(PartialEq, Debug, Clone, Eq)]
pub enum TokenCommands{
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
    DIRECTORY(String)
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

trait GetValue{
    fn get_value(&self) -> String;
}

impl GetValue for TokenObjects{
    fn get_value(&self) -> String{
        match self{
            TokenObjects::FILE(file) => file.to_string(),
            TokenObjects::DIRECTORY(dir) => dir.to_string(),
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