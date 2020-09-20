extern crate base64;
extern crate image;
extern crate itertools;

fn main() {
    println!("{:?}", (1..=10).filter(|x| x % 2 == 0).size_hint());
    println!("{:?}", (1..=7).zip(1..=6).collect::<Vec<_>>());

    let ret1 = ["1", "2", "a"]
        .iter()
        .map(|xx| xx.parse::<i32>())
        .inspect(|num| {
            if let Err(ref e) = *num {
                println!("error: {}", e)
            }
        })
        .filter_map(|ret| ret.ok())
        // .filter(|ret| ret.is_ok())
        // .map(|ret| ret.unwrap())
        .sum::<i32>();
    println!("ret1:{:?}", ret1);

    for elt in itertools::merge(&[1, 2, 3], &[2, 3, 4]) {
        println!("!!!!!!!!!!!!!!!!");
        println!("{:?}", elt);
    }
    let mut merge1 = itertools::merge(&[1, 2, 3], &[2, 3, 4]);
    println!("merge1:{:?}", merge1.clone().sum::<i32>());
    println!(
        "merge1:{:?}",
        merge1.clone().map(|x| x * x).collect::<Vec<_>>()
    );

    //let ret2 = (&merge1 as &mut dyn Iterator<Item=&i32>).count();
    let ret2 = merge1.product::<i32>();
    println!("ret2:{:?}", ret2);

    println!("ret2");

    let a1 = b"hello world";
    let b1 = base64::encode("1");
    println!("{:?}", b1);
}
