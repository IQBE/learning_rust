enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn shoutout() {
        println!("Shoutout to Let's Get Rusty!")
    }
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn three_plus_five(a: Option<i8>) -> Option<i8> {
    match a {
        Some(3) => Some(3 + 5),
        _ => None,
    }
}

fn main() {
    let x: i8 = 5;
    let y: Option<i8> = Some(3);
    let z: Option<i8> = None;

    let sum = x + y.unwrap_or(0);
    let times = x * z.unwrap_or_default(); // Default of i8 is 0

    println!("Calculations: sum = {sum}, times = {times}");

    println!("{:?}", three_plus_five(y));
    println!("{:?}", three_plus_five(Some(6)));
    println!("{:?}", three_plus_five(None));
}
