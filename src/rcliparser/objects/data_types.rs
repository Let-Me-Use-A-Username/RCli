use std::{fs::{DirEntry, File}, path::{Path, PathBuf}};

#[derive(Debug)]
///Data types used by the invoker
pub enum Data{
    //Simple data types
    PathData(PathBuf),
    StringData(String),
    VecStringData(Vec<String>),
    FileData(File),
    DirPathData(Vec<PathBuf>),
    DirEntryData(Vec<DirEntry>),
    StatusData(i32),

    //complex data types
    ComplexData(Box<Vec<Data>>),
    /*
        productions steps:
        read someFile.txt | touch afile.txt
        StringData(String) | touch Data::PathData::(aFile.txt)
        touch ComplexData( PathData, StringData)
    */
    
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