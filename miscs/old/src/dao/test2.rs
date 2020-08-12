use std::fmt::Debug;
pub trait Fly {
    fn fly(&self) -> bool;
}
#[derive(Debug)]
struct Duck;
#[derive(Debug)]
struct Pig;

impl Fly for Duck {
    fn fly(&self) -> bool {
        true
    }
}

impl Fly for Pig {
    fn fly(&self) -> bool {
        return false;
    }
}

fn fly_static(s: impl Fly + Debug) -> bool {
    s.fly()
}

fn can_fly(s: impl Fly + Debug) -> impl Fly + Debug {
    if s.fly() {
        println!("{:?} can fly", s);
    } else {
        println!("{:?} can not fly", s);
    }
    s
}

//pub fn func2() {
//    let pig = Pig;
//    assert_eq!(fly_static(pig), false);
//    let duck = Duck;
//    assert_eq!(fly_static(duck), true);
//
//    let pig = Pig;
//    let pig = can_fly(pig);
//    let duck = Duck;
//    let duck = can_fly(duck);
//
//    let duck2: impl Fly = can_fly(duck);
//}

pub fn func3() {
    use std::thread;
    let x = vec![1, 2, 3, 4];
    thread::spawn(|| x);
}
