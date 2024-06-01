use std::{collections::VecDeque, path::{Path, PathBuf}};

use super::grammar_objects::CommandType;

#[derive(Debug, Clone)]
pub enum Data{
    /* 
        Parser objects 
    */
    SimpleData(String),
    CommandData(CommandType),
    DataVector(Box<VecDeque<Data>>),
    /* 
        Invoker commands return types
    */
    PathData(PathBuf),
    StringData(String),
    VecStringData(Vec<String>),
    DirPathData(Vec<PathBuf>),
    StatusData(i32),
}

impl Data{
    pub fn get_path(&self) -> Option<&Path>{
        match &self{
            Data::SimpleData(path) => {
                let obj = Path::new(path);
                return Some(obj)
            },
            _ => return None
        }
    }

    pub fn get_value(&self) -> Option<&String>{
        match &self{
            Data::SimpleData(path) => {
                return Some(path)
            },
            _ => return None
        }
    }
}
