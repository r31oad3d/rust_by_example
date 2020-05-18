use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;
use std::any::{Any, TypeId};
#[derive(HelloMacro)]
struct Pancake;

fn main() {
    let v1 = "hello";
    let mut a: &Any;

    a = &v1;

    println!("{:?}", a.type_id());
    assert!(a.is::<&str>());

    print_any(&v1);

    let v2: u32 = 1;
    print_any(&v2);

    Pancake::hello_macro();
}

fn print_any(any: &Any) {
    if let Some(v) = any.downcast_ref::<u32>() {
        println!("u32 {:x}", v);
    } else if let Some(v) = any.downcast_ref::<&str>() {
        println!("str {:?}", v);
    } else {
        println!("else");
    }
}
