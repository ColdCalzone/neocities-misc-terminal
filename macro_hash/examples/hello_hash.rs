extern crate hash;
use hash::hash;

fn main() {
    let hash = hash!("World!");
    println!("Hello, {}", hash);
}
