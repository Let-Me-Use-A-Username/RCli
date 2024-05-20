use std::{fs::{DirEntry, File}, path::PathBuf};

#[derive(Debug)]
pub enum Data{PathData(PathBuf),
    StringData(String),
    VecStringData(Vec<String>),
    FileData(File),
    DirPathData(Vec<PathBuf>),
    DirEntryData(Vec<DirEntry>),
    StatusData(i32)
}