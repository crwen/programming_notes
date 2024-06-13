use std::{env, process};

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    // unwrap_or_else, defined on Result<T, E>
    // allows us to define some custom, non-panic! error handling
    // return the inner value Ok if Ok; else Err
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("Problem running program: {e}");
        process::exit(1);
    }
}
