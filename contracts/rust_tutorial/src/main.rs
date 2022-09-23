use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    let mut vector: Vec<i32> = vec![1,2,3,4,5];
    for i in &mut vector {
        *i = 2;
    }

    for i in &vector {
        println!("{}",i);
    }

}

// fn main() {
//     let age: i32 = 1;
//     match age {
//         1_i32..=18_i32 => println!("Important Birthday"),
//         21 | 50 => println!("Hihi"),
//         65..=i32::MAX => println!("Huhu"),
//         _ => println!("Nothing")
//     }
// }

// fn main() {
//     println!("What is your name?");
//     let mut name: String = String::from("hi");
//     let greeting: &str = "Nice to meet you";
//     // io::stdin()
//     //     .read_line(&mut name)
//     //     .expect("Didn't Receive Input");
//     // println!("Hello {}. {}",name.trim_end().trim_start(), greeting);
//     println!("{:p}", &name);
//     println!("{:p}", greeting);
// }
