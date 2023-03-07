fn main() {
    // s1의 읽기 권한을 빌려줌. (&=참조자(레퍼런스))
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // s1의 쓰기 권한(가변 참조자)을 빌려줌(&s일 경우 에러 발생)
    let mut s = String::from("hello");
    change(&mut s);

    // 가변 참조자의 경우, 하나를 더 만들려고 하면 에러.
    let mut s = String::from("hello");
    let r1 = &mut s;
    println!("{r1}");
    // let r2 = &mut s; 특정 스코프 내에서 가변 참조는 한번만 생성 가능.
    {   //별도의 스코프이므로 이건 문제 없음.
        let r2 = &mut s; 
        println!("{r2}");
    }

    // 가변 참조자의 또 한가지 조건
    let mut s = String::from("hello");
    let r1 = &s; // 문제 없음
    let r2 = &s; // 문제 없음
    let r3 = &mut s; 
    // 에러 발생(불변 참조가 있을때 가변 참조가 일어날 경우)
   // println!("{r2}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String {
//     // Dangling: 포인터를 보존하는 동안, 그 메모리를 해제함으로써 다른 개체에게 
//     // 사용하도록 줘버렸을 지도 모를 메모리를 참조하고 있는 포인터
//     let s = String::from("hello");
//     &s
// }

fn no_dangle() -> String {
    // Dangling을 피하기 위해선 소유권 자체를 반환.
    let s = String::from("hello");
    s
}