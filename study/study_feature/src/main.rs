#[cfg(feature = "f2")]
use rand::random;

#[cfg(feature = "f1")]
fn feature1() {
    println!("feature1");
}

#[cfg(feature = "f2")]
fn feature2() {
    println!("feature2");
}

fn main() {
    #[cfg(feature = "f1")]
    feature1();
    if cfg!(feature = "f1") {
        println!("feature1!!!");
    }
    #[cfg(feature = "f2")]
    feature2();
}
