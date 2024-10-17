// if and let lets you handle values that match one pattern while ignoring the rest in a less
// verbose way
let config_max = Some(3u8);
match config_max {
    Some(max) => println!("The max is configured to be {max}");
    _ => (),
}
// same as
let config_max = Some(3u8);
if let Some(max) = config_max {
    println!("The max is configured to be {maxx}");
}
// if let block won't run if the value doesn't match the pattern
// less typing, less indentation and less boilerplate code.
// but we lose exhaustive checking that match enforces
// we can include else, which would be similar to _
else {
    count +=1;
}
