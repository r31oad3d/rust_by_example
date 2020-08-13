macro_rules! foo {
    (@as_expr $e:expr) => { $e };
    ($($tts:tt)*) => {
        foo!(@as_expr $($tts)*)
    };
}
