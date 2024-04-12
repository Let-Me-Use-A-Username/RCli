use std::{collections::HashMap, fs::File};
use std::fmt;
use serde::{Deserialize, Serialize};

const GRAMMAR_PATH: &str = "src\\rcliparser\\grammar.json";

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub enum CommandType{
    Core,
    Sub,
    Object,
    Flag
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Command{
    pub command_type: CommandType,
    pub command: Vec<String>,
    pub next: Vec<String>,
    pub is_terminal: bool
}

impl fmt::Debug for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Command {{ command_type: {:?}, command: {:?}, next: {:?}, }}", 
            self.command_type, self.command, self.next)
    }
}


pub fn load_grammar() -> HashMap< String, Command>{
    let grammar_file = File::open(GRAMMAR_PATH).unwrap();
    let grammar: serde_json::Value = serde_json::from_reader(grammar_file).unwrap();
    let mut commands: HashMap<String, Command> = HashMap::new();

    let core = Command {
        command_type: CommandType::Core,
        command: serde_json::from_value(grammar["command_type"]["core"]["commands"].clone()).unwrap(),
        next: serde_json::from_value(grammar["command_type"]["core"]["next"].clone()).unwrap(),
        is_terminal:serde_json::from_value::<bool>(grammar["command_type"]["core"]["isTerminal"].clone()).unwrap()
    };

    let sub = Command {
        command_type: CommandType::Sub,
        command: serde_json::from_value(grammar["command_type"]["sub"]["commands"].clone()).unwrap(),
        next: serde_json::from_value(grammar["command_type"]["sub"]["next"].clone()).unwrap(),
        is_terminal:serde_json::from_value::<bool>(grammar["command_type"]["sub"]["isTerminal"].clone()).unwrap()
    };

    let object = Command {
        command_type: CommandType::Object,
        command: serde_json::from_value(grammar["command_type"]["object"]["commands"].clone()).unwrap(),
        next: serde_json::from_value(grammar["command_type"]["object"]["next"].clone()).unwrap(),
        is_terminal:serde_json::from_value::<bool>(grammar["command_type"]["object"]["isTerminal"].clone()).unwrap()
    };

    let flag = Command {
        command_type: CommandType::Flag,
        command: serde_json::from_value(grammar["command_type"]["flag"]["commands"].clone()).unwrap(),
        next: serde_json::from_value(grammar["command_type"]["flag"]["next"].clone()).unwrap(),
        is_terminal:serde_json::from_value::<bool>(grammar["command_type"]["flag"]["isTerminal"].clone()).unwrap()
    };

    commands.insert("core".to_string(), core);
    commands.insert("sub".to_string(), sub);
    commands.insert("object".to_string(), object);
    commands.insert("flag".to_string(), flag);


    return commands;
    
}