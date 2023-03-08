fn main() {
    // 강제로 에러 발생시키기
    panic!("crash and burn");
    
    // 자연스럽게 터지는 코드
    let v = vec![1, 2, 3];
    v[99];
}