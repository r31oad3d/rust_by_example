use async_std::{fs::File, io, prelude::*, task};
use std::path::Path;
use std::time::Duration;

async fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path).await?;
    let mut content = String::new();
    file.read_to_string(&mut content).await?;
    Ok(content)
}

fn main() {
    // println!("{:?}", Path::new(".").canonicalize().unwrap());
    // let reader_task = task::spawn( async {
    //     let result = read_file("study/study_std/async/data.csv").await;
    //     match result {
    //         Ok(s) => println!("{:?}", s),
    //         Err(e) => println!("Error reading file: {:?}", e),
    //     }
    // });
    // task::block_on(reader_task)

    // let reader_task = task::spawn(read_file("study/study_std/async/data.csv"));

    // match task::block_on(reader_task) {
    //     Ok(s) => println!("{:?}", s),
    //     Err(e) => println!("Error reading file: {:?}", e),
    // }

    // let reader_task = task::spawn( async {
    //    std::fs::read_to_string("study/study_std/async/data.csv")
    // });
    //
    // let ret = task::block_on(reader_task);
    // println!("{:?}", ret);

    // task::block_on( async {
    //     panic!("test panic!");
    // });

    task::spawn(async {
        panic!("test panic!");
    });

    task::block_on(async {
        task::sleep(Duration::from_millis(10000)).await;
    });
}
