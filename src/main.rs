use std::process::Command;
use std::io::stdin;

fn main() {
    println!("Hello, world!");
    println!("Welcome into the Gshell");

    let mut input=String::new();
    stdin().read_line(&mut input).unwrap();
    // unwrap() says to return the value of the instruction

    // read_line leaves a trailing newline, which trim removes
    let cmd = input.trim();

    Command::new(cmd)
        .spawn()
        .unwrap();
}
