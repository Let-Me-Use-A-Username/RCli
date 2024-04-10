use std::fs::File;
use std::fmt;
use serde::{Deserialize, Serialize};

const GRAMMAR_PATH: &str = "src\\rcliparser\\grammar.json";

#[derive(Serialize, Deserialize)]
pub struct Command{
    command_type: String,
    command: Vec<String>,
    next: Vec<String>
}

impl fmt::Debug for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Command {{ command_type: {}, command: {:?}, next: {:?}, }}", 
            self.command_type, self.command, self.next)
    }
}


pub fn load_grammar() -> Vec<Command>{
    let grammar_file = File::open(GRAMMAR_PATH).unwrap();
    let grammar: serde_json::Value = serde_json::from_reader(grammar_file).unwrap();
    let mut commands: Vec<Command> = Vec::new();

    let core = Command {
        command_type: "core".to_string(),
        command: serde_json::from_value(grammar["core"]["commands"].clone()).unwrap(),
        next: serde_json::from_value(grammar["core"]["next"].clone()).unwrap()
    };

    let sub = Command {
        command_type: "sub".to_string(),
        command: serde_json::from_value(grammar["sub"]["commands"].clone()).unwrap(),
        next: serde_json::from_value(grammar["sub"]["next"].clone()).unwrap()
    };

    let object = Command {
        command_type: "object".to_string(),
        command: serde_json::from_value(grammar["object"]["commands"].clone()).unwrap(),
        next: serde_json::from_value(grammar["object"]["next"].clone()).unwrap()
    };

    let flag = Command {
        command_type: "flag".to_string(),
        command: serde_json::from_value(grammar["flag"]["commands"].clone()).unwrap(),
        next: serde_json::from_value(grammar["flag"]["next"].clone()).unwrap()
    };

    commands.push(core);
    commands.push(sub);
    commands.push(object);
    commands.push(flag);


    return commands;
    
}