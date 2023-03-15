// https://rinthel.github.io/rust-lang-book-ko/ch15-05-interior-mutability.html
// https://velog.io/@peeeeeter_j/Rust-Programming-25
// 내부 가변성 (interior mutability): 어떤 데이터에 대한 불변 참조자가 있을 때라도데이터를 변형할 수 있는 러스트의 디자인 패턴

// Box<T>, Rc<T>, 혹은 RefCell<T>을 선택하는 이유의 요점은 다음과 같습니다:

// Rc<T>는 동일한 데이터에 대해 복수개의 소유자를 가능하게 합니다; 
// Box<T>와 RefCell<T>은 단일 소유자만 갖습니다.

// Box<T>는 컴파일 타임에 검사된 불변 혹은 가변 빌림을 허용합니다; 
// Rc<T>는 오직 컴파일 타임에 검사된 불변 빌림만 허용합니다; RefCell<T>는 런타임에 검사된 불변 혹은 가변 빌림을 허용합니다.

// RefCell<T>이 런타임에 검사된 가변 빌림을 허용하기 때문에, RefCell<T>이 불변일 때도 RefCell<T> 내부의 값을 변경할 수 있습니다.

use std::rc::Rc;
use std::cell::RefCell;

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
    where T: Messenger {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 0.75 && percentage_of_max < 0.9 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        } else if percentage_of_max >= 0.9 && percentage_of_max < 1.0 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        }
    }
}

struct MockMessenger {
    sent_messages: RefCell<Vec<String>>,
}

impl MockMessenger {
    fn new() -> MockMessenger {
        MockMessenger { sent_messages: RefCell::new(vec![]) }
    }

    fn send_v2(&self, message: &str) {
        let mut one_borrow = self.sent_messages.borrow_mut();
        let mut two_borrow = self.sent_messages.borrow_mut();
        one_borrow.push(String::from(message));
        two_borrow.push(String::from(message));
    }
}

impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
    	// 쓰기 권한 빌리기 (.borrow_mut())
        self.sent_messages.borrow_mut().push(String::from(message));
    }
}

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let x = 5;
    // x는 쓰기 권한으로 빌릴 수 없음
    // let y = &mut x; // Error!

    let mock_messenger = MockMessenger::new();
    let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

    limit_tracker.set_value(80);
    // 읽기 권한 빌리기 (.borrow())
    assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);

    // already borrowed: BorrowMutError: Panic으로 에러 반환됨.
    // mock_messenger.send_v2("asd");

    // Rc<T>와 RefCell<T>를 조합하여 가변 데이터의 복수 소유자 만들기
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    // Refcell이기에 불변 생성도 수정이 가능함.
    // 이미 a에게 값을 빌려주었고, B, C 또한 빌려주었음에도 가변 참조가 가능함.
    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}







