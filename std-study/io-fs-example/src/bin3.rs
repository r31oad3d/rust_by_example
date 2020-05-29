use std::error::Error;
use std::io::prelude::*;
use std::process::{Command, Stdio};

static PANGRAM: &'static str = "a\n";

fn main() {
    let process = match Command::new("wc")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        Err(why) => panic!("could not spawn wc: {}", why.to_string()),
        Ok(process) => process,
    };

    match process.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
        Err(why) => panic!("couldn't write to wc stdin: {}", why.to_string()),
        Ok(_) => println!("sent pangram to wc"),
    }

    let mut s = String::new();
    match process.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => panic!("couldn't read wc stdout: {}", why.to_string()),
        Ok(_) => print!("wc responded with:\n{}", s),
    }

    ()
}
