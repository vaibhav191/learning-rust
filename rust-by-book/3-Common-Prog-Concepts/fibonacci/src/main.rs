use std::io;
fn main() {
    let mut num = String::new();

    println!("Enter the fibonacci number to get result:");
    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read input!");
    let num = match num.trim().parse::<u32>() {
        Ok(num) => num,
        Err(_) => {
            println!("Error parsing number");
            return;
        }
    };

    println!("{}", fibo(num));
}

fn fibo(num: u32) -> u32 {
    match num {
        0 => 0,
        1 => 1,
        _ => fibo(num - 1) + fibo(num - 2),
    }
}
