fn main() {
    let mut counter = 0;
    let result = loop { // this is an expression, hence can be placed after let to be evaluated and
                        // assigned
        counter += 1;
        if counter == 10{
            break counter*2 // adding ; or not is not making any difference here.
        }
    }; 
    println!("The result is {result}");
}
