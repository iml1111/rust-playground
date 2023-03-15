// https://rinthel.github.io/rust-lang-book-ko/ch13-02-iterators.html

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
	// into_iter: 소유권을 소비하며 iteration(원본 객체 사용 불가)
	// https://doc.rust-lang.org/std/iter/index.html#the-three-forms-of-iteration
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}

#[derive(PartialEq, Debug)]
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}


impl Iterator for Counter {
	type Item = u32;

	fn next(&mut self) -> Option<Self::Item> {
		self.count += 1;

		if self.count < 6 {
			Some(self.count)
		} else {
			None
		}
	}
}

fn main() {
	// 러스트의 이터레이터는 소비하기 전까지는 아무런 동작도 하지 않음.
	let v1 = vec![1, 2, 3];
	let v1_iter = v1.iter();
	for val in v1_iter {
	    println!("Got: {}", val);
	}

	// 모든 반복자는 표준 라이브러리에 정의된 Iterator 트레잇을 구현
	let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    // sum을 호출한 후 v1_iter은 사용불가
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);

    // 다른 반복자를 생성하기
    let v1: Vec<i32> = vec![1, 2, 3];
	v1.iter().map(|x| x + 1);
	// 이를 활용해서 벡터 변환 가능
	let v1: Vec<i32> = vec![1, 2, 3];
	let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
	assert_eq!(v2, vec![2, 3, 4]);

	// 클로저 써보기
	let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];
    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 10, style: String::from("boot") },
        ]
    );

    // Iterator 트레잇으로 커스텀 이터레이터 만들기
    let mut counter = Counter::new();
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);

    let sum: u32 = Counter::new().zip(Counter::new().skip(1))
                                 .map(|(a, b)| a * b)
                                 .filter(|x| x % 3 == 0) // [6, 12]
                                 .sum();
    assert_eq!(18, sum);
    let temp: Vec<_> = Counter::new().zip(Counter::new().skip(1))
                                 .map(|(a, b)| a * b).collect();
    println!("{:?}", temp);
}











