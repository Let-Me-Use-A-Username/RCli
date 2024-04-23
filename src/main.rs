#[warn(non_snake_case)]

mod rcliparser;

fn main() {
    println!("HJello");
    rcliparser::parser::match_parse("list C:/Users/AlexanderMcLean/Desktop".to_string())
}
