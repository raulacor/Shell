#[allow(unused_imports)]
#[allow(unconditional_recursion)]
use std::io::{self, Write};

fn main() {
    print!("$ ");
    io::stdout().flush().unwrap();
    // Wait for user input
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!("{}: command not found", input.trim());
    main();
}
