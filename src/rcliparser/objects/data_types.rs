use std::{fs::{DirEntry, File}, path::{Path, PathBuf}};

pub enum GenericData<T>{
    Data(T)
}

#[derive(Debug)]
pub enum Data{
    PathData(PathBuf),
    StringData(String),
    VecStringData(Vec<String>),
    FileData(File),
    DirPathData(Vec<PathBuf>),
    DirEntryData(Vec<DirEntry>),
    StatusData(i32)
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