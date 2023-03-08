use std::io;
use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    let f = File::open("hello.txt");

    // Result 타입 match를 통해 잠재된 실패를 분기.
    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        },
    };

    let f = File::open("hello.txt");
    // 서로 다른 에러에 대한 매칭도 가능.
    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!(
                        "Tried to create file but there was a problem: {:?}",
                        e
                    )
                },
            }
        },
        Err(error) => {
            panic!(
                "There was a problem opening the file: {:?}",
                error
            )
        },
    };

    // 에러 분기 처리를 위한 숏컷
    // except는 패닉 메시지를 고를 수 있음.
    let f = File::open("hello.txt").unwrap();
    let f = File::open("hello.txt").expect("Failed to open hello.txt");


    let s = read_username_from_file_v3();
    println!("{s:?}");

    // 오직 Result를 반환하는 함수 내에서만 물음표 연산자를 사용할 수 있음
    // main() 함수에서는 에러 발생!!
    // let f = File::open("hello.txt")?;
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    // s로 읽어오는데 성공하면 s 리턴
    // 실패하면 error 리턴
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
    
fn read_username_from_file_v2() -> Result<String, io::Error> {
    // 위 함수의 쇼트컷 버전
    // ?를 만났을때 내부가 Erro면 바로 반환.(Result에만 사용 가능)
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_v3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}