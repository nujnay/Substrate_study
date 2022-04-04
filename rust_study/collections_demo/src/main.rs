use std::any::Any;
use std::collections::HashMap;
use std::fmt::format;

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
    let vector_demo = VectorDemo1 {
        width: 30
    };
    let v3 = vec![vector_demo];
    println!("Hello, world!{:#?}", v3[0]);

    let mut s = String::new();
    let s1 = "initial contents".to_string();
    let mut s2 = String::from("A");
    s2.push_str(&s1);
    s2.push('1');
    println!("{}", s2);

    let s4 = String::from("s4");
    let s5 = String::from("s4");
    let s6 = s4 + &s5;

    // println!("s4 {}", s4);
    // println!("s5 {}", s5);
    // println!("s6 {}", s6);

    let s7 = String::from("s7");
    let s8 = String::from("s8");
    let s9 = String::from("s9");
    let s10 = format!("{}-{}-{}", s7, s8, s9);

    let h = s10.len();

    let s11 = String::from("闫军");
    let s13 = s11.chars().nth(0).unwrap();
    println!("-==={}", s13);
    let s12 = &s11[0..3];

    for a in s11.bytes() {
        println!("---{}", a)
    }
    let mut scores = HashMap::new();

    let blue = String::from("Blue");
    let yellow = String::from("yellow");
    scores.insert(blue, 10);
    scores.insert(yellow, 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    match score {
        Some(s) => println!("{}", s),
        None => println!("none")
    }
    scores.entry(String::from("purple")).or_insert(0);

    for (k, v) in &scores {
        println!("{},{}", k, v);
    }

    println!("{:?}", &scores);

    let text = "hello I am YanJun hello";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    };
    println!("{:?}", map);

    let teams = vec![String::from("blue"), String::from("yellow")];
    let initail_scoer = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initail_scoer.iter()).collect();
}

#[derive(Debug)]
struct VectorDemo1 {
    width: i32,

}
