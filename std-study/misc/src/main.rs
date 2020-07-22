// #![feature(core_intrinsics)]

use std::ops::{Sub, SubAssign};
use std::process::Output;

struct Foo<T>(T);

struct Bar<T: ?Sized>(T);

//struct FooUse(Foo<[i32]>);

struct BarUse(Bar<[i32]>);

struct B<'l> {
    pub a: &'l i32,
}
enum Favor<'a> {
    Nor(u32),
    NorRef(u32),
    Ref(&'a u32),
    RefRef(&'a u32),
}

fn config(data: &u32) {
    println!("log data: {}", data);
}

fn log(fav: Favor) {
    match fav {
        Favor::Nor(data) => {
            config(&data);
            print_type_name_of(data);
        }
        Favor::NorRef(ref data) => {
            config(data);
            print_type_name_of(data);
        }
        Favor::Ref(data) => {
            config(data);
            print_type_name_of(data);
        }
        Favor::RefRef(ref data) => {
            config(data);
            print_type_name_of(data);
        }
    }
}
#[derive(Debug)]
struct S11<T: Sub<Output = T>> {
    value: T,
}

impl<T: Sub<Output = T>> Sub for S11<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        S11 {
            value: self.value - rhs.value,
        }
    }
}

fn main() {
    let ref a = 1;
    let b = B { a };
    let r = &1;
    let &a = r;
    let a = *r;
    let x = &false;
    print_type_name_of(x);
    let &x = &false;
    print_type_name_of(x);
    let ref x = &false;
    print_type_name_of(x);

    log(Favor::Nor(1));
    log(Favor::Ref(&2));
    log(Favor::NorRef(3));
    log(Favor::RefRef(&4));

    let s1 = String::from("hello");
    print_type_name_of(s1);

    let a = 7;
    let b = &a;
    println!("{:?}", b as *const i32);
    println!("{:p}", b);

    let a: u32 = 0x10111213;
    let b: [u8; 4];
    b = unsafe { std::mem::transmute(a) };

    println!("{:x}-{:x}-{:x}-{:x}", b[0], b[1], b[2], b[3]);

    println!("minus returns : {:?}", minus(2, 1));
    println!(
        "minus returns : {:?}",
        minus(S11 { value: 2 }, S11 { value: 1 })
    );
    println!(
        "minus returns : {:?}",
        minus(S11 { value: 4.8 }, S11 { value: 1.9 })
    );
    println!(
        "minus returns : {:?}",
        minus(S11 { value: 1000_usize }, S11 { value: 9_usize })
    );
    let s111 = S11 { value: 123 };
    let s112 = S11 { value: 321 };
    println!(
        "minus returns : {:?}",
        minus(S11 { value: s111 }, S11 { value: s112 })
    );
    let s111 = S11 { value: 123 };
    let s112 = S11 { value: 321 };
    println!(
        "minus returns : {:?}",
        S11 { value: s111 } - S11 { value: s112 }
    );
}
fn print_type_name_of<T>(_: T) {
    // println!("{}", unsafe { std::intrinsics::type_name::<T>() });
    println!("{}", std::any::type_name::<T>())
}

fn minus<T>(v1: T, v2: T) -> T
where
    T: Sub<Output = T>,
{
    v1 - v2
}
