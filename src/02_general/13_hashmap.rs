// https://rinthel.github.io/rust-lang-book-ko/ch08-03-hash-maps.html 
use std::collections::HashMap;

fn main() {
	let mut scores = HashMap::new();
	// 기본적으로는 Key에는 String만 넣어야 함.
	scores.insert(String::from("Blue"), 10);
	scores.insert(String::from("Yellow"), 50);
	println!("{scores:?}");

	// 제네릭 설정을 통해 다양한 타입을 설정 가능.
	let teams  = vec![1, 2];
	let initial_scores = vec![10, 50];
	let scores: HashMap<_, _> = teams.iter()
		.zip(initial_scores.iter()).collect();
	// https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect
	println!("{scores:?}");

	// 갑 접근하기
	let mut scores = HashMap::new();
	scores.insert(String::from("Blue"), 10);
	scores.insert(String::from("Yellow"), 50);

	let team_name = String::from("Blue");
	let score = scores.get(&team_name);
	println!("{:?}", score);
	for (key, value) in &scores {
	    println!("{}: {}", key, value);
	}

	// insert는 덮어쓰기
	let mut scores = HashMap::new();
	scores.insert(String::from("Blue"), 10);
	scores.insert(String::from("Blue"), 25);
	// 할당된 값이 없는 경우에만 추가하기
	// entry method를 통해 유무 판별 -> or_insert 메소드 사용.
	// 값이 있는 경우: Entry(OccupiedEntry { key: "Blue", value: 25, .. })
	// 값이 없는 경우: Entry(VacantEntry("Red"))
	scores.entry(String::from("Yellow")).or_insert(50);
	scores.entry(String::from("Blue")).or_insert(50);
	println!("{:?}", scores);

	// default-dict 패턴을 흉내내보기
	let text = "hello world wonderful world";
	let mut map = HashMap::new();

	// splite_whitespace: SplitWhitespace라는 이터레이터 반환.
	// https://doc.rust-lang.org/src/core/str/iter.rs.html#1206
	for word in text.split_whitespace() {
	    let count = map.entry(word).or_insert(0);
	    *count += 1;
	}
	println!("{:?}", map);

}