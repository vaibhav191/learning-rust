const NUMBERS: [&str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
    "eleventh", "twelfth",
];
const ITEMS: [&str; 12] = [
    "a partridge in a pear tree",
    "two turtile doves",
    "three french hens",
    "four calling bords",
    "five gold rings",
    "six geese a-laying",
    "seven swans a-swimming",
    "eight maids a-milking",
    "nine ladies dancing",
    "ten lords a-leaping",
    "eleven pipers piping",
    "twelve drummers drumming",
];

const TOTAL_DAYS: usize = 12;

fn main() {
    for i in 0..TOTAL_DAYS {
        sing_chorus(i);
        for j in (0..=i).rev() {
            println!("{}{}", if j == 0 { "and " } else { "" }, ITEMS[j]);
        }
        println!()
    }
}

fn sing_chorus(x: usize) {
    println!(
        "On the {} day of Christmas, my true love sent to me",
        NUMBERS[x]
    );
}
