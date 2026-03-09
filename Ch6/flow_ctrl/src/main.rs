enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Aaa,
    Bbb,
    Ccc,
}

impl Coin {
    fn value_in_cents(&self) -> u8 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("This quarter is from {:?}!", state);
                25
            }
        }
    }
}

fn main() {
    let penny = Coin::Penny;
    println!("Penny is {} cents", penny.value_in_cents());

    let nickel = Coin::Nickel;
    println!("Nickel is {} cents", nickel.value_in_cents());

    let dime = Coin::Dime;
    println!("Dime is {} cents", dime.value_in_cents());

    let quarter = Coin::Quarter(UsState::Aaa);
    println!("Quarter is {} cents", quarter.value_in_cents());

    let a = Some(5);
    let b: Option<i32> = None;

    plus_one(a);
    plus_one(b);

    match a {
        None => println!("None"),
        Some(i) => println!("Value a is {i}"),
    }
    match b {
        None => println!("None"),
        Some(i) => println!("Value b is {i}"),
    }

    if let Some(i) = a {
        println!("Value a is {i} from if let");
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
