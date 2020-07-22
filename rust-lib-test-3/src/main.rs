extern crate libloading;

use libloading::{Library, Symbol};
use std::cell::RefCell;
use std::ops::Deref;

fn main() {
    let out_lib = Library::new("/Users/johnj/Development/code/rust/rust_by_example/target/release/libtestlib2.dylib").unwrap();
    unsafe {
        let func: Symbol<unsafe extern "C" fn(i32, i32) -> i32> =
            out_lib.get(b"multiply").unwrap();
        println!("{:?}", func(5, 6));

        // let V_vale : Symbol<unsafe extern i32> = out_lib.get(b"V").unwrap();
    }
    let s1 = RefCell::new(Box::new(String::from("1")));
    let s2 = s1.borrow_mut();
    let s3 = s2;
    print_type_name_of(&s3);
    //println!("{:?}", (*s2));
    let s4 = String::from("123");
    //println!("{}", calculate_length_v1(s4));
    println!("{}", calculate_length_v2(&s4));

    let s5 = String::from("abcdefg");
    let mut str1 = first_word(&s5);
    //std::mem::drop(ret1);
    //s5 = String::from("xyz");
    //s5.clear();
    //std::mem::drop(s5);
    str1 = "1";
    std::mem::drop(s5);
    println!("{}", str1);
}
fn print_type_name_of<T>(_: T) {
    println!("{}", std::any::type_name::<T>())
}
//bad
fn calculate_length_v1(ref a: String) -> usize {
    print_type_name_of(a);
    a.len()
}

fn calculate_length_v2(a: &String) -> usize {
    print_type_name_of(a);
    a.len()
}

fn first_word<'a>(s: &'a String) -> &'a str {
    &s[0..1]
}

fn longest_st<'l>(s1: &'l str, s2: &'l str) -> &'l str {
    if s1.len() > s2.len() {
        &s1
    } else {
        &s2
    }
}

// fn dangle_pointer() -> &'static String {
//     let s1 = String::from("111");
//
//     &s1
// }
