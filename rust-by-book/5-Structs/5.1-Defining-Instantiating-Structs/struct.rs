struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername"),
        email: String::from("someemail@email.com"),
        sign_in_count: 1,
    };
    println!("Email of the user1 is: {}", user1.email);
}
