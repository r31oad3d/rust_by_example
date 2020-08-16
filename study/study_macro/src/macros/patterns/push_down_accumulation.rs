macro_rules! init_array {
    (@accum (0, $_e:expr) -> ($($body:tt)*))
        => { init_array!(@as_expr [$($body)*]) };
    (@accum (1, $e:expr) -> ($($body:tt)*))
        => { init_array!(@accum (0, $e) -> ($($body)* $e,)) };
    (@accum (2, $e:expr) -> ($($body:tt)*))
        => { init_array!(@accum (1, $e) -> ($($body)* $e,)) };
    (@accum (3, $e:expr) -> ($($body:tt)*))
        => { init_array!(@accum (2, $e) -> ($($body)* $e,)) };
    (@as_expr $e:expr) => {$e};
    [$e:expr; $n:tt] => {
        {
            let e = $e;
            init_array!(@accum ($n, e.clone()) -> ())
        }
    };
}

macro_rules! init_array_another_wat {
    (@accum 0, $_e:expr) => {/* empty */};
    (@accum 1, $e:expr) => {$e};
    (@accum 2, $e:expr) => {$e, init_array_another_wat!(@accum 1, $e)};
    (@accum 3, $e:expr) => {$e, init_array_another_wat!(@accum 2, $e)};
    [$e:expr; $n:tt] => {
        {
            let e = $e;
            [init_array_another_wat!(@accum $n, e)]
        }
    };
}
