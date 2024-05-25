use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
///Data types used by the invoker
pub enum Data{
    //Simple data types
    PathData(PathBuf),
    //read
    StringData(String),
    //grep
    VecStringData(Vec<String>),
    //list
    DirPathData(Vec<PathBuf>),
    //cd
    StatusData(i32),

    //complex data types
    DataVector(Box<Vec<Data>>)
}

impl Data{
    pub fn get_path(&self) -> Option<&Path>{
        match &self{
            Data::PathData(path) => {
                return Some(path.as_path())
            },
            _ => return None
        }
    }

    pub fn get_string(&self) -> Option<&String>{
        match &self{
            Data::StringData(string) => {
                return Some(string)
            },
            _ => return None
        }
    }
}
