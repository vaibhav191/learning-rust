fn main() {
    println_labeled_measurement(5, 'h');
}

fn println_labeled_measurement(x: i32, unit: char) {
    println!("The measurement is {x}{unit}");
}
