use std::env;
use std::process;

use grist;
use grist::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Wuhoh! There was an error: {}", err);
        process::exit(1);
    });

    if let Err(e) = grist::run(config) {
        eprintln!("Application Error: {}", e);
        process::exit(1);
    }
}
