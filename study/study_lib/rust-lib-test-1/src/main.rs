fn main() {
    // #[link(name = "testlib2", kind = "static")]
    // extern "C" {
    //     #[link_name = "add"]
    //     fn add1111(a: i32, b: i32) -> i32;
    // }

    #[link(name = "testlib2", kind = "dylib")]
    extern "Rust" {
        #[link_name = "add"]
        fn add1111(a: i32, b: i32) -> i32;
        #[link_name = "minus"]
        fn minus1111(a: i32, b: i32) -> i32;
        #[link_name = "do_callback"]
        fn do_callback(f: Box<dyn Fn(i32) -> i32>) -> i32;

    }

    unsafe { println!("{}", add1111(1, 2)) }
    unsafe { println!("{}", minus1111(3, 4)) }

    unsafe {
        println!("{:?}", do_callback(Box::new(|a| { a * a })));
    }
}
