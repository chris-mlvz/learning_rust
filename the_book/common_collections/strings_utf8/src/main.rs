fn main() {
    // * Creating a new String
    // let mut s = String::new();

    // * to_string method
    // let data = "initial contents".to_string();

    // * String From
    // let s = String::from("initial contents");

    // * Updating a String
    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    let mut s2 = String::from("lo");
    s.push('l');
}
