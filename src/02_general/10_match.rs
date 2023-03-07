// https://rinthel.github.io/rust-lang-book-ko/ch06-02-match.html
fn main() {
	let a = Coin::Penny;
    value_in_cents(a);
    
    let b = CoinV2::Quarter(UsState::Alabama);
    value_in_cents_v2(b);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?} {:?} {:?}", five, six, none);

    // _ 변경자(placeholder) (나머지)
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => println!("nothing!"),
    }

    // if let을 통한 간결한 흐름제어
    let some_u8 = Some(0u8);
    match some_u8 {
        Some(3) => println!("three"),
        _ => (),
    }
    if let Some(3) = some_u8 { // 위와 동일한 의미
        println!("three")
    }
    // if let & else 방식도 가능.
    let mut count = 0;
    let coin = CoinV2::Quarter(UsState::Alabama);
    if let CoinV2::Quarter(UsState::Alabama) = coin {
        println!("State quarter from {:?}!", UsState::Alabama);
    } else {
        count += 1;
    }
    
}


// match를 통한 enum 분기 패턴
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    // match 문을 통한 분기 가능
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// 값들을 바인딩하는 패턴들
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum CoinV2 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents_v2(coin: CoinV2) -> u32 {
    match coin {
        CoinV2::Penny => 1,
        CoinV2::Nickel => 5,
        CoinV2::Dime => 10,
        CoinV2::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

// Option<T>를 이용하는 매칭
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}




