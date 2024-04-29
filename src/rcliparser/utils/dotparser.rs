use std::path::Path;

use crate::rcliparser::lexical_analyzer::TokenObjects;

//function that translates dots in a way that canonicalize understands
pub fn parse_path(directory_token: TokenObjects){
    let string_path = match directory_token{
        TokenObjects::DIRECTORY(dir) => dir,
        TokenObjects::FILE(file) => file,
        _ => unreachable!()
    }.to_owned();

    let path = Path::new(&string_path);
    
    for component in path.components(){
        println!("{component:?}");
        match component {
            std::path::Component::Prefix(_) => todo!(),
            std::path::Component::RootDir => todo!(),
            std::path::Component::CurDir => todo!(),
            std::path::Component::ParentDir => todo!(),
            std::path::Component::Normal(_) => todo!(),
        }
    }
}