use std::path::Path;
use std::{collections::HashMap, fs::File};
use std::fmt;
use serde::{Deserialize, Serialize};

const GRAMMAR_PATH: &str = "src\\rcliparser\\utils\\grammar.json";

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub enum CommandType{
    Core,
    Sub,
    Object,
    Flag,
    INVALID
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct Command{
    pub command: Vec<String>,
    pub next: Vec<CommandType>,
    pub is_terminal: bool
}

impl fmt::Debug for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Command {{ commands: {:?}, next: {:?}, is_terminal: {:?}}}", 
            self.command, self.next, self.is_terminal)
    }
}


pub fn load_grammar() -> HashMap<CommandType, Command>{
    let grammar_path = Path::new(GRAMMAR_PATH);
    let grammar_file = File::open(grammar_path).unwrap();
    let grammar: serde_json::Value = serde_json::from_reader(grammar_file).unwrap();
    let mut commands: HashMap<CommandType, Command> = HashMap::new();

    let core = Command {
        command: serde_json::from_value(grammar["command_type"]["core"]["commands"].clone()).unwrap(),
        next: create_chain(serde_json::from_value(grammar["command_type"]["core"]["next"].clone()).unwrap()),
        is_terminal:serde_json::from_value::<bool>(grammar["command_type"]["core"]["isTerminal"].clone()).unwrap()
    };

    let sub = Command {
        command: serde_json::from_value(grammar["command_type"]["sub"]["commands"].clone()).unwrap(),
        next: create_chain(serde_json::from_value(grammar["command_type"]["sub"]["next"].clone()).unwrap()),
        is_terminal:serde_json::from_value::<bool>(grammar["command_type"]["sub"]["isTerminal"].clone()).unwrap()
    };

    let object = Command {
        command: serde_json::from_value(grammar["command_type"]["object"]["commands"].clone()).unwrap(),
        next: create_chain(serde_json::from_value(grammar["command_type"]["object"]["next"].clone()).unwrap()),
        is_terminal:serde_json::from_value::<bool>(grammar["command_type"]["object"]["isTerminal"].clone()).unwrap()
    };

    let flag = Command {
        command: serde_json::from_value(grammar["command_type"]["flag"]["commands"].clone()).unwrap(),
        next: create_chain(serde_json::from_value(grammar["command_type"]["flag"]["next"].clone()).unwrap()),
        is_terminal:serde_json::from_value::<bool>(grammar["command_type"]["flag"]["isTerminal"].clone()).unwrap()
    };

    commands.insert(CommandType::Core, core);
    commands.insert(CommandType::Sub, sub);
    commands.insert(CommandType::Object, object);
    commands.insert(CommandType::Flag, flag);


    return commands;
    
}

fn create_chain(input: Vec<String>) -> Vec<CommandType>{
    let mut next_commands = Vec::<CommandType>::new();
    
    for command in input{
        match command.as_str() {
            "sub" => {
                next_commands.push(CommandType::Sub)
            },
            "object" => {
                next_commands.push(CommandType::Object)
            },
            "flag" => {
                next_commands.push(CommandType::Flag)
            },
            _ => {
                next_commands.push(CommandType::INVALID)
            }
        }
    }
    return next_commands;
}