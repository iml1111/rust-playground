// 테스트 코드 조직화시키기
// https://rinthel.github.io/rust-lang-book-ko/ch11-03-test-organization.html
extern crate test20;

mod common;

#[test]
fn intergration_test_add() {
    common::setup();
    let result = test20::add(2, 2);
    assert_eq!(result, 4);
}