use std::io;

use aoc{{year}}_{{day}}_{{round}}::{% raw %}{{% endraw %}{{main-function}}, Result};

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let lines = io::stdin().lines().filter_map(|l| l.ok());
    let answer = {{main-function}}(lines)?;
    println!("Answer: {answer}");
    Ok(())
}
