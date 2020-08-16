mod a;

use a::d;

mod a1 {
    pub const I: i32 = 3;

    use self::b1::semisecret;

    pub fn bar(z: i32) -> i32 {
        semisecret(I) * z
    }

    pub fn foo(y: i32) -> i32 {
        semisecret(I) * y
    }

    mod b1 {
        pub(in crate::a1) use self::c1::semisecret;
        mod c1 {
            const J: i32 = 4;
            pub(in crate::a1) fn semisecret(x: i32) -> i32 {
                x + J
            }
        }
    }
}

fn main() {
    d::print_ddd();
}
