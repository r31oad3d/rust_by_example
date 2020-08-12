fn main() {
    let mut x = 5;
    println!("the value of x is {}", x);
    x = 6;
    println!("the value of x is {}", x);

    const MAX_POINT: u32 = 100000;

    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("the value of y is {}", y);

    let a = 2.0;
    let b: f32 = 3.0;

    let c = 'a';

    let heart_eyed_cat = '😻';

    println!("the value of heart_eyed_cat is {}", heart_eyed_cat);

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x1, y1, z1) = tup;
    println!("the value of y1 is {}", y1);

    let xx: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = xx.0;
    let six_point_four = xx.1;
    let one = xx.2;

    println!("the value of five_hundred is {}", five_hundred);

    let array = [1, 2, 3, 4, 5];

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let array1: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{}", months[1]);
}
