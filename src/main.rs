use std::{io, time, io::Write};

fn main() {
    // probaby better to have this as a global in the future
    // probably be good to have this as an array so the match statement (or equivalent) can be done programmatically 
    // let options = "[a: 1] [b: 5] [c: 10] [d: 15] [exit]";

    println!("Welcome to pomodoro!"); // One time startup messsage

    loop { // main application loop
        let input = get_input(
            "Choose from the following timer lengths or enter a custom time:\n[a: 1] [b: 5] [c: 10] [d: 15] [exit]"
        );
        
        match input.as_str() {
            "exit" => break,
            "a" => start_timer(1),
            "b" => start_timer(5),
            "c" => start_timer(10),
            "d" => start_timer(15),
            _ => println!("Try again!") // haven't implemented custom times yet
        }
    }
}

fn get_input(prompt: &str) -> String {
    let mut line = String::new(); // initialize output
    println!("{prompt}");

    io::stdin().read_line(&mut line).expect("bad input");

    return line.trim().to_string();

}

// macro needs to appear before it's used in the code, but not functions
macro_rules! print_flush{
    ($input:expr) =>  {
        print!($input);
        io::stdout().flush().expect("erro flushing");
    }
}

fn start_timer(duration: u64) {
    let start = time::Instant::now();

    for t in (0..duration + 1).rev() {
        while time::Instant::now() < start + time::Duration::new(duration - t, 0) {} // do nothing/wait for one second
        // !TODO measure length of output
        // for not, just assuming that no number will be more than 5 digits
        let clear = " ".repeat(5);
        print_flush!("\r{clear}\r{t}");
        // print!("\r{}", " ".repeat(5)); // clear previous output
        // print!("\r{t}"); // print current number
        // io::stdout().flush().expect("error flushing");
    }
    println!(); // print newline
}
