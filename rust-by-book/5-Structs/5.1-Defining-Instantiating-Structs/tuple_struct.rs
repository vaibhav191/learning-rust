// struct keyword and the struct name followed by the types in the tuple.
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0 , 0);
    // can be accessed using . or destructure
}
