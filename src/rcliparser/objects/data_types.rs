use std::{fs::{DirEntry, File}, path::PathBuf};

use super::tokens::{FlagObjectPair, GetValue, TokenFlag, TokenObjects};

#[derive(Debug)]
pub enum Data{
    RegexData(String),//used for grep
    BoolData(String), //used for terminal flags i.e. --hidden, --recursive

    PathData(PathBuf),
    FileData(File),
    DirVecData(Vec<DirEntry>),
    StatusData(i32),
    BufferedStringData(String)
}


impl From<FlagObjectPair> for Data{
    fn from(value: FlagObjectPair) -> Self {
        let flag : TokenFlag = value.clone().try_into().unwrap();
        let obj : TokenObjects = value.try_into().unwrap();

        match flag{
            TokenFlag::FLAG(_, flag_value) => {
                match flag_value.as_str(){
                    "-d" | "-destination" => {
                        Data::PathData(PathBuf::from(obj.get_value()))
                    },
                    "-r" | "-regex" => {
                        Data::RegexData(obj.get_value())
                    },
                    "--hidden" | "--recursive" | "--force"=> {
                        Data::BoolData(obj.get_value())
                    }
                    _ => unreachable!(),
                }
            },
            _ => unreachable!(),
        }
    }
}