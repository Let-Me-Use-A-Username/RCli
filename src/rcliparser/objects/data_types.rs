use std::{fs::{DirEntry, File}, path::PathBuf};

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