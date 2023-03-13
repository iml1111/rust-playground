// https://rinthel.github.io/rust-lang-book-ko/ch12-00-an-io-project.html
extern crate greprs;

use std::env;
use std::process;
use greprs::Config;


fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Config problem: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);
    
    if let Err(e) = greprs::run(config) {
        println!("Applicaiton error: {}", e);
        process::exit(1);
    }
}
