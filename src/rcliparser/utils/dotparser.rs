use std::path::{Path, PathBuf};

use crate::rcliterminal::terminal_singlenton::Terminal;


//function that translates dots in a way that canonicalize understands
pub fn parse_path(directory_token: String, terminal_instance: &mut Terminal){
    
    let mut current_dir = PathBuf::from(terminal_instance.get_current_directory());
    current_dir.push(directory_token);

    let path_components = current_dir.components();

    let os_strings: Vec<_> = path_components.clone().map(|comp| comp.as_os_str()).collect();

    println!("OS Strings {os_strings:?}\n");

    for component in path_components{
        match component {
            std::path::Component::Prefix(pref) => {
                println!("Prefix:{pref:?}");
            },
            std::path::Component::RootDir => {
                println!("RootDir:{component:?}");
            },
            std::path::Component::CurDir => {
                println!("CurrentDir:{component:?}");
            },
            std::path::Component::ParentDir => {
                println!("ParentDir:{component:?}");
                if os_strings.len() == 1 {
                    //return terminal_instance.get_current_directory().parent();
                }
            },
            std::path::Component::Normal(n) => {
                println!("Normal:{n:?}")
            },
        }
    }
}