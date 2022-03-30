fn main() {
    let mut s1 = String::from("ddd");
    borrow1(&mut s1);
    s1.push_str("world");
    println!("{}", s1);
}

fn borrow1(s: &mut String) {
    s.push_str("world");
}