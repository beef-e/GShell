use std::process::Command;
use std::io::{stdin, stdout, Write};

fn main() {
    
    println!("Hello, world!");
    println!("Welcome into the Gshell");

    loop{    
        // use the `>` character as the prompt
        // need to explicitly flush this to ensure it prints before read_line
        // flush prints the output instantly
        print!("> ");
        stdout().flush();

        

        let mut input=String::new();
        stdin().read_line(&mut input).unwrap();
        // unwrap() says to return the value of the instruction

        // read_line leaves a trailing newline, which trim removes
        // let cmd = input.trim();


        // erything after the first whitespace is separated and treated as an argument
        // split_whitespace splits using the whitespace as dividing character
        // parts is an iterator
        let mut parts = input.trim().split_whitespace();
        // next returns the next element (in this case, element 0)
        // and removes it from parts
        let cmd = parts.next().unwrap();
        let args = parts;

        let mut child = Command::new(cmd)
            .args(args)
            .spawn()
            .unwrap();
        
        child.wait();
    }
}
