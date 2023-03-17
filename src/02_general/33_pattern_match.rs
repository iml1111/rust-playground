fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    // favorite_color가 Some인 경우, color에 할당하고 진입.
    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }


    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    // stack.pop()이 Some인 경우, top에 할당하고 재진입.
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // for loop
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // 리터럴 매칭
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // 명명 변수 매칭
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);

    // 다중 패턴
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // 범위 매칭
    let x = 5;
    match x {
        1 ... 5 => println!("one through five"),
        _ => println!("something else"),
    }
    let x = 'c';
    match x {
        'a' ... 'j' => println!("early ASCII letter"),
        'k' ... 'z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // 구조체 분해하기
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    // let Point { x, y } = p; 도 가능함.
    assert_eq!(0, a);
    assert_eq!(7, b);
    match p {
        // x는 무관, y만 0인 경우
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        // y는 무관, x만 0인 경우
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    // 열거형 해체하기
    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        },
        Message::Move { x, y } => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x,
                y
            );
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!(
                "Change the color to red {}, green {}, and blue {}",
                r,
                g,
                b
            )
        }
    }

    // 참조자 해체하기
    let points = vec![
        Point { x: 0, y: 0 },
        Point { x: 1, y: 5 },
        Point { x: 10, y: -3 },
    ];

    // 135
    // 만약 &Point { x, y } 에서 & 를 뺀다면 타입 불일치(type mismatch) 오류가 발생. 
    // iter는 벡터 내 요소들의 실제 값이 아닌 참조자이기 때문
    let sum_of_squares: i32 = points
        .iter()
        .map(|&Point { x, y }| x * x + y * y)
        .sum();

    // 구조체와 튜플 해체하기
    let ((feet, inches), Point {x, y}) = ((3, 10), Point { x: 3, y: -10 });

    // _ 를 사용해 값 일부 무시하기
    // _ 는 이름만 변수가 아님. 정말 바인딩되지 않도록 특수 처리됨.
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    // 값이 유효다는 것 자체만 보겠다는 의미
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        },
    }

    // _를 붙여 미사용 변수에 대한 경고 생성을 막을 수 있음.
    let _x = 5;

    // ..을 이용해 나머지 변수 부분 무시하기
    let origin = Point { x: 0, y: 0 };
    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        },
    }

    // ref를 통해 패턴 변수가 소유권을 갖지 않고 참조자로 가져오기
    let robot_name = Some(String::from("Bors"));
    match robot_name {
        Some(ref name) => println!("Found a name: {}", name),
        None => (),
    }
    println!("robot_name is: {:?}", robot_name);
    // ref mut를 통해 가변 참조자로 가져올 수도 있음.
    let mut robot_name = Some(String::from("Bors"));
    match robot_name {
        Some(ref mut name) => *name = String::from("Another name"),
        None => (),
    }
    println!("robot_name is: {:?}", robot_name);


    // 매치 가드를 통한 추가 조건
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {:?}", n),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);
    
    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    // @ (at 연산자) 바인딩
    let msg = MessageV2::Hello { id: 5 };
    match msg {
        // 패턴과 매치되는지 확인하는 동시에 해당 값을 갖는 변수를 생성
        MessageV2::Hello { id: id_variable @ 3...7 } => {
            println!("Found an id in range: {}", id_variable)
        },
        MessageV2::Hello { id: 10...12 } => {
            println!("Found an id in another range")
        },
        MessageV2::Hello { id } => {
            println!("Found some other id: {}", id)
        },
    }

}

struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn foo(_: i32, y: i32) {
    // 인자 값 버리기 (_)
    println!("This code only uses the y parameter: {}", y);
}

enum MessageV2 {
    Hello { id: i32 },
}








