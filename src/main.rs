#[warn(non_snake_case)]

mod rcliparser;
mod rcliterminal;

fn main() {
    rcliterminal::terminal::start_terminal();
}
