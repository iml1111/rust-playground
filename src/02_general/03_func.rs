fn main() {
    another_function(5);
    another_function2(1, 2);

    
    let x = 5;
    // 표현식 (일종의 익명 함수)
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);

    let x = five();
    println!("The value of x is: {}", x);
    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

fn another_function(x: i32) {
    println!("value x is: {}", x);
}

fn another_function2(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

// 반환 값을 갖는 함수
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}