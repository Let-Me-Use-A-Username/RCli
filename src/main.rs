#[warn(non_snake_case)]

mod rcliparser;

fn main() {
    println!("HJello");
    rcliparser::parser::parse("create readme.txt".to_string());
}
