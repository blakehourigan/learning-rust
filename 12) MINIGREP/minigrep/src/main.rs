use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = match Config::new(&args) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("error: {e}");
            process::exit(1)
        }
    };

    if let Err(e) = minigrep::run(config) {
        eprintln!("problem opening file contents: {e}");
        process::exit(1)
    };
}
