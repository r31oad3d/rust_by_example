#![allow(unused_imports)]
use std::fs::File;
use std::io::Result as IOResult;
use std::io::{
    self, BufRead, BufReader, BufWriter, Read, Seek, SeekFrom, Write,
};

fn main() -> IOResult<()> {
    let mut f = File::open("./io-example/file1")?;
    let mut buffer = [0; 20];
    let n1 = f.read(&mut buffer)?;
    println!("n: {:?}", n1);
    println!("the bytes: {:?}", &buffer[..n1]);

    buffer = [0; 20];
    f.seek(SeekFrom::Start(10))?;
    let n2 = f.read(&mut buffer)?;
    println!("n: {:?}", n2);
    println!("the bytes: {:?}", &buffer[..n2]);

    let f = File::open("./io-example/file1")?;
    let mut reader = BufReader::new(f);
    let mut buffer1 = String::new();
    while let Ok(ret) = reader.read_line(&mut buffer1) {
        if ret == 0 {
            break;
        } else {
            print!("{}", buffer1);
            buffer1.clear()
        }
    }

    let f = File::open("./io-example/file1")?;
    {
        let mut writer = BufWriter::new(f);
        writer.write(&[97])?;
    }

    // let mut input = String::new();
    // io::stdin().read_line(&mut input)?;
    // println!("you typed: {}", input.trim());

    //io::stdout().write(&[97])?;

    let f = File::open("./io-example/file1")?;
    let reader = BufReader::new(f);

    for line in reader.lines() {
        println!("{}", line?);
    }

    io::copy(&mut io::stdin(), &mut io::stdout())?;

    Ok(())
}
