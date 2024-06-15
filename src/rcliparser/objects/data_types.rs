use std::{collections::VecDeque, path::{Path, PathBuf}};

#[derive(Debug, Clone)]
pub enum Data{
    /* 
        Parser objects. Invoker input data types.
        DataVector may be Invoker output in specific functions.
    */
    SimpleData(String),
    DataVector(Box<VecDeque<Data>>),
    /* 
        Invoker output types
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
            Data::StringData(string) => {
                return Some(string)
            },
            _ => return None
        }
    }
}
