fn main() {
    let s = String::from("hello");

    change(&s); // just like variables, references are immutable by default
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}
// Error: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
