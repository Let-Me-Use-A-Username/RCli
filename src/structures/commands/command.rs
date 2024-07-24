use std::collections::HashSet;

use crate::structures::token::Token;

use super::{flag::Flag, status::Status};

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum Command{
    Cd,                     //change working directory
    Pwd,                    //print working directory 
    Ls,                     //list directory
    Move,                   //move or rename file
    Copy,                   //copy file 
    Remove,                 //remove file or directory
    Touch,                  //create empty file
    MkDir,                  //create directory
    Find,                   //search system form file 
    Zip,                    //zip folder
    Unzip,                  //unzip folder

    Echo,                   //print text
    Read,                   //display file content
    Grep,                   //search for a regex pattern
    Sed,                    //search, replace or delete pattern (file)
    Awk,                    //search and manipulate file through regex
    Sort,                   //sort output based on some regex or flag
    Head,                   //display first N lines
    Tail,                   //display last N lines
    Diff,                   //compare two files content and differences

    Export,                 //export a variable
    Alias,                  //create an alias command. Export must be used to remember
    Unalias,                //remove alias
    Shortcut,               //shortcut to executables
    Sudo,                   //activate admin priviliged

    Df,                     //display system disk usage
    Jobs,                   //display running processes
    Kill,                   //terminate process
    Shutdown,               //shutdown  system

    Curl,                   //data transfer between servers

    Note                    //opens default notepad
}

///Command Functionality
trait Executable {
    fn execute<T>(&self) -> Status<T>;
}


/*
    Command structures https://linux.die.net/man/1/ 
*/
pub struct Cd{
    parameter: Token,
}

pub struct Pwd{}

pub struct Ls{
    parameter: Token,
    flags: Vec<Flag>
}

pub struct Move{
    parameter: Token,
    flags: Vec<Flag>
}

pub struct Copy{
    parameter: Token,
    flags: Vec<Flag>
}

pub struct Remove{
    parameter: Token,
    flags: Vec<Flag>
}

pub struct Touch{
    parameter: Token,
    flags: Vec<Flag>
}

pub struct MkDir{
    parameter: Token,
    flags: Vec<Flag>
}

pub struct Find{
    parameter: Token,
    flags: Vec<Flag>
}

pub struct Zip{
    parameter: Token,
    flags: Vec<Flag>
}

pub struct Unzip{
    parameter: Token,
    flags: Vec<Flag>
}


pub struct Echo{
    parameter: Token,
    flags: Vec<Flag>
}

pub struct Read{
    parameter: Token,
    flags: Vec<Flag>
}

pub struct Grep{
    parameter: Token,
    flags: Vec<Flag>
}

pub struct Sed{
    parameter: Token,
    flags: Vec<Flag>
}

pub struct Awk{
    parameter: Token,
    flags: Vec<Flag>
}


pub struct Sort{
    parameter: Token,
    flags: Vec<Flag>
}

pub struct Head{
    parameter: Token,
    flags: Vec<Flag>
}

pub struct Tail{
    parameter: Token,
    flags: Vec<Flag>
}

pub struct Diff{
    parameter: Token,
    flags: Vec<Flag>
}

pub struct Export{
    parameter: Token,
    flags: Vec<Flag>
}

pub struct Alias{
    parameter: Token,
    flags: Vec<Flag>
}

pub struct Unalias{
    parameter: Token,
    flags: Vec<Flag>
}

pub struct Shortcut{
    parameter: Token,
    flags: Vec<Flag>
}

pub struct Sudo{
    parameter: Token,
    flags: Vec<Flag>
}

pub struct Df{
    parameter: Token,
    flags: Vec<Flag>
}

pub struct Jobs{
    parameter: Token,
    flags: Vec<Flag>
}

pub struct Kill{
    parameter: Token,
    flags: Vec<Flag>
}

pub struct Shutdown{
    parameter: Token,
    flags: Vec<Flag>
}

pub struct Curl{
    parameter: Token,
    flags: Vec<Flag>
}

pub struct Note{
    parameter: Token,
    flags: Vec<Flag>
}