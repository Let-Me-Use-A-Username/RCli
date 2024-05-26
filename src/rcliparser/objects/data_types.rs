use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
///Data types used by the invoker
pub enum Data{
    //Simple data types
    DataType(String, DataType),
    //most functions
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DataType{
    String, 
    Path, 
    VectorString,
    VectorPath
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
}
