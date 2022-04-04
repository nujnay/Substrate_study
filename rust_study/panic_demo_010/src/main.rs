use std::fs::File;
use std::io;
use std::io::{ErrorKind, Read};


fn main() {
    //  panic!("eeeeeee");
    // let v = vec![1, 2, 3];
    // v[99];
    let result = read_username_from_file();

    // let f = File::open("hello.txt");
    // match f {
    //     Ok(file) => file,
    //     Err(err) => {
    //         match err.kind() {
    //             ErrorKind::NotFound => match File::create("hello.txt") {
    //                 Ok(fc) => fc,
    //                 Err(e) => panic!("Error file :{:?}", e),
    //             },
    //             oe => { panic!("error opening the file: {:?}", oe) }
    //         }
    //     }
    // };
    // let f = File::open("hello_2.txt").unwrap();
    // let f = File::open("hello_3.txt").expect("eorre");


}
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello4.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello4.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
