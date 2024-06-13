use std::any::Any;


fn main() {


    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let _home = IpAddr::V4(String::from("127.0.0.1"));
    let _loopback = IpAddr::V6(String::from("::1"));

    let msg = Message::Write(String::from("hello"));
    let _mv = Message::Move {x: 0, y: 1};
    let _q = Message::Quit;
    let _cc = Message::ChangeColor(255, 255, 255);
    msg.call();


    let _some_number = Some(5); // Option<i32>
    let _some_char = Some('e'); // Option<char>

    let _absent_number: Option<i32> = None;

}

enum IpAddrKind {
    V4,
    V6,
}

fn route(_ip_kind: IpAddrKind) {

}

enum IpAddr {
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}


impl Message {
    fn call(&self) {
        

    }
}
