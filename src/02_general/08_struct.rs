struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
// 튜플 구조체
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    // 구조체 선언
    let mut user1 = User {
        email: String::from("asd@asd.com"),
        username: String::from("asd"),
        active: true,
        sign_in_count: 1,
    }
    user1.email = String::from("anotheremail@example.com");

    // user1 재사용
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    // 튜플 구조체 생성
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    //kawargs가 같으면 생략 가능.
    User {
        email, 
        username,
        active: true,
        sign_in_count: 1,
    }
}
