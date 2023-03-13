use std::env;
use std::process;

use mini_grep::Config;
use mini_grep::run;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing the argument {}", err);
        process::exit(1);
    });

    println!("Searching for :{}", config.query);
    println!("in File :{}", config.filename);

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
