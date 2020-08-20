#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => { // moved `state` ownership
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn main() {
    let v1 = value_in_cents(Coin::Nickel);
    let c = Coin::Quarter(UsState::Alabama);
    let v2 = value_in_cents(c);
    // println!("The value of c is: {:#?}", c);
    let v3 = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("The value of v is: {}", v1);
    println!("The value of v is: {}", v3);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // The _ Placeholder
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),  // The `()` is just the unit value, so nothing will happen in the `_` case.
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
