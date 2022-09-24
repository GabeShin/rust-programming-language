use std::env;
use std::process;
use minigrep::{Config, run};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    
    println!("Search for {} in {}", config.query, config.file_path);
    if let Err(e) = run(config) {
        eprintln!("Application Error: {e}");
        process::exit(1)
    }
}