fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // we can also use return to return a value and exit a function
                               // if we use return here, it will exit the main function and print
                               // wont be accessible hence it will throw an error
        }
    };
    println!("The result is {result}");
}
