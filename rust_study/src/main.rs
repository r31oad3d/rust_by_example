mod trait_object;
use std::fmt::{Debug, Display};

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("Read more from {}", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub location: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn notify(item: impl Summary) {
    println!("Breaking News! {}", item.summarize());
}

pub fn notify_another<T: Summary>(item: T) {
    println!("Breaking News! {}", item.summarize());
}

pub fn notify_2(_item1: impl Summary, _item2: impl Summary) {}

pub fn notify_3<T: Summary>(_item1: T, _item2: T) {}

pub fn notify_4(_item: impl Display + Summary) {}

pub fn notify_5<T: Display + Summary>(_item: T) {}

pub fn some_function<T: Display + Clone, U: Clone + Debug>(
    _t: T,
    _u: U,
) -> i32 {
    1
}

pub fn some_function_2<T, U>(_t: T, _u: U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    2
}

fn print_sum(v: &Vec<i32>) {
    println!("sum{}", v[0] + v[1]);
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best
    hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

    let mut v = Vec::new();
    for i in 1..1000 {
        v.push(i);
    }
    println!("1    {}", v[1]);
    print_sum(&v);
    println!("2    {}", v[1]);
    println!("11");
}
