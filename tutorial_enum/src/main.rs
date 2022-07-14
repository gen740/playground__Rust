#[derive(Debug)]
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

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            return 1;
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

pub fn main() {
    let penny = Coin::Penny;
    let quarter = Coin::Quarter(UsState::Alaska);
    println!("The value of penny is {}", value_in_cents(penny));
    println!("The value of penny is {}", value_in_cents(quarter));
}
