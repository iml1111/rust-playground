// https://rinthel.github.io/rust-lang-book-ko/ch10-03-lifetime-syntax.html
fn main() {
	{
	    let r;
	    {
	        let x = 5;
	        r = &x;
	    } // => 여기서 x가 소멸
	    // 스코프 밖에서 r에 접근시 Error   
	}

	// 정상적인 사용방법
	{
	    let x = 5;            // -----+-- 'b
	    let r = &x;           // --+--+-- 'a
	    println!("r: {}", r); //   |  |
	}

	// 라이프타임 명시 문법 예제
	let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // x,y 문자열은 같은 라이프타임(수명)
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
    // 비록 string1이 살아있지만, 가장 짧은 string2가 우선시되기에 Error.
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    } // 여기가 a'가 수명주기의 끝. Error 발생.
    //println!("The longest string is {}", result);

    // x의 라이프타임만 적용해보기
    let string1 = String::from("long string is long");
    let result;
    { // y의 라이프타임을 고려하지 않음.
        let string2 = String::from("xyz");
        result = longest_x(string1.as_str(), string2.as_str());   
    }
    println!("The longest string is {}", result);
    


}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
	// 함수 시그니처 내에 라이프타임 명시
	// 이 함수는 또한 적어도 라이프타임 'a만큼 살아있는 스트링 슬라이스를 반환.
	// 해당 라이프타임 명시가 실제로 얼마나 오랫동안 살아남는가를 바꾸는 건 아님.
	// x, y : 해당 제약 사항을 통해 x,y 문자열은 같은 라이프타임을 가져야 함.
	// 만약 다를 경우, x 와 y의 라이프타임 중 짧은 쪽으로 취급.
	// -> &'a str : 반환값으로 적어도 인풋과 같은 만큼의 라이프타임을 가지는 결과를 반환할 것임.
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_x<'a>(x: &'a str, y: &str) -> &'a str {
    x
}