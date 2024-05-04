use std::path::{Path, PathBuf};

use crate::rcliterminal::terminal_singlenton::Terminal;


/*
Todo! Currently when trying to execute cd .. to parent dir where parent dir is 
root, it doesn't execute, however cd ../ works. The current workaround is when operation 
fails we return the parent if it exists
*/
pub fn parse_dir(directory: &Path, terminal_instance: &mut Terminal) -> PathBuf{

    let target_dir = directory.canonicalize();
    let current_dir = terminal_instance.get_current_directory();

    let op: PathBuf = match target_dir {
        Ok(target) => target,
        Err(error) => {
            if directory.eq(&PathBuf::from("..")){
                
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
Function that based on a paths components return a hard coded path. This is done due to 
how windows addresses paths like C:/ When trying to traverse with .. canonicalize fails to 
understand the path, so we hardcode a return type
*/
pub fn get_path(path: PathBuf, attr: i32) -> PathBuf{
    let components = path.components();
    
    let p = path.parent();
    match attr{
        //Prefix + Rootdir | Prefix + Root + Parent |Prefix + Root + Normal | Prefix + Root + + Curent + Parent
        3 | 7 | 8 | 10 => {
            if p.is_some(){
                return p.unwrap().to_path_buf()
            }
        },
        //Current Dir
        6 => {
            return path
        },
        //Normal
        15 => {
            let com = &components.last().unwrap();
            let normal_path = Path::new(com);
            return PathBuf::from(normal_path)
        },
        _ => return path.to_path_buf()
    }
    return path
}

/*
Function that returns path componets
*/
pub fn get_path_components(mut path: PathBuf, terminal_instance: &mut Terminal) -> (PathBuf, i32){
    
    let mut comp = 0;

    path = match path.canonicalize() {
        Ok(op) => {
            op
        },
        Err(_) => {
            eprintln!("Failed to canonicalize path");
            terminal_instance.get_current_directory()
        },
    };

    println!("Path canonicalized {path:?}");

    let path_components = path.components();
    
    for component in path_components{
        match component {
            std::path::Component::Prefix(pref) => {
                comp += 1;
                //println!("Prefix:{pref:?}");
            },
            std::path::Component::RootDir => {
                comp += 2;
                //println!("RootDir:{component:?}");
            },
            std::path::Component::CurDir => {
                comp += 3;
                //println!("CurrentDir:{component:?}");
            },
            std::path::Component::ParentDir => {
                comp += 4;
                //println!("ParentDir:{component:?}");
            },
            std::path::Component::Normal(n) => {
                comp += 5;
                //println!("Normal:{n:?}")
            },
        }
    }
    return (path, comp)
}