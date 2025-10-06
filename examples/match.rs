#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let alaskan_cents = value_in_cents(Coin::Quarter(UsState::Alaska));
    let penny_cents = value_in_cents(Coin::Penny);

    println!("alaskan_cents = {alaskan_cents}");
    println!("penny_cents = {penny_cents}");
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}
