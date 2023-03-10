use std::fmt;
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}
// 해당 구조체가 라이프타임을 가지고 있다면 구현 메소드도 아래와 같이 구현해야 함.
impl<'a> Display for ImportantExcerpt<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ImportantExcerpt({})", self.part)
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
    println!("{i:?}");

    let long = "very long";
    let short = "short";
    longest_with_an_announcement(long, short, i);
}