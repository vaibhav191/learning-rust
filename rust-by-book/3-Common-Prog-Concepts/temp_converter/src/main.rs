use std::io;
fn main() {
    let mut choice = String::new();
    println!("1 - Celcius to Farenheit");
    println!("2 - Farenheit to Celcius");
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line!");
    let choice: i32 = match choice.trim().parse::<i32>() {
        Ok(num) => num,
        Err(_) => {
            println!("Error parsing the number!");
            return;
        }
    };

    let acceptable_choices = [1, 2];
    if !acceptable_choices.contains(&choice) {
        println!("Invalid choice!");
        return;
    }

    let mut temp = String::new();
    println!("Enter temperature:");
    io::stdin()
        .read_line(&mut temp)
        .expect("Please entire a number");
    let temp: f32 = match temp.trim().parse::<f32>() {
        Ok(num) => num,
        Err(_) => {
            println!("Error parsing the number!");
            return;
        }
    };

    let temp_conv = match choice {
        1 => to_fahren(temp),
        2 => to_celcius(temp),
        _ => unreachable!(),
    };

    println!("temp: {temp_conv} {}", if choice == 1 { 'f' } else { 'c' });
}

fn to_fahren(x: f32) -> f32 {
    (x * 9.0 / 5.0) + 32.0
}

fn to_celcius(x: f32) -> f32 {
    (x - 32.0) * 5.0 / 9.0
}
