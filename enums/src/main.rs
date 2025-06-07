#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool{
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u8 {
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

fn describe_state_quarter(coin: &Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}


fn main() {

    let q = Coin::Quarter(UsState::Alabama);
    println!("Quarter value: {}", value_in_cents(&q));
    
    if let Some(text) = describe_state_quarter(&q) {
        println!("{}", text);
    }

    if let Some(text) = describe_state_quarter(&Coin::Quarter(UsState::Alaska)) {
        println!("{}", text);
    }

    let coin = Coin::Dime;
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else if let Coin::Dime = coin {
        count += 2;
    } else {
        count += 1;
    }
    println!("{}", count);

    println!("Nickel value: {}", value_in_cents(&Coin::Nickel));
    println!("Penny value: {}", value_in_cents(&Coin::Penny));


}
