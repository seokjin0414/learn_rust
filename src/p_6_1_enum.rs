fn p_6_1_enum() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    fn route(ip_kind: IpAddrKind) {}

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddr::V4(String::from("123.0.0.7"));
    let loopback = IpAddr::V6(String::from("::1"));

    let home = IpAddr2::V4(127, 0, 0, 1);
    let loopback = IpAddr2::V6(String::from("::1"));

    let m = Message::Write(String::from("HAHAHA"));
    m.call();

    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y;
}

enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(String),
    V6(String),
}

enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct Ipv4Addr {
}

struct Ipv6Addr {
}

enum IpAddr3 {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

enum Message {
    Quit,
    Move { x: i32 , y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {

    }
}
