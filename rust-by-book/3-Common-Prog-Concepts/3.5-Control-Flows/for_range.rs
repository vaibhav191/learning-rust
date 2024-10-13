fn main() {
    // when using .. it includes the first number and excludes the last number
    // when using ..= it includes the last number
    for number in (1..=4).rev(){ //reverses the range
       println!("{number}!");
    }
}
