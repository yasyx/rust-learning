use minigrep::{run, Config};
use std::env;
use std::process;
fn main() {

    let config = Config::from(env::args()).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });

    if let Err(msg) = run(&config) {
        eprintln!("Error: {}", msg);
        process::exit(1);
    }
}
