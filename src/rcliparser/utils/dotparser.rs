use std::path::{self, Path, PathBuf};

use crate::rcliterminal::terminal_singlenton::Terminal;


/*
Todo! Currently when trying to execute cd .. to parent dir where parent dir is 
root, it doesn't execute, however cd ../ works. The current workaround is when operation 
fails we return the parent if it exists
    */
pub fn parse_root_dir(directory_token: &Path, terminal_instance: &mut Terminal) -> PathBuf{

    let target_dir = directory_token.canonicalize();
    let current_dir = terminal_instance.get_current_directory();

    let op: PathBuf = match target_dir {
        Ok(target) => target,
        Err(error) => {
            if directory_token.eq(&PathBuf::from("..")){
                
                let parent = current_dir.parent();

                if parent.is_some(){
                    return parent.unwrap().to_path_buf()
                }
                eprintln!("INTERNAL ERROR: {error}");
                return current_dir
            }
            return current_dir
        }
    };

    return op
}

/*
Function that returns root for terminal_instance
*/
pub fn get_root(mut path: PathBuf, terminal_instance: &mut Terminal){

    path = match path.canonicalize() {
        Ok(op) => {
            op
        },
        Err(_) => {
            eprintln!("Failed to canonicalize path");
            terminal_instance.get_current_directory()
        },
    };

    let path_components = path.components();
    let os_strings: Vec<_> = path_components.clone().map(|comp| comp.as_os_str()).collect();
    
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