use std::process::Command;

fn main() {
    Command::new("tsc").status().unwrap();
}