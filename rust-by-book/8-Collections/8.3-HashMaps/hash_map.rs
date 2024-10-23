use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    // hashmaps store their data on heap, and used least, and hence not part of std library

    let team_name = String::from("Blue");
    let score = score.get(&team_name).copied().unwrap_or(0);
    // get returns an Option<&V>, if no vlue for key in hashmap, get will return None
    // handles Option by calling copied to get Option<i32> rather than an Option<&i32>,
    // then unwrap_or to set score to zero if scores don't have entry for the key

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    // adding key and value only if key isn't present

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");

    // updating a value based on old value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
    // The split_whitespace method returns an iterator over subslices, separated by whitespace, of the value in text. The or_insert method returns a mutable reference (&mut V) to the value for the specified key. Here, we store that mutable reference in the count variable, so in order to assign to that value, we must first dereference count using the asterisk (*). The mutable reference goes out of scope at the end of the for loop, so all of these changes are safe and allowed by the borrowing rules.

    // HashMap uses SipHash that provides resistance to DoS attacks involving hash tables.
    // This is not the fastest hash algo available, but trade-off for better security.
    // Faster but less safer hasher is available with BuildHasher trait.
}
