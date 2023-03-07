// https://rinthel.github.io/rust-lang-book-ko/ch07-01-mod-and-the-filesystem.html
// ├── foo
// │   ├── bar.rs (contains the declarations in `foo::bar`)
// │   └── mod.rs (contains the declarations in `foo`, including `mod bar`)

// 퍼블릭
pub mod client;
pub mod network;

mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {}

    mod inside {
        pub fn inner_function() {}

        fn secret_function() {}
    }
}

fn try_me() {
    // 얘만 됨.
    outermost::middle_function();
}

#[cfg(test)]
mod tests {
    // 부모로 역순으로 접근 가능.
    use super::client;

    #[test]
    fn it_works() {
        client::connect();
    }
}
