use minigrep::{run, Config};
use std::env;
use std::process;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Search for {} in {}", config.query, config.file_path);
    if let Err(e) = run(config) {
        eprintln!("Application Error: {e}");
        process::exit(1)
    }
}
