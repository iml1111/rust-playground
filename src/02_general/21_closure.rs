// https://rinthel.github.io/rust-lang-book-ko/ch13-01-closures.html
use std::thread;
use std::time::Duration;

// 클로저와 클로저를 호출한 결과값을 갖고 있는 구조체
struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}
impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );

    // 두개의 다른 타입으로 클로저 호출시 에러 (한번의 추론으로 확정)
    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));
    //let n = example_closure(5);

    // Caching Logic
    generate_workout_v2(
        simulated_user_specified_value,
        simulated_random_number
    );

    // 클로저의 특성: 환경캡쳐 (동일한 스코프 내의 변수 사용 허용)
    // 단 이 경우, x를 자동으로 (read)로 빌려오게 됨.
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));


    // move를 써서 소유 권한을 통째로 클로저로 전송.
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;
    // println!("can't use x here: {:?}", x);
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
}

fn generate_workout(intensity: u32, random_number: u32) {
    // 클로저 익명함수
    let expensive_result = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(1));
        intensity
    };

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result(intensity)
            );
        }
    }
}

fn generate_workout_v2(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}