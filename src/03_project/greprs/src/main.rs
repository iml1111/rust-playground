// https://rinthel.github.io/rust-lang-book-ko/ch12-00-an-io-project.html
extern crate greprs;

use std::env;
use std::process;
use greprs::Config;


fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Config problem: {}", err);
        process::exit(1);
    });
    
    if let Err(e) = greprs::run(config) {
        eprintln!("Applicaiton error: {}", e);
        process::exit(1);
    }
}
