// * Defining an Enum
// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

// fn main() {
//     let home = IpAddr::V4(127, 0, 0, 1);

//     let loopback = IpAddr::V6(String::from("::1"));
// }

// * another example
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// * Methods on  enums
impl Message {
    fn call(&self) {
        println!("{}", "hi");
    }
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();
}
