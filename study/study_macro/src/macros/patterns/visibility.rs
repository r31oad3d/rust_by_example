macro_rules! struct_name {
    ($(pub)* struct $name:ident $($rest:tt)*) => {
        stringify!($name)
    };
}

macro_rules! struct_name_v2 {
    ($(pub)? struct $name:ident $($rest:tt)*) => {
        stringify!($name)
    };
}

macro_rules! newtype {
    {struct $name:ident($t:ty);} => { newtype! { () struct $name($t); } };
    {pub struct $name:ident($t:ty);} => { newtype! { (pub) struct $name($t); } };

    (($($vis:tt)*) struct $name:ident($t:ty);) => {
        as_item!{
            impl $name {
                $($vis)* fn new(value: $t) -> Self {
                    $name(value)
                }
            }
        }
    };
}
// macro_rules! newtype_v2 {
//     {$(pub)? struct $name:ident($t:ty);} => { newtype_v2! { $(pub)? struct $name($t); } };
//     (($($vis:tt)*) struct $name:ident($t:ty);) => {
//         as_item!{
//             impl $name {
//                 $($vis)* fn new(value: $t) -> Self {
//                     $name(value)
//                 }
//             }
//         }
//     };
// }

macro_rules! as_item { ($i:item) => {$i} }
