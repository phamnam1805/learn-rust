#![allow(unused)]

fn main() {
    let mut s: String = String::from("Hello");
    s.push_str(", World!");
    println!("{}", s);

    // let mut a: &str = "Hi";
    // println!("{}", a);
    // a = "Hello";
    // println!("{}", a);

    let mut s1: String = String::from("Hello ");
    let mut s2 = &mut s1;
    s2.push_str("string");
    println!("{:p}", s2);
    s1.push_str("vkl");
    println!("{:p}", &s1);

    // s1.push_str("string");    
    // println!("{}", s1);
    // s2.push_str("How");
    // println!("{}", s2);
}
