use std::{env, process};
use minigrep_mm::{run, Config};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1)
    });

    println!("Searching for \"{}\"", config.query);
    println!("in file {}", config.file_path);

    if let Err(e) = run(config) {
        eprintln!("{e}");
        process::exit(1);
    }

}
