extern crate rand;

use rand::Rng;

fn main() {
    println!("What is your name?");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).unwrap();
    println!("Hello, {}!", name.trim());

    println!("Rolling dice...");
    let mut rng = rand::thread_rng();
    let aa: i32 = rng.gen_range(1..6);
    let bb: i32 = rng.gen_range(1..6);
    println!("Die 1: {}", aa);
    println!("Die 2: {}", bb);
    let total: i32 = aa + bb;
    println!("Total value: {}", total);
    if total > 7 {
        println!("{} won!", name.trim());
    } else {
        println!("{} lost!", name.trim());
    }
}