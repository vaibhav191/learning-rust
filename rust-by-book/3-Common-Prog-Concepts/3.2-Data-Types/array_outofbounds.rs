use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];
    println!("Enter an array index:");
    let mut index = String::new(); // declaring 
    io::stdin().read_line(&mut index).expect("Failed to read line"); // reading
    let index: usize = index.trim().parse::<usize>().expect("Index must be a number"); // parsing
    let element = a[index]; // accessing
    println!("The value of the element at index {index} is: {element}");
}
