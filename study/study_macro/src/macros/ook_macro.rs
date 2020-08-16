//[0:30000]
//[72]H[101]e[108]l[108]l[111]o[32] [87]W[111]o[114]r[108]l[100]d[33]![10]
// Ook. Ook? - 指针增。
// Ook? Ook. - 指针减。
// Ook. Ook. - 所指单元增。
// Ook! Ook! - 所指单元减。
// Ook! Ook. - 将所指单元写至标准输出。
// Ook. Ook! - 从标准输入读至所指单元。
// Ook! Ook? - 进入循环。
// Ook? Ook! - 如果所指单元非零，跳回循环起始位置；否则，继续执行。
#[macro_export]
macro_rules! Ook {
    (@start $($Ooks:tt)*) => {{
        type CellType = u8;
        const MEM_SIZE: usize = 30_000;
        fn ook() -> ::std::io::Result<Vec<CellType>> {
            use ::std::io;
            use ::std::io::prelude::*;
            use std::thread;
            use std::time::Duration;
            fn _re() -> io::Error {
                io::Error::new(io::ErrorKind::Other, String::from("ran out of input"))
            }

            fn _inc(a: &mut [u8], i: usize) {
                let c = &mut a[i];
                *c = c.wrapping_add(1);
            }

            fn _dec(a: &mut [u8], i: usize) {
                let c = &mut a[i];
                *c = c.wrapping_sub(1);
            }

            let _r = &mut io::stdin();
            let _w = &mut io::stdout();

            let mut _a:Vec<CellType> = Vec::with_capacity(MEM_SIZE);
            _a.extend(::std::iter::repeat(0).take(MEM_SIZE));
            let mut _i = 0;
            {
                let _a = &mut *_a;
                Ook!(@e (_a, _i, _inc, _dec, _r, _w, _re); ($($Ooks)*));
            }
            println!("{:?}",_a);
            Ok(_a)
        }
        ook()
    }};

    // extraction point
    (@e $syms:tt; ()) => {};

    // Ook. Ook? - 指针增。
    (@e ($a:expr, $i:expr, $inc:expr, $dec:expr, $r:expr, $w:expr, $re:expr);
        (Ook. Ook? $($tail:tt)*))
    => {
        $i = ($i + 1) % MEM_SIZE;
        Ook!(@e ($a, $i, $inc, $dec, $r, $w, $re); ($($tail)*));
    };
    // Ook? Ook. - 指针减。
    (@e ($a:expr, $i:expr, $inc:expr, $dec:expr, $r:expr, $w:expr, $re:expr);
        (Ook? Ook. $($tail:tt)*))
    => {
        $i = if $i == 0 { MEM_SIZE } else { $i } - 1;
        Ook!(@e ($a, $i, $inc, $dec, $r, $w, $re); ($($tail)*));
    };
    // Ook. Ook. - 所指单元增。
    (@e ($a:expr, $i:expr, $inc:expr, $dec:expr, $r:expr, $w:expr, $re:expr);
        (Ook. Ook. $($tail:tt)*))
    => {
        $inc($a, $i);
        Ook!(@e ($a, $i, $inc, $dec, $r, $w, $re); ($($tail)*));
    };
    // Ook! Ook! - 所指单元减。
    (@e ($a:expr, $i:expr, $inc:expr, $dec:expr, $r:expr, $w:expr, $re:expr);
        (Ook! Ook! $($tail:tt)*))
    => {
        $dec($a, $i);
        Ook!(@e ($a, $i, $inc, $dec, $r, $w, $re); ($($tail)*));
    };
    // Ook! Ook. - 将所指单元写至标准输出。
    (@e ($a:expr, $i:expr, $inc:expr, $dec:expr, $r:expr, $w:expr, $re:expr);
        (Ook! Ook. $($tail:tt)*))
    => {
        //print!("{:?}", &$a[$i .. $i+1]);
        //thread::sleep(Duration::from_millis(100));
        //r#try!($w.write_all(&$a[$i .. $i+1]));
        $w.write_all(&$a[$i .. $i+1])?;
        //r#try!($w.flush());
        Ook!(@e ($a, $i, $inc, $dec, $r, $w, $re); ($($tail)*));
    };
    // Ook. Ook! - 从标准输入读至所指单元。
    (@e ($a:expr, $i:expr, $inc:expr, $dec:expr, $r:expr, $w:expr, $re:expr);
        (Ook. Ook! $($tail:tt)*))
    => {
        r#try!(
            match $r.read(&mut $a[$i .. $i+1]) {
                Ok(0) => Err($re()),
                ok @ Ok(..) => ok,
                err @ Err(..) => err
            }
        );
        Ook!(@e ($a, $i, $inc, $dec, $r, $w, $re); ($($tail)*));
    };
    // Ook! Ook? - 进入循环。
    // Ook? Ook! - 如果所指单元非零，跳回循环起始位置；否则，继续执行。
    (@e ($a:expr, $i:expr, $inc:expr, $dec:expr, $r:expr, $w:expr, $re:expr);
        (Ook! Ook? $($tail:tt)*))
    => {
        while $a[$i] != 0 {
            Ook!(@x ($a, $i, $inc, $dec, $r, $w, $re); (); (); ($($tail)*));
        }
        Ook!(@s ($a, $i, $inc, $dec, $r, $w, $re); (); ($($tail)*));
    };
    (@x $syms:tt; ($($depth:tt)*); ($($buf:tt)*);
        (Ook! Ook? $($tail:tt)*))
    => {
        // 嵌套变深
        Ook!(@x $syms; (@ $($depth)*); ($($buf)* Ook! Ook?); ($($tail)*));
    };
    (@x $syms:tt; (@ $($depth:tt)*); ($($buf:tt)*);
        (Ook? Ook! $($tail:tt)*))
    => {
        // 嵌套变浅
        Ook!(@x $syms; ($($depth)*); ($($buf)* Ook? Ook!); ($($tail)*));
    };
    (@x $syms:tt; (); ($($buf:tt)*);
        (Ook? Ook! $($tail:tt)*))
    => {
        // 最外层的循环已被处理完毕，现在转而处理缓存到的标记。
        Ook!(@e $syms; ($($buf)*));
    };
    (@x $syms:tt; $depth:tt; ($($buf:tt)*);
        (Ook $op0:tt Ook $op1:tt $($tail:tt)*))
    => {
        Ook!(@x $syms; $depth; ($($buf)* Ook $op0 Ook $op1); ($($tail)*));
    };
    // End of loop.
    (@s $syms:tt; ();
        (Ook? Ook! $($tail:tt)*))
    => {
        Ook!(@e $syms; ($($tail)*));
    };
    // Enter nested loop.
    (@s $syms:tt; ($($depth:tt)*);
        (Ook! Ook? $($tail:tt)*))
    => {
        Ook!(@s $syms; (@ $($depth)*); ($($tail)*));
    };
    // Exit nested loop.
    (@s $syms:tt; (@ $($depth:tt)*);
        (Ook? Ook! $($tail:tt)*))
    => {
        Ook!(@s $syms; ($($depth)*); ($($tail)*));
    };
    // Not a loop opcode.
    (@s $syms:tt; ($($depth:tt)*);
        (Ook $op0:tt Ook $op1:tt $($tail:tt)*))
    => {
        Ook!(@s $syms; ($($depth)*); ($($tail)*));
    };

    (/*@entry*/$($Ooks:tt)*) => {
        Ook!(@start $($Ooks)*)
    };
}
