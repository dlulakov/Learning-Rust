#[derive(Debug)]
enum State {


    Alabama,
    Alaska,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State),
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        },
    }
}
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
       None => None,
       Some(i) => Some(i + 1),
        
    }


}



fn main() {
    println!("Hello, world!");
    value_in_cents(Coin::Quarter(State::Alabama));
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    let dice_roll = 9;
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}
