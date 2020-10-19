#![recursion_limit = "64"]
#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(clippy::single_component_path_imports)]
#![allow(unused_macros)]
#![allow(unused_mut)]
#![allow(dead_code)]
#[macro_use]
mod macros;
#[macro_use]
mod macros1;
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;
use std::collections::HashMap;

fn macro1_main() {
    struct Dummy(i32);

    impl Dummy {
        double_method! {
        self,
        {
            self.0 *= 2;
            self
        }
    }
    }
    //let color = vec![RED,GREEN, BLUE];
    let a = 3 * four!();
    let b = multiply_add!(1, 2, 3);
    println!("a={}", a);
    println!("b={}", b);
    let v = vec_strs![1 + 1, 2 + 2];
    assert_eq!(&*v, &["2", "4"]);
    println!("{:?}", stringify!(dummy(2 * (1 + (3)))));
    println!("{:?}", capture_expr_then_stringify!(dummy(2 * (1 + (3)))));

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

    //let four = using_a1!(a1/10);

    println!("{}", what_is!(self));
    println!("{}", call_with_ident!(what_is(self)));
}

fn main() {



    let ret = my_map!(
        "a" => "1",
        "b" => "2",
        "c1" => "3",
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
    let ret7_1 = add_one_by_one_v2_1!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10,);
    let ret7_2 = add_one_by_one_v2_2!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
    let ret7_3 = add_one_by_one_v2_3!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
    // let ret7 = { 1 + (2 + (3 + (4 + (5 + (6 + (7)))))) };
    println!("{:?}", ret7);
    println!("{:?}", ret7_1);
    #[derive(HelloMacro)]
    struct A;
    A::hello_macro();

    let ret8 = add_one_by_one_v3!();
    println!("{:?}", ret8);

    let ret9: HashMap<&str, i32> = my_map_v2! {"a" => 1, "b"=>2};
    let ret9: HashMap<&str, i32> = my_map_v3!("a" => 1);
    let ret10: HashMap<&str, i32> = my_map!();

    recognise_tree!(expand_to_larch!());
    call_with_larch!(recognise_tree);

    callback!(callback(callback(println("haha"))));

    let a = 42;
    let b = "Ho-dee-oh-di-oh-di-oh!";
    let c = (false, 2, 'c');
    mixed_rules!(
        trace a;
        trace b;
        trace c;
        trace b = "They took her where they put the crazies.";
        trace b;
    );
    println!(concat!(stringify!(a), " = {:?}"), a);

    assert_eq!(foo!(42), 42);
    // foo!(42 43);

    //foo!(1);
    let strings: [String; 3] = init_array![String::from("hi!"); 3];
    // let strings: [String; 3] = init_array_another_wat![String::from("hi!"); 3];
    assert_eq!(tuple_default!(i32, bool, String), (0, false, String::new()));
    let tup1: (i32, bool, String) = tuple_default!(i32, bool, String);
    let tup2: (i32, bool, String) = tuple_default_v2!(i32, bool, String);

    //let tup2: (i32, bool, String) = (<i32>::default(), <bool>::default(), <String>::default());
    //let tup2: (i32, bool, String) = (<i32>::default(), <bool>::default(), <String>::default());
    // println!("{:?}",<i32>::default());

    println!(struct_name!(
        pub struct Jim;
    ));
    println!(struct_name!(
        struct Jim;
    ));
    println!(struct_name!(pub pub pub struct Jim;));

    println!(struct_name_v2!(
        pub struct Jim;
    ));
    println!(struct_name_v2!(
        struct Jim;
    ));
    //println!(struct_name_v2!(pub pub struct Jim;));

    #[derive(Debug, Eq, PartialEq)]
    struct Dummy(i32);

    newtype! { pub struct Dummy(i32); };

    assert_eq!(Dummy::new(42), Dummy(42));

    // newtype_v2! { struct Dummy(i32); };

    // assert_eq!(Dummy::new(42), Dummy(42));
    let equals_zero =
        abacus!((++-+-+++--++---++----+-+----------++++++++++-+-+-+) -> ());
    let equals_zero_v2 = abacus_v2!(++-+-++++-----+-++-);
    println!("{:?}", equals_zero_v2);
    //assert_eq!(equals_zero, true);
    const COUNT: u32 = count_idents!(A, B, C);
    println!("{:?}", COUNT);

    as_item! {struct Dummy1;}
    as_stmt!(let as_pat!(_) = as_expr!(42));
    count_tts!(0 1 2);
    count_tts_v4!(0 1 2);

    let ret11 = parse_unitary_variants!(
         enum Dummy { A=1, B=3, C=5 }
         => stringify(variants:)
    );
    println!("{:?}", ret11);
}
