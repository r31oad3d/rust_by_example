#![allow(unused_imports)]
use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use std::sync::Mutex;

fn main() {
    for i in (0..21).filter(|x| (x % 2 == 0) && (x % 3 == 0)) {
        print!("{:?}\t", i);
    }
    println!();

    for i in (0..10).map(|x| x * x) {
        print!("{:?}\t", i);
    }
    println!();
    let res1 = (1..=5).fold(0, |acc, x| acc + x * x);
    println!("res1 = {:?}\t", res1);

    let cities = ["Toronto", "New York", "Melbourne"];
    let populations = [2_615_060, 8_550_405, 4_529_500];

    let matrix = cities.iter().zip(populations.iter());

    for (c, p) in matrix {
        println!("{:10}: population = {}", c, p);
    }

    for c in char_iter::new('A', 'Z') {
        print!("{} ", c);
    }
    println!();

    let v1 = (1..=10).any(|x| x % 7 == 0);
    println!("{:?}", v1);

    let v2 = (1..=10).collect::<Vec<i32>>();
    println!("{:?}", v2);

    let v3 = (1..)
        .map(|x| x * x)
        .filter(|x| x % 5 == 0)
        .take(10)
        .collect::<Vec<i32>>();

    println!("{:?}", v3);

    for i in (1..=10).step_by(4) {
        print!("{:?}\t", i);
    }

    println!();
    let mut vv1 = Box::new(vec![1, 2, 3, 4, 5]);
    let mut vv2 = Box::new(vec![6, 7, 8, 9, 0]);
    let mut vv3 = Box::new(vec![11, 12, 13, 14, 15, 16, 17]);

    let mut vvv1 = vec![vv1, vv2, vv3];
    let vvvv = vvv1
        .into_iter()
        .map(move |mut v| {
            v.push(100);
            v
        })
        .collect::<Vec<Box<Vec<i32>>>>();
    println!("vvvv={:?}", vvvv);
    std::mem::drop(vvvv);
    // let mut vvv2 = vec![vv1, vv2, vv3];
    // vvv2.iter().for_each(|mut v| {
    //     v.push(200);
    // });
    // println!("vvv2={:?}", vvv2);
    // let mut vv1 = Rc::new(vec![1, 2, 3, 4, 5]);
    // let mut vv2 = Rc::new(vec![6, 7, 8, 9, 0]);
    // let mut vv3 = Rc::new(vec![11, 12, 13, 14, 15, 16, 17]);
    // let mut vvv1 = vec![vv1, vv2, vv3];
    //
    // vvv1.iter().for_each(|v| {
    //
    //     v.borrow_mut().push(100);
    // });
    // println!("!!vvv1={:?}", vvv1);
    // std::mem::drop(vvv1);

    let mut vv1 = RefCell::new(vec![1, 2, 3, 4, 5]);
    let mut vv2 = RefCell::new(vec![6, 7, 8, 9, 0]);
    let mut vv3 = RefCell::new(vec![11, 12, 13, 14, 15, 16, 17]);
    let mut vvv1 = vec![vv1, vv2, vv3];

    vvv1.iter().for_each(|v| (*v.borrow_mut()).push(100));
    println!("vvv1={:?}", &vvv1);
    let vvv2 = vvv1
        .into_iter()
        .map(|v| {
            (*v.borrow_mut()).push(200);
            v
        })
        .collect::<Vec<RefCell<Vec<i32>>>>();
    println!("vvv2={:?}", vvv2);
    std::mem::drop(vvv2);

    let mut vv1 = RefCell::new(vec![1, 2, 3, 4, 5]);
    let mut vv2 = RefCell::new(vec![6, 7, 8, 9, 0]);
    let mut vv3 = RefCell::new(vec![11, 12, 13, 14, 15, 16, 17]);
    let mut vvv1 = RefCell::new(vec![vv1, vv2, vv3]);


    fn push_value(v: &RefCell<Vec<RefCell<Vec<i32>>>>, value: i32, postion: usize) -> () {
        let b1 = v.borrow_mut();
        let mut postion_v =
            RefMut::map(b1, |t| t.get_mut(postion).unwrap());
        (*postion_v).borrow_mut().push(value);
    }
    push_value(&vvv1, 1000, 2);
    // (*postion_0).borrow_mut().push(400);
    // std::mem::drop(postion_0);

    println!("vvv3={:?}", &vvv1);
}
