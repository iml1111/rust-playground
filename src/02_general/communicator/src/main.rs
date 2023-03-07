extern crate communicator;

pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

use a::series::of;
use a::series::of::nested_modules;

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::{Red, Yellow};
use TrafficLight::*;

fn main() {
    communicator::client::connect();

    // 아래와 같이 줄여서 사용 가능.
    a::series::of::nested_modules();
    of::nested_modules();
    nested_modules();

    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green;
    let green = Green;
}