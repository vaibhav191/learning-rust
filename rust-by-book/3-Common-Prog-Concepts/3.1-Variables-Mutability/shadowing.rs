// ability to declare a new variable with the same name as the previous variable
// handy for converting values from one type to another
// can shadow a variable by using the same variable's name and repeating the use of the 'let
// keyword'

fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x in the outer scope is: {}", x);
}

// The output of the above code is:
// The value of x in the inner scope is: 12
// The value of x in the outer scope is: 6
