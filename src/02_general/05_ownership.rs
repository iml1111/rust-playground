fn main() {
	// 불변인 문자열을 String 타입으로 변경하며 불러옴.
	let mut s = String::from("hello");
	s.push_str(", world!");
	println!("{}", s); 

	// 이동하기 (스택에 있는 경우, 복사)
	let x = 5;
	let y = x;

	// 이동하기 (소유권 이전)
	let s1 = String::from("asd");
	let s2 = s1;
	// s1를 사용하려고 하면 Error 뜸.
	println!("{x} {y} {s2}");

	// 복사하기
	let s1 = String::from("hello");
	let s2 = s1.clone();
	println!("s1 = {}, s2 = {}", s1, s2);


	// 함수를 통한 이동(위와 똑같이 적용)
	let x = 5;
	let s = String::from("hello");
	takes_ownership(s);
	makes_copy(x);

	let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}

fn takes_ownership(some_string: String) {
	// 소유권이 아예 이전됨. 종료와 함께 메모리 해제.
	println!("{some_string}");
}

fn makes_copy(some_integer: i32) {
	// stack이므로 값이 복사됨.
	println!("{some_integer}");
}

fn gives_ownership() -> String {
	// 변수를 생성해서 소유권 채로 넘겨줌.
	let some_string = String::from("hello");
	some_string
}

fn takes_and_gives_back(a_string: String) -> String {
	// 소유권을 받았다가 다시 돌려줌.
	a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}