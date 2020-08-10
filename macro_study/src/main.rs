#![recursion_limit = "256"]

#[macro_use]
mod macros;

use std::collections::HashMap;

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
}
