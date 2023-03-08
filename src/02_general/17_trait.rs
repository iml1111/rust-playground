use std::cmp::PartialOrd;

pub trait Summarizable {
    // Trait, 일종의 인터페이스 객체
    // 제약사항 / 특성 상속 등에 사용됨.
    fn summary(&self) -> String {
        String::from("(Read more...)")
    }

    fn author_summary(&self) -> String;
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summarizable for Tweet {
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn author_summary(&self) -> String {
        format!("@{}", self.username)
    }
}

struct WeatherForecast {
    high_temp: f64,
    low_temp: f64,
    chance_of_precipitation: f64,
}

impl Summarizable for WeatherForecast {
    fn summary(&self) -> String {
        format!("The high will be {}, and the low will be {}. The chance of
        precipitation is {}%.", self.high_temp, self.low_temp,
        self.chance_of_precipitation)
    }

    fn author_summary(&self) -> String {
        format!("#Author")
    }
}

pub fn notify<T: Summarizable>(item: T) {
    // Summarizable Trait 구현된 객체만 들어올 수 있음.
    println!("Breaking news! {}", item.summary());
}

// 기존에 있는 Trait 적용하기
fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_generic<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summary());
    notify(tweet);

    let numbers = vec![34, 50, 25, 100, 65];
    let result = largest(&numbers);
    println!("The largest number is {}", result);

    let numbers = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&numbers);
    println!("The largest number is {}", result);

    // 제네릭을 통해 다양한 자료형을 입력받을 수 있도록 함.
    let numbers = vec![34, 50, 25, 100, 65];
    let result = largest_generic(&numbers);
    println!("The largest number is {}", result);

    let chars = vec!['y', 'm', 'a', 'q'];
    let result = largest_generic(&chars);
    println!("The largest char is {}", result);
}