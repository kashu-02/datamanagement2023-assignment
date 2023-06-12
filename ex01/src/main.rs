extern crate rand;

use rand::Rng;

fn main() {
    println!("Rolling dice...");
    let mut rng = rand::thread_rng();
    let aa: i32 = rng.gen_range(1..6);
    let bb: i32 = rng.gen_range(1..6);
    println!("Die 1: {}", aa);
    println!("Die 2: {}", bb);
    let total: i32 = aa + bb;
    println!("Total value: {}", total);
    if total > 7 {
        println!("You won!");
    } else {
        println!("You lost!");
    }
}