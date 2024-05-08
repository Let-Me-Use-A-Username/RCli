use std::path::{Path, PathBuf};

use crate::rcliterminal::terminal_singlenton::Terminal;

/*
Function that returns path componets. Mimics way windows handles attributes
*/
pub fn get_path_components(mut path: PathBuf, terminal_instance: &mut Terminal) -> (PathBuf, i32){
    
    let mut comp = 0;

    path = match path.canonicalize() {
        Ok(op) => {
            op
        },
        Err(_) => {
            eprintln!("Failed to canonicalize path {path:?}");
            terminal_instance.get_current_directory()
        },
    };

    let path_components = path.components();
    
    for component in path_components{
        match component {
            std::path::Component::Prefix(pref) => {
                comp += 1;
                println!("Prefix:{pref:?}");
            },
            std::path::Component::RootDir => {
                comp += 2;
                println!("RootDir:{component:?}");
            },
            std::path::Component::CurDir => {
                comp += 4;
                println!("CurrentDir:{component:?}");
            },
            std::path::Component::ParentDir => {
                comp += 8;
                println!("ParentDir:{component:?}");
            },
            std::path::Component::Normal(n) => {
                comp += 10;
                println!("Normal:{n:?}")
            },
        }
    }
    return (path, comp)
}