use std::net::{Ipv4Addr, Ipv6Addr};

fn main() {
    // Enums and Pattern Matching
    // Defining an Enum
    // Enum Values
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let _home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let _loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let _home2 = IpAddr2::V4(String::from("127.0.0.1"));
    let _loopback2 = IpAddr2::V6(String::from("127.0.0.1"));

    let _home3 = IpAddr3::V4(127, 0, 0, 1);
    let _loopback = IpAddr3::V6(String::from("::1"));

    let _std_home = Ipv4Addr::new(127, 0, 0, 1);
    let _std_loopback = Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0xc00a, 0x2ff);

    let m = Message::Write(String::from("hello"));
    m.call();

    // The Option Enum and It's Advantages Over Null Values
    // Option
    /*
    enum Option<T> {
        Some<T>,
        None,
    }
    */

    let _some_number = Some(5);
    let _some_string = Some("a string");
    // If you use the rather than some , we need to tell the Rust what type of Option<T> we have
    // because the compiler can't infer the type that the some variant will hold by at a none value.
    let _absent_number: Option<i32> = None;

    // let x: i8 = 5;
    // let y: Option<i8> = some(5);
    // TODO:
    // let sum = x + y;

    // The match Control Flow Operator
    // Patterns that Bind to Values

    // Matching with Option<T>
    let five = Some(5);
    // matching Some(T)
    let _six = plus_one(five);
    let _none = plus_one(None);
    // Matches Are Exhaustive
    // The _ Placeholder
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }

    //Concise Control Flow with if let
    let some_u8_value2 = Some(0u8);
    match some_u8_value2 {
        Some(3) => println!("three"),
        _ => (),
    }
    // The following code behaves the same as the match
    if let Some(3) = some_u8_value2 {
        println!("three");
    }

    let mut count = 0;
    let coin = Coin2::Quarter(UsState::Alabama);
    match coin {
        Coin2::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
    // The following code behaves the same as the match
    let mut count2 = 0;
    let coin2 = Coin2::Quarter(UsState::Alabama);
    if let Coin2::Quarter(state) = coin2 {
        println!("State quarter from {:?}", state);
    } else {
        count2 += 1;
    }
}

// Defining an Enum
enum IpAddrKind {
    V4,
    V6,
}

fn route(ip_type: IpAddrKind) {}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddr2 {
    V4(String),
    V6(String),
}

enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("{:#?}\n", self);
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        // Coin::Penny => 1,
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin2 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents2(coin: Coin2) -> u8 {
    match coin {
        Coin2::Penny => 1,
        Coin2::Nickel => 5,
        Coin2::Dime => 10,
        Coin2::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn plus_one2(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        // None => Some(0),
        _ => None,
    }
}
