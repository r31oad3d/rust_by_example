#![allow(dead_code, unused_variables)]
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportExcerpt<'a> {
    part: &'a str,
}

struct Foo<'a, 'b> {
    x: &'a i32,
    y: &'b i32,
}

struct Context<'a>(&'a str);

struct Parser<'b> {
    context: &'b Context<'b>,
}

impl<'c> Parser<'c> {
    fn parse(&self) -> Result<(), &'c str> {
        Err(&self.context.0[1..])
    }
}

fn parse_context<'d>(context: &'d Context<'d>) -> Result<(), &'d str> {
    Parser { context }.parse()
}

fn main() {
    let s1 = String::from("111");
    let s2 = "222";

    let result = longest(s1.as_str(), s2);

    let novel = String::from("call me Ishmael. Some years ago...");
    let first_sentence = novel.split(".").next().expect("Cound not find a '.'");

    let i = ImportExcerpt {
        part: first_sentence,
    };

    let x = 6;
    let m;
    {
        let y = 6;
        let f = Foo { x: &x, y: &y };
        m = f.x;
    }
    println!("{}", m);
}
