// https://rinthel.github.io/rust-lang-book-ko/ch15-04-rc.html
// 레퍼런스 카운팅 스마트 포인터
// Rc<T>가 오직 단일 스레드 시나리오 상에서만 사용 가능

enum List {
    Cons(i32, Box<List>),
    Nil,
}

enum ListV2 {
    ConsV2(i32, Rc<ListV2>),
    NilV2,
}

use List::{Cons, Nil};
use ListV2::{ConsV2, NilV2};
use std::rc::Rc;

fn main() {

	// 이렇게 같은 a 변수를 소유하려고 하면 box라도 에러가 뜸.
    let a = Cons(
    	5,
        Box::new(
        	Cons(10, Box::new(Nil))
        )
    );
    let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a)); // Error !!

    // b를 만들때는 a의 소유권을 얻는 대신, a를 가지고 있는 Rc<ListV2>를 클론
    // 데이터에 대한 참조 카운트는 증가
    let a = Rc::new(
    	ConsV2(
    		5, Rc::new(
    			ConsV2(10, Rc::new(NilV2))
    		)
    	)
    );
    // Rc::clone(& << 로 빌려줌.
    let b = ConsV2(3, Rc::clone(&a));
    let c = ConsV2(4, Rc::clone(&a));

    // Rc<T>의 클론 생성은 참조 카운트를 증가
    let a = Rc::new(ConsV2(5, Rc::new(ConsV2(10, Rc::new(NilV2)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = ConsV2(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = ConsV2(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

}
