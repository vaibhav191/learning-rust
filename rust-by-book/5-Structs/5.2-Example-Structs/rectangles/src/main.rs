// Simple Example
// fn main() {
//     let height1 = 50;
//     let width1 = 20;
//     println!("The area of the rectangle is {} square pixels", area(height1, width1));
// }
// fn area(height: u32, width: u32) -> u32 {
//     width * height
// }

// refactoring with tuples
// fn main() {
//     let rect1 = (30, 50);
//     println!("The area of the rectangle is {} square pixels", area(rect1));
// }
// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// Using structs
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle{
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {} square pixels", area(&rect1));
}
fn area(rectangle: &Rectangle) -> u32 { // borrows the function rather than taking its ownership
    rectangle.height * rectangle.width
}
