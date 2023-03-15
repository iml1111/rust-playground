// https://rinthel.github.io/rust-lang-book-ko/ch15-01-box.html

// 재귀적 타입을 다루기 위해 Box가 사용됨(크기 추산이 불가능한 경우)
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
	// 해당 값을 힙에 집어넣어 할당함
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(
    	1, 
    	Box::new(
    		Cons(
    			2,
    			Box::new(
    				Cons(3, Box::new(Nil))
    			)
    		)
    	)
    );
}