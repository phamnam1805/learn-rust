
fn do_something(vec: &mut Vec<i32>){
    vec.push(4);
    println!("{:?}", vec);
    println!("Inside doSomething()");
}

fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    do_something(&mut vec);
    vec.push(3);
    println!("{:?}", vec);
}
