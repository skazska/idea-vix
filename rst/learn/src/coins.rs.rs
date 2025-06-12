use rand::prelude::*;

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

impl UsState {
    fn inc_amount(&self) -> u8 {
        match self {
            UsState::Alabama => 2,
            UsState::Alaska => 5,
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn inc_money(r#in: Option<Coin>) -> Option<u8> {
    let mut result: u8 = 0;

    let Some(coin) = r#in else {
        return None;
    };

    if let Coin::Quarter(state) = coin {
        result += state.inc_amount();
    } else {
        result += 1;
    };

    Some(result)
}

fn main() {
    let mut rng = rand::rng();
    let coin = if rng.random_bool(0.5) {
        Some(Coin::Penny)
    } else {
        Some(Coin::Quarter(UsState::Alabama))
    };

    match inc_money(coin) {
        Some(amount) => println!("You have {} cents", amount),
        None => println!("No money"),
    }
}