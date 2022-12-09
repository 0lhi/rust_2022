// enum IpAddrKind {
//     V4,
//     V6,
// }

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// fn main() {
//     let four = IpAddrKind::V4;
//     let six = IpAddrKind::V6;
//     route(IpAddrKind::V4);
//     route(IpAddrKind::V6);
//     let home = IpAddr::V4
// }

// fn route(ip_kind: IpAddrKind) {}

fn main() {
    // let home = IpAddr::V4(127, 0, 0, 1); //IpAddr::V4(String::from("127.0.0.1"));
    // let loopback = IpAddr::V6(String::from("::1"));
    // let absent_number: Option<i32> = None;
    // dbg!(absent_number);
    let state = UsState::Alabama;
    let firstcoin = Coin::Quarter(state);
    value_in_cents(Coin::Quarter(Alaska));
}
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("You threw in a quarter from {:?}", state);
            25
        }
    }
}
