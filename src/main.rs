mod rcliparser;
mod rcliterminal;
mod rclilogger;
mod rclishell;

fn main() {
    let shell = rclishell::shell::Shell::new();
    shell.run();
}
