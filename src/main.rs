// use std::io;
use rand::Rng;

const TREE:&str = "tree";

fn main() {
    println!("hello, world!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    let tree_tup: (i32, u8, char, bool) = (-999, 255, 'z', true);
    println!("tup0 = {}", tree_tup.0);
    println!("tup1 = {}", tree_tup.1);
    println!("tup2 = {}", tree_tup.2);
    println!("tup3 = {}", tree_tup.3);

    println!("secret number {}", secret_number);
    println!("secret word {}", TREE);
}



