#[derive(Debug)] // so we can inspect state
enum UsState {
    Alaska,
    Alabama,
    Arizona,
    //
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin:Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            
        }
    }
}
// example - value_in_cents(Coin::Quarter(UsState::Alaska))
