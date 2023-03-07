// https://rinthel.github.io/rust-lang-book-ko/ch06-02-match.html
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
	
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
