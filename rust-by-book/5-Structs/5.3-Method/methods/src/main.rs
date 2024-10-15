#[derive(Debug)]
struct Rectangle{
    height: u32,
    width: u32,
}

impl Rectangle{
    fn area(&self) -> u32 { // &self is short for &Self, Self is an alias for the type that the
                            // impl block is for i.e. Rectangle in this case
                            // & is used to indicate that the method borrows the Self instance
        self.height*self.width
    }
    // fn can have same name as a struct's field, example - 
    fn width(&self) -> bool {
        if self.width > 0 {true} else {false}
    }
}

fn main() {
    let rect1 = Rectangle{
        height : 50,
        width : 30,
    };
    // println!("The rectangle has a width: {}", rect1.width());
    // println!("The area of the rectangle is {} square pixels", rect1.area());
    let rect2 = Rectangle{
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle{
        width: 60,
        height: 45,
    };
    let square = Rectangle::square(10);
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Creating square of size {}, {square:#?}", 10);
}

// all functions inside impl block are called 'associated functions', associated with the type
// named after impl.
// can define associated functions that don't have self as their first parameter.
impl Rectangle{
    fn can_hold(&self, other: &Rectangle) -> bool {
        // if (self.height >= other.height) && (self.width >= other.width){
        //     true
        // }
        // else {
        //     false
        // }
        self.width > other.width && self.height> other.height
    }
}

// Associted functions that aren't methods are often used for constructors that will return a new
// instance of the struct. These are often called 'new', but 'new' isn't a special name in rust.
// example -
impl Rectangle{
    fn square(size: u32) -> Self {
        Self{
            height: size,
            width: size,
        }
    }
}
