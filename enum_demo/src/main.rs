use std::net::{Ipv4Addr, Ipv6Addr};

fn main() {
    let four = IpAddKind::V4;

    let home = IpAddr {
        kind: IpAddKind::V4,
        address: String::from("::1"),
    };

    let loopback_V4 = IpAddrEnum::V4(127, 0, 0, 1);
    let loopback_V6 = IpAddrEnum::V6(String::from("::1"));

    let normal_V4 = IpAddrNormal::V4(Ipv4Addr::new(127, 0, 0, 1));

    let m = Message::Write(String::from("hello"));
    m.call();
    
    let some_number = Some(5);
    let some_chat = Some('e');
    let absent_number: Option<i32> = None;

    let mut x = Some(2);
    match x.as_mut() {
        Some(v) => println!("{v}"),
        None => {},
    };

    match some_number {
        Some(v) => println!("{v}"),
        None => {},
    }

}
enum IpAddKind {
    V4,
    V6,
}

enum IpAddrEnum {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum IpAddrNormal {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

struct IpAddr {
    kind: IpAddKind,
    address: String,
}

fn route(ip_kind: IpAddKind) {
    
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self) {
        // 在这里定义方法体
    }
}

