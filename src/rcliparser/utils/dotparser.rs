use std::path::Path;

use crate::rcliparser::objects::tokens::TokenObjects;


//function that translates dots in a way that canonicalize understands
pub fn parse_path(directory_token: TokenObjects){
    let string_path = match directory_token{
        TokenObjects::DIRECTORY(dir) => dir,
        TokenObjects::FILE(file) => file,
        _ => unreachable!()
    }.to_owned();

    let path = Path::new(&string_path);
    
    for component in path.components(){
        match component {
            std::path::Component::Prefix(pref) => println!("Prefix:{pref:?}"),
            std::path::Component::RootDir => println!("RootDir:{component:?}"),
            std::path::Component::CurDir => println!("CurrentDir:{component:?}"),
            std::path::Component::ParentDir => println!("ParentDir:{component:?}"),
            std::path::Component::Normal(n) => println!("Normal:{n:?}"),
        }
    }
}