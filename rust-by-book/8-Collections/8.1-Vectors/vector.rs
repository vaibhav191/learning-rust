fn main() {
    let v: Vec<i32> = Vec::new();
    // Vec<T> signifies inferred type for the vector.
    // vec! macrocreates a new vector that holds the values you give it.
    let v = vec![1, 2, 3];
    // updating - push
    let mut v = Vec::new();
    v,push(5);
    v.push(6);

    //reading
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {third}");
    let third: Option<&i32> = v.get(2);
    match third{
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // for loop and immutable reference
    let v = vec![100, 32, 57];
    let i in &v {
        println!{"{i}"};
    }
    // mutable elements iteration
    let mut v = vec![100, 32, 57];
    for i in &mut v{
        *i += 50;
    }


    // storing different types in vector
    enum Spreadsheetcell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    // using an enum and match ensures that rust will at compile time that every possible case is
    // handled
    // if we don't know the exhaustive set of types a program will get at runtime, enum technique
    // won't work. instead Trail object would be needed.
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
