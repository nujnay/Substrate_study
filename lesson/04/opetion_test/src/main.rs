fn main() {
    let some_u8_value = Some(3u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }
}

fn operation_2() {
    let five = Some(5);
    let six = five.map(|i| i + 1);
    print!("{:?}", six);
    let none: Option<i32> = None;
    let none_result = none.map(|i| i + 1);
    println!("{:?}", none_result);
    operation_1();
}

fn operation_1() {
    println!("Hello, world!");
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("five:{}", five.unwrap());
    println!("six:{}", six.unwrap());
    println!("none:{:?}", none);
}


enum ResultU8String {
    OK(u8),
    Err(String),
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

