use std::{fs::{DirEntry, File}, path::PathBuf};

#[derive(Debug)]
pub enum Data{
    PathData(PathBuf),
    FileData(File),
    DirVecData(Vec<DirEntry>),
    StatusData(i32),
    BufferedStringData(String)
}