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

fn main() {
    let penny = Coin::Penny;

    let quarter = Coin::Quarter(UsState::Alabama);
    println!("value in cents {0}", value_in_cents(&penny));

    value_in_cents(&quarter);
}

fn value_in_cents(coin: &Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("state of the quarter is {state:?}");
            25
        }
    }
}
