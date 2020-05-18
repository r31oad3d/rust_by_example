//bitflag! {
//    flags Color: u8 {
//        const RED = 0b0001,
//        const GREEN = 0b0010,
//        const BLUE = 0b0100,
//        const BRIGHT = 0b1000,
//    }
//}
//
//lazy_static! {
//    static ref FIB_100: u32 = {
//        fn fib(a: u32) -> u32 {
//            match a {
//                0 => 0,
//                1 => 1,
//                a => fib(a-1) + fib(a-2),
//            }
//        }
//
//        fib(100)
//    };
//}

macro_rules! four {
    () => {
        1 + 3
    };
}

macro_rules! gibberish {
    (4 fn ['spang "whammo"] @_@) => {};
}

macro_rules! times_five {
    ($e:expr) => {
        5 * $e
    };
}

macro_rules! multiply_add {
    ($a:expr, $b:expr, $c:expr) => {
        $a * ($b + $c)
    };
}

macro_rules! vec_strs {
    ($($element:expr),*) => {
        {
            let mut v = Vec::new();
            $(
                v.push(format!("{}", $element));
            )*
            v
        }
    };
}

macro_rules! capture_expr_then_stringify {
    ($e:expr) => {
        stringify!($e)
    };
}

macro_rules! capture_then_match_tokens {
    ($e:expr) => {
        match_tokens!($e)
    };
}
macro_rules! match_tokens {
    ($a:tt + $b:tt) => {
        "got an addition"
    };
    ($i:ident) => {
        "got an identifier"
    };
    ($($other:tt)*) => {
        "got something else"
    };
}

macro_rules! using_a1 {
    ($e:expr) => {{
        let a1 = 42;
        $e
    }};
}

macro_rules! what_is {
    (self) => {
        "the keyword is 'self'"
    };
    ($i:ident) => {
        concat!("the identifier '", stringify!($i), "'")
    };
}

macro_rules! call_with_ident {
    ($c:ident($i:ident)) => {
        $c!($i)
    };
}

macro_rules! make_mutable {
    ($i:ident) => {let mut $i = $i};
}

macro_rules! double_method {
    ($self_:ident, $body:expr) => {
        fn double(mut $self_) -> Dummy {
            $body
        }
    }
}
