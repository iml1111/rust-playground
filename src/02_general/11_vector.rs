enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let mut v: Vec<i32> = Vec::new();
    // 이렇게도 가능함 (매크로)
    let mut v = vec![1, 2, 3];
    v.push(5);
    println!("{:?}", v);

    // 요소 읽기
    let a: &i32 = &v[2];
    let b: Option<&i32> = v.get(2);
    let c: Option<&i32> = v.get(999);
    println!("{a:?} {b:?} {c:?}");

    // Error 예제
    let mut v = vec![1, 2, 3, 4, 5]; 
    let first = &v[0]; // 여기서 빌린 후, 끝날때까지 지속됨.
    v.push(6); // 쓰기 권한을 빌리기 때문에 first 호출시 터짐.

    // 루프 처리하기
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{:?}", i);
    }
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        // i는 [&mut v] 이기 때문에 여기에 값을 넣으려면 *역참조를 해야 함.
        // 그냥 i는 읽을 때 활용.
        *i += 50; 
    }
    println!("{v:?}");

    // 열거형을 통해 여러 타입 드리븐하기
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}