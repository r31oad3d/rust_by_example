use std::cell::Cell;
use std::rc::Rc;
trait Foo {
    // fn func1() {
    //     println!("func1");
    // }
    // fn func2();

    fn func3(&self) {
        println!("func3 \t");
        // Self::func1();
    }

    fn func4(&self);
}

trait Bar: Foo {
    fn func4(&self);
}

trait TR1 {
    fn func1_tr1(&self);
}

struct ST1(i32);
struct ST2(f32);
impl Foo for ST1 {
    // fn func2() {
    //     println!("func2 from struct");
    // }

    fn func4(&self) {
        print!("func4 from struct1\t");
        // Self::func1();
        // Self::func2();
        self.func3();
    }
}
impl Foo for ST2 {
    fn func4(&self) {
        print!("func4 from struct2 foo\t");
        // Self::func1();
        // Self::func2();
        self.func3();
    }
}
impl Bar for ST2 {
    fn func4(&self) {
        print!("func4 from struct2 bar\t");
        // Self::func1();
        // Self::func2();
    }
}

// impl<T> Foo for T{
//     fn func4(&self) {
//         print!("func4 from struct\t");
//         // Self::func1();
//         // Self::func2();
//         self.func3();
//     }
// }

impl dyn Foo {
    //const I:i32 = 100;
    fn func5(&self) {
        print!("func5 from dyn trait\t");
    }
}
impl TR1 for dyn Foo {
    fn func1_tr1(&self) {
        println!("func1_tr1 of dyn Foo");
    }
}

fn main() {
    let st1 = ST1(1);
    // ST1::func1();
    // ST1::func2();
    st1.func4();
    // st1.func5();
    println!("{:?}", st1.0);
    // println!("{:?}", st1.I);
    let o1 = &st1 as &dyn Foo;
    o1.func4();
    o1.func5();
    o1.func1_tr1();
    // println!("{:?}", o1.I);
    // println!("{:?}", o1.0);
    // let st2 = 3;
    // let o2 = &st2 as &dyn Foo;
    // o2.func4();
    let st2 = ST2(2.0);
    //let o2 = &st2 as &dyn Foo;
    <ST2 as Foo>::func4(&st2);
    Foo::func4(&st2);
    Bar::func4(&st2);
    //st2.func4();
    // let v = vec![1, 2, 3];
    // let o = &v as &Clone;
    //let b1 = box 1;
    println!();

    let mut rc_pointer1 = Rc::new(1);
    println!(
        "strong:{}, weak:{}, value:{}",
        Rc::strong_count(&rc_pointer1),
        Rc::weak_count(&rc_pointer1),
        rc_pointer1
    );
    *Rc::get_mut(&mut rc_pointer1).unwrap() = 4;
    let mut rc_pointer2 = Rc::clone(&mut rc_pointer1);
    println!(
        "strong:{}, weak:{}, value:{}",
        Rc::strong_count(&rc_pointer1),
        Rc::weak_count(&rc_pointer1),
        rc_pointer1
    );
    println!(
        "strong:{}, weak:{}, value:{}",
        Rc::strong_count(&rc_pointer1),
        Rc::weak_count(&rc_pointer1),
        rc_pointer1
    );
    let rc_weak1 = Rc::downgrade(&rc_pointer1);
    println!(
        "strong:{}, weak:{}, value:{}",
        Rc::strong_count(&rc_pointer1),
        Rc::weak_count(&rc_pointer1),
        rc_pointer1
    );

    #[derive(Debug)]
    struct RcTest1 {
        a: i32,
        b: i32,
    };

    let rctest1 = RcTest1 { a: 1, b: 2 };
    #[derive(Debug, Copy, Clone)]
    struct TT(i32);
    #[derive(Debug)]
    struct RcTest2 {
        a: i32,
        b: Cell<i32>,
        c: Cell<TT>,
    };

    let rctest2 = RcTest2 {
        a: 1,
        b: Cell::new(2),
        c: Cell::new(TT(4)),
    };

    //rctest1.b = 8;
    rctest2.b.set(8);

    println!("{:?}", rctest1);
    println!("{:?}", rctest2);
}
