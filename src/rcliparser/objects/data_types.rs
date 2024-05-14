use std::{fs::{DirEntry, File}, path::PathBuf};

pub enum Data{
    PathData(PathBuf),
    FileData(File),
    DirVecData(Vec<DirEntry>),
    StatusData(i32),
    BufferedStringData(String)
}