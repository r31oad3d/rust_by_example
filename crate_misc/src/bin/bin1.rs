extern crate base64;
extern crate image;

use std::fs;
use std::path::Path;

fn main() {
    let path = Path::new(".");
    println!("{:?}", path.canonicalize().unwrap());
    match fs::read("crate_misc/res/6027044.jpg") {
        Ok(img_buf) => {
            let bytes_written = base64::encode(img_buf);
            println!("{:?}", bytes_written);
        }
        Err(e) => {
            println!("{:?}", e.to_string());
        }
    }
}
