struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let email = String::from("email@domain.com");
    let username = String::from("username123");
    let user1: User = build_user(email, username);
    let user2: User = User {
        email: String::from("email2@domain.com"), // if we only give String value for suppose email
                                                  // and we inherit the username from user1 then
                                                  // user1 will no longer be usable as a whole
                                                  // since the String in the username will be moved
                                                  // into user2. Hence we have given both username
                                                  // and email to user2. The remaining values
                                                  // active and sign_in_count implement a Copy
                                                  // trait and hence are still valid.
        username: String::from("username2"),
        ..user1
    };

    println!("username: {}, email: {}", user1.username, user1.email);
    println!("username: {}, email: {}", user2.username, user2.email);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
