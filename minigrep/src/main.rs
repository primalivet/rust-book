use std::env;
use std::process;

use minigrep::Config;

fn main() -> () {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Arguments error: {err}");
        process::exit(1);
    });

    println!("Searching for {:?} in file {:?}", config.query, config.path);

    if let Err(err) = minigrep::run(config) {
        eprintln!("Application error: {err}");
        process::exit(1);
    }
}
