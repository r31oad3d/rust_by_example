macro_rules! my_map {
    ($($key:expr => $value:expr),*) => {
        {
            let mut m = HashMap::new();
            $(m.insert($key,$value);)*
            m
        }
    };
}
macro_rules! one {
    () => {
        1
    };
}

macro_rules! two {
    () => {
        1 + one!()
    };
}

macro_rules! three {
    () => {
        1 + two!()
    };
}

macro_rules! four {
    () => {
        1 + three!()
    };
}

// macro_rules! add_self {
//     ($now:expr, $target:expr) => {
//         if $now == $target {
//                 add_self!($now);
//          } else {
//                 add_self!($now+1,$target);
//         }
//     };
//     ($now:expr) => {$now};
// }

macro_rules! capture_then_match_tokens {
    ($e:expr) => {
        match_tokens!($e);
        // println!("{:?}", $e);
    };
}
macro_rules! match_tokens {
    ($a:tt + $b:tt) => {
        "got an addition"
    };
    (($i:ident)) => {
        "got an identifier"
    };
    ($($other:tt)*) => {
        "got something else"
    };
}

macro_rules! using_a {
    ($e:expr) => {{
        let a = 42;
        $e
    }};
}

macro_rules! using_a_v2 {
    ($a:ident, $e:expr) => {{
        let $a = 42;
        $e
    }};
}

macro_rules! write_html {
    ($w:expr, ) => (());

    ($w:expr, $e:tt) => (write!($w, "{}", $e));

    ($w:expr, $tag:ident [ $($inner:tt)* ] $($rest:tt)*) => {{
        write!($w, "<{}>", stringify!($tag));
        write_html!($w, $($inner)*);
        write!($w, "</{}>", stringify!($tag));
        write_html!($w, $($rest)*);
    }};
}
