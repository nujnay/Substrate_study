fn main() {
    println!("Hello, world!");
    let home = IpAddr {
        kind: IpAddress::V4(127, 0, 0, 1),
        address: String::from("127.0.0.1"),
    };

    let q = Message::Quit;
    let m = Message::Move { x: 32, y: 24 };
    let w = Message::Write(String::from("Hello"));
    let c = Message::ChangeColor(0, 255, 255);

    let some_numer = Some(5);
    let some_string = Some("A String");

    let absent_number: Option<i32> = None;


    let pennt = Coin::Quarter(UsState::Alabama);
    println!("{}", value_in_cents(pennt));

    let five = Some(5);
}


#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            print!("State quater from {:?}!", state);
            25
        }
    }
}

enum IpAddress {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct IpAddr {
    kind: IpAddress,
    address: String,
}


enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),

}

impl Message {
    fn call(&self) {}
}