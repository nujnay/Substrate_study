use std::io;
//prelude
use rand::Rng;
// trait
use std::cmp::Ordering;


fn main() {
    println!("guessing");
    let secret_number = rand::thread_rng().gen_range(1, 101); // immutable i32 u32 i64
    println!("{}", secret_number);

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("can not read this line");

        //shadow
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),//arm
            Ordering::Greater => println!("to big"),
            Ordering::Equal => { break; }
        }
    }
}
