fn main() {
	let mut s = String::from("hello world");
	let word = first_word(&s);
	s.clear();
	println!("{word}");

	let s = String::from("hello world");
	let hello = &s[0..5];
	let world = &s[6..11];
	let s = String::from("hello");
	let slice = &s[0..2];
	let slice = &s[..2];
	let s = String::from("hello");
	let len = s.len();
	let slice = &s[3..len];
	let slice = &s[3..];
	let s = String::from("hello");
	let len = s.len();
	let slice = &s[0..len];
	let slice = &s[..];

	let a = [1, 2, 3, 4, 5];
	let slice = &a[1..3];

	let mut s = String::from("hello world");
	let word = first_word_v2(&s);
	// s.clear(); => &s를 빌리는 동안 s에서 mut 빌림이 일어나선 안됨.
	println!("{word}");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // String을 bytes 배열로 변환.

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_v2(s: &str) -> &str {
	let bytes = s.as_bytes();
	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			return &s[0..i];
		}
	}
	&s[..]
}