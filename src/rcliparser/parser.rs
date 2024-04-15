#[warn(unused_imports)]
use super::input_reader::accept_input;

use super::lexical_analyzer::analyze;


pub fn parse(user_input: String){
    let mut input = accept_input(user_input.as_str());
    let tokens = analyze(&mut input);
    //create_tree(tokens);
}

// fn create_tree(tokens: Vec<Tokens>){
//     let node = Node("data");
//     let tree = Tree::new(node);
// }

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_parse(){
        parse("create readme.txt".to_string());
        parse("create ./path/to/readme.txt".to_string());
        parse("list path/to/file".to_string());
        parse("list path/to/file --hidden".to_string());
        parse("list --hidden".to_string());
    }
    
}