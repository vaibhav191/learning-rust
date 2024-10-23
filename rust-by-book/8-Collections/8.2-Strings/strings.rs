fn main() {
    // many operations of Vec<T> would also be available with String, because String is actually
    // implemented as a wrapper around vector of bytes.
    let mut s = String::new();
    // to_display method is available with any mehtod that wants to implement Display trait
    // and hence available with string literals
    let data = "initial contents";
    let s = data.to_string();
    // or
    let s = "initial contents".to_string();
    // or
    let s = String::from("initial contents");
    // since they are utf-8 encoded, we can include any properly encoded data in them.
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");

    // updating string
    // conveniently use the + operator or the format! macro to concatenate String values
    let mut s = String::from("foo");
    s.push_str("bar");
    // push_str takes a string slice since we don't want to take ownership of the parameter.
    // we might want to use s2 after appending its contents to s1
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    // push takes single char as a parameter and adds it ot the string
    let mut s = String::from("lo");
    s.push('l');

    //combine two strings
    let s1 = String::from("Hello ,");
    let s2 = String::from("World!");
    let s3 = s1 + &s2; // s1 has been moved here and can no longer be used;

    // + operator uses add method whose signature looks like
    // fn add(self, s: &str) -> String {

    // adding more
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    // or you can use format! macro
    let s = format!("{s1}-{s2}-{s3}");

    // cannot access string characters using indexing
    // String is a wrapper over a Vec<u8>
    let hello = String::from("Hola"); // 4 byte long

    // size
    let hello = String::from("Здравствуйте"); // 24 byte long, since each character takes 2
                                              // byte of space instead
    let hello = "Здравствуйте";
    let answer = &hello[0];
    // You already know that answer will not be З, the first letter. When encoded in UTF-8, the first byte of З is 208 and the second is 151, so it would seem that answer should in fact be 208, but 208 is not a valid character on its own. Returning 208 is likely not what a user would want if they asked for the first letter of this string; however, that’s the only data that Rust has at byte index 0. Users generally don’t want the byte value returned, even if the string contains only Latin letters: if &"hello"[0] were valid code that returned the byte value, it would return 104, not h.

The answer, then, is that to avoid returning an unexpected value and causing bugs that might not be discovered immediately, Rust doesn’t compile this code at all and prevents misunderstandings early in the development process.
    
    // A final reason Rust doesn’t allow us to index into a String to get a character is that indexing operations are expected to always take constant time (O(1)). But it isn’t possible to guarantee that performance with a String, because Rust would have to walk through the contents from the beginning to the index to determine how many valid characters there were.
    
    // SLICING STRINGS
    let hello = "Здравствуйте";

    let s = &hello[0..4]; // will output Зд
                          // of type &str

    // METHODS FOR ITERATING OVER STRINGS
    for c in "Зд".chars() {
        println!("{c}");
    }
    // З
    // д


    for b in "Зд".bytes() {
        println!("{b}");
    }
    // output - 
    // 208
    // 151
    // 208
    // 180

    // note - unicode scalar values may be made of more than 1 byte!
    // Getting grapheme clusters from strings, as with the Devanagari script, is complex, so this functionality is not provided by the standard library. Crates are available on crates.io if this is the functionality you need.
}
