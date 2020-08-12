#![recursion_limit = "20"]

#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(clippy::single_component_path_imports)]



#[macro_use]
mod macros;

use std::collections::HashMap;
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

fn main() {
    let ret = my_map!(
        "a" => "1",
        "b" => "2",
        "c1" => "3",
        "c2" => "3",
        "c3" => "3",
        "c4" => "3",
        "c5" => "3",
        "c" => "3",
        "d" => "4"
    )
    .iter_mut()
    .map(|(k, v)| (*k, v.parse::<i32>().unwrap() * 2))
    .collect::<HashMap<&str, i32>>();
    println!("{:?}", ret);

    let ret2 = 1 + four!();
    // add_self!(1,3);

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
    let a = 10;
    let ret3 = using_a!(a / 10);
    println!("{:?}", ret3);

    // let b = 10;
    // let ret4 = {
    //     let b = 50;
    //     b / 10
    // };
    // println!("{:?}", ret4);

    //let a1 = 10;
    let ret5 = using_a_v2!(a1, a1 / 10);
    println!("{:?}", ret5);

    use std::fmt::Write;
    let mut out = String::new();

    write_html!(&mut out,
    html[
        head[
            title["Macros guide"]
        ]
        body[
            h1["Macros are the best!"]
            h2["hahaha"]
            script["script"]
            A[b[]]
        ]
    ]);
    println!("{:?}", out);
    //    assert_eq!(
    //        out,
    //        "<html><head><title>Macros guide</title></head>\
    // <body><h1>Macros are the best!</h1></body></html>"
    //    );

    //let mut sum = 0;
    let ret6 = add_one_by_one_v1!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
    println!("{:?}", ret6);
    // let ret7 = add_one_by_one_v2!(1,);
    // add_one_by_one_v2!(1,2,3,4,5,6,7);
    let ret7 = add_one_by_one_v2!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
    // let ret7 = { 1 + (2 + (3 + (4 + (5 + (6 + (7)))))) };
    println!("{:?}", ret7);

    #[derive(HelloMacro)]
    struct A;
    A::hello_macro();

    let ret8 = add_one_by_one_v3!();
    println!("{:?}", ret8);

    let ret9: HashMap<&str, i32> = my_map_v2!("a" => "1",);
    let ret9: HashMap<&str, i32> = my_map_v3!();
    let ret10: HashMap<&str, i32> = my_map!();
}
