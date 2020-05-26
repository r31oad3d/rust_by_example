use std::ops::Deref;

fn main() {
    let s1 = &5;
    let s2 = Box::new(5);

    assert_eq!(5, *s1);
    assert_eq!(5, *s2);

    fn demo(i: &str) {
        println!("{:?}", i);
    }

    fn demo2(i: &A) {
        println!("demo2");
    }

    struct A;

    impl Deref for A {
        type Target = A;

        fn deref(&self) -> &Self::Target {
            println!("deref");
            self
        }
    }

    impl Drop for A {
        fn drop(&mut self) {
            println!("drop");
        }
    }

    let s = Box::new(String::from("123"));
    demo(&s);
    let a = Box::new(A);
    demo2(&a);

    let mut v: Vec<i32> = vec![];
    v.reserve(50);
    v.push(1);
    println!("{:?}", v.capacity());
    println!("{:?}", v.len());

    let x = Box::new(5);
    println!("x={}", x);
}
