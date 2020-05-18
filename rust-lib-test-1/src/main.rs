fn main() {
    #[link(name = "testlib2", kind = "static")]
    extern "C" {
        #[link_name = "add"]
        fn add1111(a: i32, b: i32) -> i32;
    }

    unsafe { println!("{}", add1111(1, 2)) }
}
