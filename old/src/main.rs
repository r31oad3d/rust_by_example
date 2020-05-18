//#![feature(generators, generator_trait)]

mod async_test;
mod dao;
mod mods;
mod pow;
use async_test::async1;
use core::cmp;
use dao::test1::*;
use dao::test2::*;
use mods::mod1::*;
use mods::mod2::*;
use mods::mod3::*;
use mods::mod4::*;
use mods::mod5::*;
use mods::mod6::*;
use mods::mod7::*;
use pow::pow::do_pow;

fn main() {
    println!("Hello, world!");
    println!("{1}, {0}", "Alice", "Bob");
    println!("{a},{b}", a = "aa", b = "cc");
    println!("{:b}", 2);
    println!("{number:>width$}", number = 1, width = 6);
    println!("{number:0>width$}", number = 1, width = 6);

    println!("{number:<width$}", number = 1, width = 6);
    println!("{number:0<width$}", number = 1, width = 6);

    println!("Pi is roughly {:.*}", 3, 3.1415926);

    println!("{:?}", DebugPrintable::new(1));

    println!("{:?} months in a year", 12);
    println!(
        "{1:?} {0:?} is the {actor:?} name",
        "John",
        "Doe",
        actor = "actor's"
    );

    //println!("Now {:?} will print", Structure(1));

    println!("{}", Point2D::new(1.1, 2.2));

    test2();
    test3();
    #[allow(unused_assignments, unused_variables)]
    {
        let ref a1: i32;
        let ref a2;
        a1 = &1;
        a2 = &1;

        let ref a = 2;
        let a = &2;

        let a3 = &1;
        let &a4 = a3;
        let a5 = *a3;

        let ref a6 = &1;
        let a7 = *a6;
        let a8 = *a7;
    }

    let _vv = ["🐮", "🥔", "🐔", "🌽"].iter();

    test4_1();

    test5();
    test5_1();
    test5_2();
    test6();
    test6_1();

    //    println!("1000 as a u16 is: {}", 1000 as u16);
    //    #[allow(overflowing_literals)]
    //    println!("1000 as a u8 is : {}", 1000 as u8);
    //    println!("  -1 as a u8 is : {}", (-1i8) as u8);
    type NanoSecond = u64;
    type Inch = u64;
    #[allow(non_camel_case_types)]
    type aa = u64;

    let nanosecond: NanoSecond = 6 as aa;

    let mut counter = 0_i128;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    assert_eq!(result, 20);

    let mut n = 1;
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
        n += 1;
    }

    for n in 1..100 {
        if n % 15 == 0 {
            println!("1fizz1buzz");
        } else if n % 3 == 0 {
            println!("1fizz");
        } else if n % 5 == 0 {
            println!("1buzz");
        } else {
            println!("{}", n);
        }
    }

    let pair = (0, -2);

    match pair {
        (0, y) => {
            println!("First is `0` and `y` is `{:?}`", y);
        }
        (x, 0) => {
            println!("`x` is `{:?}` and last is `0`", x);
        }
        _ => {
            println!("It doesn't matter what they are");
        }
    }

    let reference = &4;

    match reference {
        &var => {
            println!("{} destructuring", var);
        }
    }

    match *reference {
        var => {
            println!("{} deref", var);
        }
    }

    let value = 5;
    let mut mut_value = 6;

    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    match mut_value {
        ref mut m => {
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        }
    }

    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    let foo = Foo { x: (1, 2), y: 3 };

    let Foo { x: (a, b), y } = foo;

    println!("a = {}, b = {},  y = {} ", a, b, y);

    let Foo { y: i, x: j } = foo;
    println!("i = {:?}, j = {:?}", i, j);

    let Foo { y, .. } = foo;
    println!("y = {}", y);

    let pair = (2, -2);
    println!("Tell me about {:?}", pair);

    match pair {
        (x, y) if x == y => println!("they are same"),
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation..."),
    }

    fn age() -> i32 {
        20
    }

    match age() {
        0 => println!("I'm not born yet I guess"),
        n @ 1...12 => println!("I'm a child of age {:?}", n),
        n @ 13...19 => println!("I'm a teen of age {:?}", n),
        n => println!("I'm an old person of age {:?}", n),
    }

    let optional = Some(7);

    match optional {
        Some(i) => {
            println!("This is a really long string and `{:?}`", i);
        }
        _ => {}
    }

    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    if let Some(i) = number {
        println!("Match {:?}!", i);
    }

    if let Some(i) = letter {
        println!("Match {:?}!", i);
    } else {
        println!("not a number");
    }

    let i_like_letter = false;
    if let Some(i) = emoticon {
        println!("Match {:?}!", i);
    } else if i_like_letter {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        println!("I don't like letters. Let's go with an emoticon :)!");
    }

    let mut optional = Some(0);

    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
    }

    fizzbuzz_to(20);
    test_7();
    test7_1();
    test7_2();
    test7_3();
    test7_4();
    test7_5();
    //    struct Functor {
    //        a: i32,
    //    }
    //
    //    let square = |x : i32| x * x;
    //    impl FnOnce for Functor {
    //        type Output = i32;
    //
    //        fn call_once(self, args: (i32)) -> Self::Output {
    //            self.a + args
    //        }
    //    }
    //
    //    let functor = Functor{a:1};
    //    functor(1);
    //
    //    let letter_count = ["apple", "banana"].iter().map(|str| { str.len() }).sum();
    //    let letter_count = ["apple", "banana"].iter().map(str::len).sum();
    //
    //    let at_least_10: Vec<_> = [8, 9, 10, 11].iter().map(|x| { cmp::min(x, 10) }).collect();
    //    let at_least_10: Vec<_> = [8, 9, 10, 11].iter().map(cmp::min(_, 10)).collect();

    func1();
    //    func2();

    async fn hello_world() {
        println!("hello world!");
    }
    let future1 = hello_world();
    //futures::executor::block_on(future1);

    use std::thread;
    static mut V: i32 = 0;
    fn un_safe_seq() -> i32 {
        unsafe {
            V += 1;
            V
        }
    }

    let child = thread::spawn(move || {
        for _ in 0..10 {
            un_safe_seq();
            unsafe {
                println!("child : {}", V);
            }
        }
    });

    for _ in 0..10 {
        un_safe_seq();
        unsafe {
            println!("main : {}", V);
        }
    }
    child.join().unwrap();

    println!("+++++");

    let mut v = vec![];
    for id in 0..5 {
        let child = thread::spawn(move || {
            println!("in child: {}", id);
        });
        v.push(child);
    }
    println!("in main: join before:");
    for child in v {
        //println!(child.);
        child.join().unwrap();
    }

    println!("in main: join after:");
    println!("+++++");
    let mut v = vec![];
    use std::panic;
    use std::thread::{current, Builder};

    for id in 0..5 {
        let thread_name = format!("child-{}", id);
        let size: usize = 17 * 1024;
        let builder = Builder::new().name(thread_name).stack_size(size);
        let child = builder
            .spawn(move || {
                println!("in child : {}", id);
                if id == 3 {
                    panic::catch_unwind(|| {
                        panic!("oh no!");
                    });
                    println!("in {} do sm", current().name().unwrap());
                }
            })
            .unwrap();
        v.push(child);
    }
    for child in v {
        child.join().unwrap();
    }

    use std::cell::RefCell;
    //use std::thread;

    thread_local!(static FOO: RefCell<u32> = RefCell::new(1));
    FOO.with(|f| {
        assert_eq!(*f.borrow(), 1);
        *f.borrow_mut() = 2;
    });
    thread::spawn(|| {
        FOO.with(|f| {
            assert_eq!(*f.borrow(), 1);
            *f.borrow_mut() = 3;
        });
    });
    FOO.with(|f| {
        assert_eq!(*f.borrow(), 2);
    });

    use std::time::Duration;
    let parked_thread = thread::Builder::new()
        .spawn(|| {
            println!("Parking thread");
            thread::park();
            println!("thread unparked");
        })
        .unwrap();
    thread::sleep(Duration::from_millis(10));
    println!("unpark the thread");
    parked_thread.thread().unpark();
    parked_thread.join().unwrap();

    use std::sync::{Arc, Mutex};

    let s = Arc::new(Mutex::new("hello".to_string()));
    let mut v = vec![];

    for _ in 0..3 {
        let s_clone = s.clone();
        let child = thread::spawn(move || {
            let mut s_clone = s_clone.lock().unwrap();
            s_clone.push_str(" world");
            println!("{}", s_clone);
        });
        v.push(child);
    }

    for child in v {
        child.join().unwrap();
    }

    extern crate rand;

    fn flip_simulate(target_filp: u64, total_flip: Arc<Mutex<u64>>) {
        let mut continue_positive = 0;
        let mut iter_counts = 0;
        while continue_positive < target_filp {
            iter_counts += 1;
            let pro_or_con = rand::random();
            if pro_or_con {
                continue_positive += 1;
            } else {
                continue_positive = 0;
            }
        }
        println!("iter_counts: {}", iter_counts);
        let mut total_flips = total_flip.lock().unwrap();
        *total_flips += iter_counts;
    }

    let mut total_flips = Arc::new(Mutex::new(0));
    let completed = Arc::new(Mutex::new(0));
    let runs = 8;
    let target_flips = 10;
    for _ in 0..runs {
        let total_flips = total_flips.clone();
        let completed = completed.clone();
        thread::spawn(move || {
            flip_simulate(target_flips, total_flips);
            let mut completed = completed.lock().unwrap();
            *completed += 1;
        });
    }
    loop {
        let completed = completed.lock().unwrap();
        if *completed == runs {
            let total_flips = total_flips.lock().unwrap();
            println!("Final average: {}", *total_flips / *completed);
            break;
        }
    }

    use std::sync::RwLock;
    let lock = RwLock::new(5);
    {
        let r1 = lock.read().unwrap();
        let r2 = lock.read().unwrap();
        assert_eq!(*r1, 5);
        assert_eq!(*r2, 5);
        std::mem::drop(r1);
        std::mem::drop(r2);
        let mut w = lock.write().unwrap();
        *w += 1;
        assert_eq!(*w, 6);
    }
    {
        let mut w = lock.write().unwrap();
        *w += 1;
        assert_eq!(*w, 7);
    }

    use std::sync::Barrier;
    let mut handles = Vec::with_capacity(5);
    let barrier = Arc::new(Barrier::new(5));
    for _ in 0..5 {
        let c = barrier.clone();
        handles.push(thread::spawn(move || {
            println!("before wait");
            c.wait();
            println!("after wait");
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }

    use std::sync::Condvar;
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair_clone = pair.clone();
    thread::spawn(move || {
        let &(ref lock, ref cvar) = &*pair_clone;
        let mut started = lock.lock().unwrap();
        thread::sleep(Duration::from_secs(5));
        *started = true;
        cvar.notify_one();
    });
    let &(ref lock, ref cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        println!("{}", started);
        started = cvar.wait(started).unwrap();
        println!("{}", started);
    }

    use std::sync::mpsc::channel;
    let (tx, rx) = channel();
    for i in 0..10 {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send(i).unwrap();
        });
    }
    for _ in 0..10 {
        let j = rx.recv().unwrap();
        println!("recv j = {}", j);
        assert!(0 <= j && j < 10);
    }

    use std::sync::mpsc::sync_channel;
    let (tx, rx) = sync_channel(1);
    tx.send(1).unwrap();
    thread::spawn(move || {
        println!("thread send 2");
        tx.send(2).unwrap();
    });

    assert_eq!(rx.recv().unwrap(), 1);
    println!("get 1");
    assert_eq!(rx.recv().unwrap(), 2);
    println!("get 2");

    //do_pow();
    //async1::test1();
}
