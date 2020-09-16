use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;
use std::any::{Any, TypeId};
#[derive(HelloMacro)]
struct Pancake;

#[derive(Default, Debug)]
struct A<'a> {
    a: i32,
    b: &'a str,
}

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

    let mut a = A::default();
    a.a = 16;
    a.b = "aaaaa";
    let a_a1 = &(a.a);
    let a_b1 = &(a.b);
    let a_a2 = &a.a;
    let a_b2 = &a.b;
    let a_a3 = &(a).a;
    let a_b3 = &(a).b;
    println!("{:?}", a_a1);
    println!("{:?}", a_a2);
    println!("{:?}", a_a3);
    println!("{:?}", a_b1);
    println!("{:?}", a_b2);
    println!("{:?}", a_b3);
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
