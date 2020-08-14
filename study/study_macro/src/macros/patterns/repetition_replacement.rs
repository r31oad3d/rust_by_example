macro_rules! replace_expr {
    ($_t:tt $sub:expr) => {
        $sub
    };
}

macro_rules! tuple_default {
    ($($tup_tys:ty),*) => {
        (
            $(
                replace_expr!(($tup_tys) Default::default()),
            )*
        )
    };
}

macro_rules! tuple_default_v2 {
    ($($tup_tys:ty),*) => {
        (
            $(
                <$tup_tys>::default(),
            )*
        )
    };
}
