// structs that don't have any fields are called unit-like structs because they behave similar to
// ()
// Useful whenyou need to implement a trait on some type but dont have any data that you want to
// store in the type itself.

struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
