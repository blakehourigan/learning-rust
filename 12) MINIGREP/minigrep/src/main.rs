use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|e| {
        eprintln!("error: {e}");
        process::exit(1)
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("problem opening file contents: {e}");
        process::exit(1)
    };
}
