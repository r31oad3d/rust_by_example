macro_rules! replace_expr {
    ($_t:tt $sub:expr) => {
        $sub
    };
}

macro_rules! count_tts {
    ($($tts:tt)*) => {
        0usize $(+ replace_expr!($tts 1usize))*
    };
}

macro_rules! count_tts_v2 {
    () => {0usize};
    ($_head:tt $($tail:tt)*) => {
        1usize + count_tts_v2!($($tail:tt)*)
    };
}

macro_rules! count_tts_v3 {
    ($_a:tt $_b:tt $_c:tt $_d:tt $_e:tt
     $_f:tt $_g:tt $_h:tt $_i:tt $_j:tt
     $_k:tt $_l:tt $_m:tt $_n:tt $_o:tt
     $_p:tt $_q:tt $_r:tt $_s:tt $_t:tt
     $($tail:tt)*)
        => {20usize + count_tts_v3!($($tail)*)};
    ($_a:tt $_b:tt $_c:tt $_d:tt $_e:tt
     $_f:tt $_g:tt $_h:tt $_i:tt $_j:tt
     $($tail:tt)*)
        => {10usize + count_tts_v3!($($tail)*)};
    ($_a:tt $_b:tt $_c:tt $_d:tt $_e:tt
     $($tail:tt)*)
        => {5usize + count_tts_v3!($($tail)*)};
    ($_a:tt
     $($tail:tt)*)
        => {1usize + count_tts_v3!($($tail)*)};
    () => {0usize};
}

macro_rules! count_tts_v4 {
    ($($tts:tt)*) => {
        <[()]>::len(&[$(replace_expr!($tts ())),*])
    };
}

macro_rules! count_idents {
    ($($idents:ident),* $(,)*) => {{
        #[allow(dead_code, non_camel_case_types)]
        enum Idents { $($idents,)* __CountIdentsLast }
        const COUNT: u32 = Idents::__CountIdentsLast as u32;
        COUNT
    }};
}
