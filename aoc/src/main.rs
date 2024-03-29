use std::io;

use aoc{{year}}_{{day}}_{{round}}::{{main-function}};

fn main() {
    tracing_subscriber::fmt::init();
    let lines = io::stdin().lines().filter_map(|l| l.ok());
    let answer = {{main-function}}(lines);
    println!("Answer: {answer}");
}
