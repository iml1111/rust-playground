#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }

    // 연관 함수: 일종의 클래스메소드, 스태틱 메소드?
    fn square(size: u32) -> Rectangle {
        Rectangle { length: size, width: size }
    }
}

fn main() {
    let rect1 = Rectangle { length: 50, width: 30 };

    println!("rect1 is {:?}", rect1);
    // 함수를 통한 면적 구현
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
    // 메소드를 통한 면적 반환
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
        //(&rect1).area()
    );
    let rect1 = Rectangle { length: 50, width: 30 };
    let rect2 = Rectangle { length: 40, width: 10 };
    let rect3 = Rectangle { length: 45, width: 60 };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // 연관 함수를 통한 구조체 생성
    println!("square is {:?}", Rectangle::square(3));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
}