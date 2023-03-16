// https://rinthel.github.io/rust-lang-book-ko/ch16-01-threads.html
// https://rinthel.github.io/rust-lang-book-ko/ch16-02-message-passing.html
use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    // 클로저를 통해 실행하는 spawn 함수 (새로운 스레드 생성 및 실행)
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // 해당 스레드가 끝날떄까지 기다림. 이게 없으면 메인스레드와 함께 모두 중지됨!
    handle.join().unwrap();

    // move로 소유권 이전하기
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();

    // 메세지 패싱 (message passing)
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        // 다른 스레드에서 메시지 전달하기
        let val = String::from("hi");
        tx.send(val).unwrap();
        // val은 전달과 동시에 소유권이 떠나게 됨.
        // println!("val is {}", val); // Error!
    });
    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    // 복수의 값들을 보내고 수신자에게 기다리게 하기
    // 송신자를 복제하여 여러 생성자 만들기
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

}