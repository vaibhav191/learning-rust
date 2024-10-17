use crate::garden::vegetables::Asparagus;

pub mod garden; // Tells compiler to include the code it finds in src/garden.rs

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");
}
