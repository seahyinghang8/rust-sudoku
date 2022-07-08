use std::{env, process};

fn main() {
    let mut args = env::args();
    args.next();

    let filename = match args.next() {
        Some(arg) => arg,
        None => {
            eprintln!("Didn't get a filename string");
            process::exit(1);
        }   
    };

    if let Err(e) = sudoku::run(&filename) {
        eprint!("Application error: {}", e);
        process::exit(1);
    };
}
