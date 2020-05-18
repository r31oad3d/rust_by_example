use std::any::Any;

fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;
    }
    lhs % rhs == 0
}

fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

pub fn fizzbuzz_to(n: u32) {
    for n in 1..n + 1 {
        fizzbuzz(n);
    }
}

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn origin() -> Point {
        Point { x: 0_f64, y: 0.0 }
    }

    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    fn area(self: &Self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2_f64 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    fn translate(self: &mut Self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

struct Pair(Box<i32>, Box<i32>);

impl Pair {
    fn destroy(self) {
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);
    }
}

pub fn test_7() {
    let rectangle = Rectangle {
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));
    pair.destroy();
}

pub fn test7_1() {
    fn function(i: i32) -> i32 {
        i + 1
    }
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred = |i| i + 1;

    let i = 1;
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));
    //println!("closure_inferred: {}", closure_inferred(1.0));

    //    struct Functor {
    //        i : i32,
    //    }
    //
    //    impl Fn for functor {
    //        fn call(&self, args: i32) -> Self::Output {
    //            unimplemented!()
    //        }
    //    }
    //
    //    Functor(1);

    let one = || 1;
    println!("closure returning one: {}", one());
}

pub fn test7_2() {
    use std::mem;

    let color = "green";

    let print = || println!("color={}", color);

    print();
    print();

    let mut count = 0;

    let mut inc = || {
        count += 1;
        println!("count={}", count);
    };

    inc();
    inc();

    //println!("{:}",inc.type_id().);
    let movable = Box::new(3);
    let consume = || {
        println!("movable {:?}", movable);
        mem::drop(movable);
    };
    consume();
}

pub fn test7_3() {
    fn apply<F>(mut f: F)
    where
        F: FnMut(),
    {
        f();
    }

    fn apple_to_3<F>(f: F) -> i32
    where
        F: Fn(i32) -> i32,
    {
        f(3)
    }

    use std::mem;
    let greeting = "hello";

    let mut farewell = "goodbye".to_owned();

    let diary = || {
        println!("I said {}", greeting);
    };

    farewell.push_str("!!!");

    println!("Then I screamed {}.", farewell);

    println!("Now I can sleep. zzzzz");

    mem::drop(farewell);

    apply(diary);

    let double = |x| 2 * x;

    println!("3 doubled {}", apple_to_3(double));
}

pub fn test7_4() {
    fn create_fn() -> Box<dyn Fn()> {
        let text = "Fn".to_owned();
        Box::new(move || println!("This is a: {}", text))
    }

    fn create_fnmut() -> Box<dyn FnMut()> {
        let text = "FnMut".to_owned();
        Box::new(move || println!("This is a: {}", text))
    }
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();

    fn_plain();
    fn_mut();
}

pub fn test7_5() {
    fn is_odd(n: u32) -> bool {
        n % 2 == 1
    }

    let upper = 1000;

    let mut acc = 0;

    for n in 0.. {
        let n_squared = n * n;
        if n_squared >= upper {
            break;
        } else if is_odd(n_squared) {
            acc += n_squared;
        }

        println!("imperative style: {}", acc);
    }

    let sum_of_squared_odd_numbers: u32 = (0..)
        .map(|n| n * n)
        .take_while(|&n| n < upper)
        .filter(|&n| is_odd(n))
        .fold(0, |sum, i| sum + i);
    println!("functional style: {}", sum_of_squared_odd_numbers);
}
