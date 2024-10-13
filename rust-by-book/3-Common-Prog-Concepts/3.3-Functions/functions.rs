fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}

// rust follows a camel case convention
// rust doesn't care where the function is defined, it needs to be defined somewhere in scope that
// can be seen by the caller
