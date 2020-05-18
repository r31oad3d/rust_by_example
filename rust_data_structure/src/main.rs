mod data_structure;
#[macro_use]
mod macros;

use data_structure::binary_tree;
use data_structure::queue::Queue;
use data_structure::stack::Stack;

fn test_stack() {
    #[derive(PartialEq, Eq, Debug)]
    struct TestStruct {
        a: i32,
    }
    let a = TestStruct { a: 5 };
    let b = TestStruct { a: 9 };
    let mut s = Stack::<&TestStruct>::new();
    assert_eq!(s.pop(), None);
    s.push(&a);
    s.push(&b);
    println!("{:?}", s);

    assert_eq!(s.pop(), Some(&b));
    assert_eq!(s.pop(), Some(&a));
    assert_eq!(s.pop(), None);
}

fn test_queue() {}

struct Dummy(i32);

impl Dummy {
    double_method! {
        self,
        {
            self.0 *= 2;
            self
        }
    }
}

fn main() {
    test_stack();

    //let color = vec![RED,GREEN, BLUE];
    let a = 3 * four!();
    let b = multiply_add!(1, 2, 3);
    println!("a={}", a);
    println!("b={}", b);
    let v = vec_strs![1 + 1, 2 + 2];
    assert_eq!(&*v, &["2", "4"]);
    println!("{:?}", stringify!(dummy(2 * (1 + (3)))));
    println!("{:?}", capture_expr_then_stringify!(dummy(2 * (1 + (3)))));

    println!(
        "{}\n{}\n{}\n",
        match_tokens!((caravan)),
        match_tokens!(3 + 6),
        match_tokens!(5)
    );
    println!(
        "{}\n{}\n{}",
        capture_then_match_tokens!((caravan)),
        capture_then_match_tokens!(3 + 6),
        capture_then_match_tokens!(5)
    );

    //let four = using_a1!(a1/10);

    println!("{}", what_is!(self));
    println!("{}", call_with_ident!(what_is(self)));
}
