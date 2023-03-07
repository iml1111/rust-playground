// https://rinthel.github.io/rust-lang-book-ko/ch03-02-data-types.html


fn main() {
	// :u32 -> 타입 명시
	let guess: u32 = "42".parse().expect("Not a number!");

	// 부동 소수점 타입
	let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // bool 타입
    let t = true;
    let f: bool = false; // with explicit type annotation

    // 수학 연산들
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;

    // 문자타입
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    // 튜플
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
    
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    // 배열 (heap이 아닌 stack에 할당됨, 고정된 숫자)
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    // let index = 10;
    // let element = a[index]; (길이 초과로 인한 에러 발생)
}