/* Example 1 - Ip Addresses */
struct Ipv4Address {
    address: String,
}

struct Ipv6Address {
    address: String,
}

enum IpAddress {
    V4(Ipv4Address),
    V6(Ipv6Address),
}

/* Example 2 - Message */
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
        println!("Message is called");
    }
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_of_coin(coin: Coin) -> i32 {
    /*
    Match is similar to if, but there’s a big difference.
    With if, the expression needs to return a Boolean value, but here, it can return any type.
    */
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("The state is {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => {
            println!("{:?}", i);
            Some(i + 1)
        }
    }
}
fn main() {
    println!("Hello, world!");
    let v4 = IpAddress::V4;
    let v6 = IpAddress::V6;

    let message = Message::Write(String::from("hello"));
    message.call();

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
