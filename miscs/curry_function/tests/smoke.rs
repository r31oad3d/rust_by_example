#![feature(type_alias_impl_trait)]

#[curry_function::curry]
fn add(x: u32, y: u32, z: u32, a: u32, b: u32, c: u32) -> u32 {
    x + y + z + a + b + c
}

#[test]
fn works() {
    assert_eq!(39, add(4)(5)(6)(7)(8)(9));
}
