use std::fmt::{Display, Error, Formatter};

/// help
///
///
///
///
///
///
///
///
/// asdasd
///
///
///
///
/// asdasd
///
#[allow(dead_code)]
fn test_comments() {
    // comment 1
    /* */
    //! doc doc doc
}

#[derive(Debug)]
pub struct Structure(i32);

#[derive(Debug)]
pub struct Deep(Structure);

#[allow(dead_code)]
pub struct UnPrintable(i32);

#[derive(Debug)]
pub struct DebugPrintable(i32);

impl UnPrintable {
    #[allow(dead_code)]
    pub fn new(v: i32) -> UnPrintable {
        UnPrintable(v)
    }
}

impl DebugPrintable {
    pub fn new(v: i32) -> DebugPrintable {
        DebugPrintable(v)
    }
}

impl Display for DebugPrintable {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.0)
    }
}

pub struct MinMax(i64, i64);

impl MinMax {
    #[allow(dead_code)]
    pub fn new(min: i64, max: i64) -> MinMax {
        MinMax(min, max)
    }
}

impl Display for MinMax {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "({},{})", self.0, self.1)
    }
}

pub struct Point2D {
    x: f64,
    y: f64,
}

impl Point2D {
    pub fn new(x: f64, y: f64) -> Point2D {
        Point2D { x, y }
    }
}

impl Display for Point2D {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "x:{}, y:{}", self.x, self.y)
    }
}
