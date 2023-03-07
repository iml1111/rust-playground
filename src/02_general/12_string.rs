// https://rinthel.github.io/rust-lang-book-ko/ch08-02-strings.html
fn main() {
    let mut s = String::new();

    // data는 String이 아님(&str)
    let data = "initial contents";
    let s = data.to_string();
    // 이걸 한방에 하면
    let s = String::from("initial contents");
    // 스트링 추가하기
    let mut s = String::from("foo");
    s.push_str("bar");

    // string 더하기
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1은 이동되어 더이상 쓸 수 없음

    // format! 을 통해 여러 문자열 합치기
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);

    let s1 = String::from("hello");
    // let h = &s1[0]; // ERROR 발생: String은 인덱싱을 지원안함.
    // utf-8 인코딩을 취급할때 첫번째 바이트로 첫글자가 성립이 안될 수 있기 때문임.
    // 이건 너무 한거 아니냐
    let char_array: Vec<char> = s1.chars().collect();
    println!("{}", char_array[0]);
}