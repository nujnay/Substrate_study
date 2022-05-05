#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,

}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.length
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.length > other.length
    }
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            length: size,
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        length: 30,
    };
    let rect2 = Rectangle {
        width: 10,
        length: 10,
    };
    println!("{:#?}", rect.area());
    println!("{:#?}", rect.can_hold(&rect2));
}
