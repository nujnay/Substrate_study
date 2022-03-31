fn main() {
    let mut v: Vec<i32> = Vec::new();
    &v.push(1);
    &v.push(2);
    &v.push(3);
    let third: &i32 = &v[2];
    let first = &v[0];

    let mut v2 = vec![1, 2, 3];
    v2.push(2);


    match v.get(100) {
        Some(third) => println!("{}", third),
        None => print!("no third element"),
    }

    for i in &mut v2 {
        *i += 50;
    }

    for i in v2 {
        println!("{}", i)
    }

    println!("Hello, world!");
}
