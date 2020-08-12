use std::fmt;
use std::fmt::{Error, Formatter};

#[derive(Default)]
pub struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let List(ref vec) = *self;
        //let List(vec)  = &*self;
        write!(f, "[")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ",")?;
            }
            write!(f, "{}", v)?;
        }
        write!(f, "]")
    }
}

pub fn test2() {
    let v = List(vec![1, 2, 3, 4]);
    println!("{}", v);
}
