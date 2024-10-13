use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..100);

    loop {
        println!("Please input your guess.");
        // variables are immutable by default in Rust
        // to make them mutable, use the mut keyword
        let mut guess = String::new(); // ::new() is an associated function of the String type, an
                                       // associated function is a function that is associated with a
                                       // type
        io::stdin() // if we had not imported io, we would have to write std::io::stdin(), stdin is a
            // function that returns an instance of std::io::Stdin
            .read_line(&mut guess) // & indicates that this argument is a reference, which gives you a
            // way to let multiple parts of your code access one piece of data
            // without needing to copy that data into memory multiple times
            // references are immutable by default, hence we need to write &mut
            // guess instead of &guess to make it mutable
            .expect("Failed to read line"); // read_line puts the user's value into the mutable string
                                            // passed but also returns a value, in this case, an
                                            // io::Result, which is an enumeration, which is a type
                                            // that can have multiple values, called variants
                                            // Result types encode error-handling information
                                            // two variants: Ok and Err
                                            // Err variant contains information about what kind of
                                            // error occurred
                                            // if instance of io::Result is an Err value, expect will
                                            // cause the program to crash and display the message
                                            // passed as an argument to expect
                                            // if dont use expect, program compiles but you get warning
                                            // let guess: u32 = guess.trim().parse::<u32>().expect("Please type a number!"); //shadowing previous
                                            //value of guess with a
                                            //new one.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
