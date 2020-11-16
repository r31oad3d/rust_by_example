fn main() {
    let c = '\u{CA0}';
    println!("{}", c);


    let s = r######""""#"""""######;
    println!("{}", s);


    let s1 = "一二三";
    // println!("{}", &s1[0..1]);
    println!("{}", &s1[0..3]);

    let s21 = "一";
    let s22 = "二";
    println!("{:-}", s21 < s22);

    println!("Hello World!");
}
