#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle{
        width: 30,
        height: 50
    };
    // println!("rect1 is {}", rect1);
// this will throw an error  if we try to print directly
// hence we add a :? and derive debug on top of rectangle struct
    println!("rect1 is {rect1:?}");
    //adding :#? makes it more readable 
    println!("readle rect1 is {rect1:#?}");
    // another approach can be to use dbg! macro instead of println, dbg! takes ownership and
    // prints to stderr as opposed to stdout where println prints.
    let scale = 2;
    let rect2 = Rectangle{
        width: dbg!(30*scale), // dbg returns ownership of the expressions' value hence its like we
                               // didn't have dbg! call there, i.e. it processes then returns that
                               // item
        height: 50,
        };
    dbg!(&rect2); // we do not want dbg to take ownership of rect2 hence we use a reference to
                  // rect2 here
}
