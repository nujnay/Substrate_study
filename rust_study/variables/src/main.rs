
fn main() {
    let one = "3333";
    multiple1(one);
    println!("{}",one.to_string());
}
fn multiple1(&one: String) {
    one = one + 1;
}
