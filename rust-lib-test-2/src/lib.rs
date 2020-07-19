pub mod compute {
    #[no_mangle]
    pub extern "Rust" fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    #[no_mangle]
    pub extern "Rust" fn minus(a: i32, b: i32) -> i32 { a - b }
}

pub mod things {
    #[derive(Default)]
    pub struct A {
        a:i32,
    }

    impl A {
        #[no_mangle]
        pub extern "Rust" fn new() -> Self {
            A::default()
        }
    }
}

pub mod methods {


    #[no_mangle]
    pub fn do_callback(f: Box<dyn Fn(i32) -> i32>) -> i32 {
        let a = 5;
        f(a)
    }
}
