fn main() {
    println!("cargo:rustc-link-search=/Users/johnj/Development/code/rust/rust_by_example/target/release/");
    println!("cargo:rustc-link-lib=dylib=testlib2");
    println!("cargo:warning=check check check");
}
