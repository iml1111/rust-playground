use std::ops::Deref;

// 커스텀 스마트 포인터[튜플] (Box를 흉내냄)
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// * 연산자로 역참조를 가능케 하기 위해서, 우리는 Deref 트레잇을 구현
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);
    // 출력해보면 같지만 실제로는 다름.
    println!("{} {}", y, *y);

    // Box를 참조자처럼 사용할 수 있음.
    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    // MyBox 예제
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    // 2개는 같은 의미
    assert_eq!(5, *y);
    assert_eq!(5, *(y.deref()));

    // 역참조 강제 기능을 통해 위와 같이만 써도 됨.
    // 역참조 강제는 Deref를 구현한 어떤 타입의 참조자를 Deref가 
    // 본래의 타입으로부터 바꿀 수 있는 타입의 참조자로 바꿔줌.
    let m = MyBox::new(String::from("Rust"));
    // 자동으로 Box 내부의 객체 참조자를 빼내줌.
    hello(&m);
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
}