//#[derive(Debug)]
//enum IpAddrKind{
//    V4(String),
//    V6(String),
//}

//enum Message{
//    Quit,
//    Move {x: i32, y: i32},
//    Write(String),
//    ChangeColor(i32, i32, i32),
//}

//impl Message {
//    fn call(&self) {
//    }
//}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin{
    Penny,
    Nickle,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin{
    Coin::Penny => {
        println!("lucky penny!");
        1},
    Coin::Nickle => 5,
    Coin::Dime => 10,
    Coin::Quarter(state) => {
        println!("State quarter from {state:?}!");
        25
    },
    }
}

fn main() {
    let quarter = Coin::Quarter(UsState::Alabama);
    println!("{}", value_in_cents(quarter));
    
    let penny = Coin::Penny;
    println!("value of this coin is: {}", value_in_cents(penny));
}

//fn ip_example(){
//    let four = IpAddrKind::V4(String::from("192.168.68.1"));
//    let six = IpAddrKind::V6(String::from("192.168.68.2")); 
//
//    println!("{:#?}", four);
//    println!("{:#?}", six);
//}
