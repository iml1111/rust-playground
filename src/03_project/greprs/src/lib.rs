use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

// 여기만 pub만 다 따라오나?
pub struct Config {
    pub query: String,
    pub filename: String,
}
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        // 가장 쉽고(참조의 생명주기를 관리하지 않아) 약간 비효율적인 방법: clone 메소드 
        // 참조에 비해 약간 더 많은 비용과 메모리가 소비.
        if args.len() < 3 {
            return Err("not enough args");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
	let mut f = File::open(config.filename)?;

	let mut contents = String::new();
	f.read_to_string(&mut contents)?;

    println!("Text: \n{}", contents);

    Ok(())
}

// https://rinthel.github.io/rust-lang-book-ko/ch12-04-testing-the-librarys-functionality.html 
#[cfg(test)]
mod test {
    use super::*;

    *[test]
    fn one_result() {
        let query = "duct";
        let contents = ""
    }
}