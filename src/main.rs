#[warn(non_snake_case)]

mod rcliparser;
mod rcliterminal;

fn main() {
    println!("HJello");
    rcliterminal::terminal::start_terminal();

}
