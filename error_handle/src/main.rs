#![allow(dead_code, unused_imports, deprecated)]
use std::convert::From;
use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::io::{self, Read};
use std::num;
use std::panic;
use std::path::Path;

fn way_1() {
    let result = panic::catch_unwind(|| {
        println!("hello!");
    });
    assert!(result.is_ok());

    let result = panic::catch_unwind(|| {
        panic!("oh no!");
    });
    assert!(result.is_err());
}

fn way_2() -> () {
    fn file_double<P: AsRef<Path>>(file_path: P) -> Result<i32, String> {
        let mut file = match File::open(file_path) {
            Ok(file) => file,
            Err(err) => return Err(err.to_string()),
        };
        let mut contents = String::new();
        if let Err(err) = file.read_to_string(&mut contents) {
            return Err(err.to_string());
        }
        let n: i32 = match contents.trim().parse() {
            Ok(n) => n,
            Err(err) => return Err(err.to_string()),
        };
        Ok(2 * n)
    }

    match file_double("/Users/johnj/Development/code/rust/rust_by_example/error_handle/testfile") {
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error: {}", err),
    }
}

fn way_3() -> () {
    fn file_double<P: AsRef<Path>>(file_path: P) -> i32 {
        let mut file = File::open(file_path).unwrap();
        let ref mut contents = String::new();
        //let mut contents = String::new().as_ref();
        file.read_to_string(contents).unwrap();
        let n: i32 = contents.trim().parse().unwrap();
        2 * n
    }

    let double = file_double("foobar");
    println!("{}", double)
}

fn way_4() {
    //and
    let x: Result<u32, &str> = Ok(2);
    let y: Result<&str, &str> = Err("later error");
    assert_eq!(x.and(y), Err("later error"));

    fn sq(x: u32) -> Result<u32, u32> {
        Ok(x * x)
    }
    fn err(x: u32) -> Result<u32, u32> {
        Err(x)
    }
    assert_eq!(Ok(2).and_then(sq).and_then(sq), Ok(16));
    assert_eq!(Ok(2).and_then(sq).and_then(err), Err(4));
    assert_eq!(Ok(2).and_then(err).and_then(sq), Err(2));
    assert_eq!(Err(3).and_then(sq).and_then(sq), Err(3));

    assert_eq!(Ok(2).or_else(sq).or_else(sq), Ok(2));
    assert_eq!(Ok(2).or_else(err).or_else(err), Ok(2));
    assert_eq!(Err(3).or_else(sq).or_else(err), Ok(9));
    assert_eq!(Err(3).or_else(err).or_else(err), Err(3));
}

fn way_5() {
    fn file_double<P: AsRef<Path>>(file_path: P) -> Result<i32, String> {
        File::open(file_path)
            .map_err(|err| err.to_string())
            .and_then(|mut file| {
                let mut contents = String::new();
                file.read_to_string(&mut contents)
                    .map_err(|err| err.to_string())
                    .map(|_| contents)
            })
            .and_then(|contents| {
                contents
                    .trim()
                    .parse::<i32>()
                    .map_err(|err| err.to_string())
            })
            .map(|n| 2 * n)
    }
    match file_double("foobar") {
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error: {}", err),
    }
}

fn way_6() {
    fn file_double<P: AsRef<Path>>(
        file_path: P,
    ) -> Result<i32, Box<dyn Error>> {
        File::open(file_path)
            .map_err(|err| From::from(err))
            .and_then(|mut file| {
                let mut contents = String::new();
                file.read_to_string(&mut contents)
                    .map_err(|err| From::from(err))
                    .map(|_| contents)
            })
            .and_then(|contents| {
                contents
                    .trim()
                    .parse::<i32>()
                    .map_err(|err| From::from(err))
            })
            .map(|n| 2 * n)
    }
    match file_double("foobar") {
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error: {}", err),
    }
}

fn way_7() {
    fn file_double<P: AsRef<Path>>(
        file_path: P,
    ) -> Result<i32, Box<dyn Error>> {
        let mut file = r#try!(File::open(file_path));
        let mut contents = String::new();
        r#try!(file.read_to_string(&mut contents));
        let n = r#try!(contents.trim().parse::<i32>());
        Ok(2 * n)
    }
}

fn way_8() {
    enum CliError {
        Io(io::Error),
        Parse(num::ParseIntError),
    }

    impl From<io::Error> for CliError {
        fn from(err: io::Error) -> CliError {
            CliError::Io(err)
        }
    }

    impl From<num::ParseIntError> for CliError {
        fn from(err: num::ParseIntError) -> CliError {
            CliError::Parse(err)
        }
    }

    fn file_double<P: AsRef<Path>>(file_path: P) -> Result<i32, CliError> {
        let mut file = r#try!(File::open(file_path));
        let mut contents = String::new();
        r#try!(file.read_to_string(&mut contents));
        let n = r#try!(contents.trim().parse::<i32>());
        Ok(2 * n)
    }
}
fn way_9() {
    enum CliError {
        Io(io::Error),
        Parse(num::ParseIntError),
    }

    impl From<io::Error> for CliError {
        fn from(err: io::Error) -> CliError {
            CliError::Io(err)
        }
    }

    impl From<num::ParseIntError> for CliError {
        fn from(err: num::ParseIntError) -> CliError {
            CliError::Parse(err)
        }
    }
    fn file_double<P: AsRef<Path>>(file_path: P) -> Result<i32, CliError> {
        let mut contents = String::new();
        File::open(file_path)?.read_to_string(&mut contents)?;
        let n: i32 = contents.trim().parse()?;
        Ok(2 * n)
    }
}

fn main() {
    //way_1();
    //way_2();
    //way_3();
    //way_4();
    //way_5();
    //way_6();
    //way_7();
    //way_8();
    way_9();
}
