fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
}
// Error: cannot borrow `s` as mutable more than once at a time
// prevents data race conditions
// data race happens when three behaviors occur:
// 1. Two or more pointers access the same data at the same time.
// 2. At least one of the pointers is being used to write to the data.
// 3. There’s no mechanism being used to synchronize access to the data.
