// https://rinthel.github.io/rust-lang-book-ko/ch16-03-shared-state.html
// 스레드간 메모리를 공유하는 통신방법
use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
	// 뮤텍스를 사용하여 한번에 한 스레드에서의 데이터 접근을 허용
    // - 데이터를 사용하기 전에 반드시 락을 얻는 시도 필요.
    // - 만일 뮤텍스 데이터 사용이 끝났다면, 다른 스레드들이 락을 얻을 수 있도록 반드시 언락.
     let m = Mutex::new(5);
    {
    	// lock(): 락 획득하기
        let mut num = m.lock().unwrap();
        *num = 6;
    } // 직접 unlock을 하지 않아도 스코프를 벗어날때 해제되긴 함.(MutexGuard의 Drop trait 기능)
    println!("m = {:?}", m);

    // 여러 스레드들 사이에서 Mutex<T> 공유하기
    // % Mutex의 소유권을 여러 스레드로 동시에 이동시킬 수 없음 (move)
    // 때문 Arc(Rc의 멀티스레딩 버전)
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
    	let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}