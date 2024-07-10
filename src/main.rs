use std::process::Command;
use std::io::{stdin, stdout, Write};
use std::env;
use std::path::Path;

fn main() {
    
    println!("Hello, world!");
    println!("Welcome into the Gshell");

    loop{    
        // use the `>` character as the prompt
        // need to explicitly flush this to ensure it prints before read_line
        // flush means to print the output instantly
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


        match cmd {
            "cd"=> {
                // Default to home if no directory is provided
                //
                // args is an iterator (from parts)
                // peekable() makes it a peekable iterator, so we can peek() the next element
                // without consuming it
                //
                // peekable() returns a Peekable<args> (args is the type of my original
                // iterator)
                // peek() returns a Option<&T>, where T is the type of the iterator
                
                // map_or is a method of the Option type, evaluating its value and returning the
                // default value ("/") if given "None" or using it if it exists.
                //
                // In this case, it takes the value of X (similar to C pointers) and returns the
                // actual value
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root  = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e);
                }
            },

            "exit" => return,

            command => {
                let mut child = Command::new(cmd)
                .args(args)
                .spawn()
                .unwrap();
        
                child.wait();
            }
        }
    }
}
