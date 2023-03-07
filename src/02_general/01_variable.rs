// https://rinthel.github.io/rust-lang-book-ko/ch03-01-variables-and-mutability.html
// 상수 또한 불변함.
// 상수는 자신이 선언되어 있는 영역 내에서 프로그램이 실행되는 시간 동안 항상 유효하기에, 당신의 어플리케이션 도메인 전체에 걸쳐 프로그램의 다양한 곳에서 사용되는 값을 상수로 하면 유용
const MAX_POINTS: u32 = 100_000;

fn main() {
    // let x = 5; => mut가 아니므로 수정 불가능.
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6; 
    println!("The value of x is: {}", x);

    // mut가 아닐지라도 let을 반복적으로 할 수는 있음.
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    // 재선언되며, 타입 또한 다시 추론됨.
    let spaces = "   ";
    let spaces = spaces.len();

    // 하지만 이 경우, 타입이 다시 추론되지 않기에 에러 반환.
    // let mut spaces = "   ";
    // spaces = spaces.len();
}