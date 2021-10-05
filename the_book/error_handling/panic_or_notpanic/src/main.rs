// * To panic! or Not to panic!

// * Cases in Which You Have More Information Than the Compiler
use std::net::IpAddr;

fn main() {
    let home: IpAddr = "127.0.0.1".parse().unwrap();
}
