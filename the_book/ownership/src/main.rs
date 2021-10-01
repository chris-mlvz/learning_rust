// * The string type

// fn main() {
//     let mut s = String::from("hello");

//     s.push_str(", world!"); // push_str() appends a literal to a String

//     println!("{}", s); // This will print `hello, world!`
// }

// * Memory and Allocation
fn main() {
    let x = 5;
    let y = x;
    println!("{},{}", x, y);

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{},{}", s1, s2);
}
