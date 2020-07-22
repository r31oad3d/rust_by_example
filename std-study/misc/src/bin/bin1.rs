trait T1 {
    fn fun1(&self);
    fn fun2(&self) {
        self.fun1();
    }
    fn fun3(&self);
}

struct ST1;
struct ST2;
impl T1 for ST1 {
    fn fun1(&self) {
        println!("fun1");
    }

    fn fun3(&self) {
        self.fun2();
    }
}
impl T1 for ST2 {
    fn fun1(&self) {
        unimplemented!()
    }

    fn fun3(&self) {
        unimplemented!()
    }
}

// fn func_t1 (switch: bool) -> impl T1 {
//     if switch {
//         ST1{}
//     } else {
//         ST2{}
//     }
// }

fn func_t2(switch: bool) -> &'static dyn T1 {
    if switch {
        &ST1 {}
    } else {
        &ST2 {}
    }
}

fn func_t3(switch: bool) -> Box<dyn T1> {
    if switch {
        Box::new(ST1 {})
    } else {
        Box::new(ST2 {})
    }
}
trait SomeTrait {}
trait AnotherTrait {}
impl SomeTrait for dyn AnotherTrait {}

impl<T> SomeTrait for T where T: AnotherTrait {}

trait Foo {
    fn default_impl(&self) {
        println!("correct impl!");
    }
}
impl dyn Foo {
    fn trait_object() {
        println!("trait object impl");
    }
    fn trait_object_self(&self) {
        println!("trait object self impl");
    }
}
struct Bar {}

impl Foo for Bar {}
// fn foo_op(obj: &Foo) {
//     obj.trait_object_self();
// }

fn main() {
    let st1: ST1 = ST1 {};
    st1.fun2();
    println!("=====");
    st1.fun3();

    let b = Bar {};
    b.default_impl();
    // b.trait_object();
    Foo::trait_object();
    let o = &b as &dyn Foo;
    o.trait_object_self();

    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // }
    // println!("{:?}", r);
}
