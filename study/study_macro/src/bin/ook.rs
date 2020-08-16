#![recursion_limit = "256"]
#![allow(deprecated)]
#[path = "../macros/ook_macro.rs"]
#[macro_use]
mod ook_macro;

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
fn main() {
    let _ = Ook!(
        Ook. Ook?  Ook. Ook.  Ook. Ook.  Ook. Ook.
        Ook. Ook.  Ook. Ook.  Ook. Ook.  Ook. Ook.
        Ook. Ook.  Ook. Ook.  Ook! Ook?  Ook? Ook.
        Ook. Ook.  Ook. Ook.  Ook. Ook.  Ook. Ook.
        Ook. Ook.  Ook. Ook.  Ook. Ook.  Ook. Ook.
        Ook. Ook?  Ook! Ook!  Ook? Ook!  Ook? Ook.
        Ook! Ook.  Ook. Ook?  Ook. Ook.  Ook. Ook.
        Ook. Ook.  Ook. Ook.  Ook. Ook.  Ook. Ook.
        Ook. Ook.  Ook! Ook?  Ook? Ook.  Ook. Ook.
        Ook. Ook.  Ook. Ook.  Ook. Ook.  Ook. Ook?
        Ook! Ook!  Ook? Ook!  Ook? Ook.  Ook. Ook.
        Ook! Ook.  Ook. Ook.  Ook. Ook.  Ook. Ook.
        Ook. Ook.  Ook. Ook.  Ook. Ook.  Ook. Ook.
        Ook! Ook.  Ook! Ook.  Ook. Ook.  Ook. Ook.
        Ook. Ook.  Ook! Ook.  Ook. Ook?  Ook. Ook?
        Ook. Ook?  Ook. Ook.  Ook. Ook.  Ook. Ook.
        Ook. Ook.  Ook. Ook.  Ook. Ook.  Ook. Ook.
        Ook. Ook.  Ook! Ook?  Ook? Ook.  Ook. Ook.
        Ook. Ook.  Ook. Ook.  Ook. Ook.  Ook. Ook?
        Ook! Ook!  Ook? Ook!  Ook? Ook.  Ook! Ook.
        Ook. Ook?  Ook. Ook?  Ook. Ook?  Ook. Ook.
        Ook. Ook.  Ook. Ook.  Ook. Ook.  Ook. Ook.
        Ook. Ook.  Ook. Ook.  Ook. Ook.  Ook. Ook.
        Ook. Ook.  Ook! Ook?  Ook? Ook.  Ook. Ook.
        Ook. Ook.  Ook. Ook.  Ook. Ook.  Ook. Ook.
        Ook. Ook.  Ook. Ook.  Ook. Ook.  Ook. Ook.
        Ook. Ook?  Ook! Ook!  Ook? Ook!  Ook? Ook.
        Ook! Ook!  Ook! Ook!  Ook! Ook!  Ook! Ook.
        Ook? Ook.  Ook? Ook.  Ook? Ook.  Ook? Ook.
        Ook! Ook.  Ook. Ook.  Ook. Ook.  Ook. Ook.
        Ook! Ook.  Ook! Ook!  Ook! Ook!  Ook! Ook!
        Ook! Ook!  Ook! Ook!  Ook! Ook!  Ook! Ook.
        Ook! Ook!  Ook! Ook!  Ook! Ook!  Ook! Ook!
        Ook! Ook!  Ook! Ook!  Ook! Ook!  Ook! Ook!
        Ook! Ook.  Ook. Ook?  Ook. Ook?  Ook. Ook.
        Ook! Ook.  Ook! Ook?  Ook! Ook!  Ook? Ook!
        Ook. Ook.  Ook. Ook.  Ook. Ook.  Ook. Ook.
        Ook. Ook.  Ook. Ook.  Ook. Ook.  Ook. Ook.
        Ook. Ook.  Ook. Ook.  Ook! Ook.
    );
}
