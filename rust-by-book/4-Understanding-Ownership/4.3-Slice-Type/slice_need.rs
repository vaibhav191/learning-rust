fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            i
        }
    }
    s.len()
}

fn second_word(s: &String) -> (usize, usize) {
    // we would have to return both start index and end index
}

// its difficulty to ensure that the indexes and the s they are referencing are tracked and valid.
// hence to keep these variables in sync, we can use slices
