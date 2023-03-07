enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddrStr {
    V4(String),
    V6(String),
}

enum IpAddrV2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self) {
        // 메소드 내용은 여기 정의할 수 있습니다.
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // enum 안에서 변수를 선언하여 데이터를 넣어서 간결하게 관리할 수 있음.
    let home = IpAddrStr::V4(String::from("127.0.0.1"));
    let loopback = IpAddrStr::V6(String::from("::1"));
    // 단일 데이터 뿐 아니라 커스텀 구조체도 넣을 수 있을 듯.
    let home = IpAddrV2::V4(127, 0, 0, 1);
    let loopback = IpAddrV2::V6(String::from("::1"));

    // Message enum에 대한 그룹처리 가능.
    let m = Message::Write(String::from("hello"));
    m.call();

    // Option<T> => Some<T> | None
    // https://doc.rust-lang.org/std/option/enum.Option.html
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // let sum = x + y; 에러발생!!! 주의!!
}

fn route(ip_type: IpAddrKind) { }