fn main() {
	let number = 3;

	// if에는 반드시 bool (if number => Error)
	if number < 5 {
		println!("true!");
	} else {
		println!("false!");
	}

	if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // let 구문에서 if문 사용하기
    let condition = true;
    let number = if condition {
        5
    } else {
        6 // 이곳에 if와 다른 타입을 해선 안됨. Ex) "six"
    };
    println!("The value of number is: {}", number);

    // 무한 반복
    // loop {println!("again!");}

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }

    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    //(1..4): Range 객체
    // rev: 리버스
    for number in (1..4).rev() {
        println!("{}!", number);
    }
}