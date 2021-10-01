// * References and Borrowing

// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);

//     println!("The length of '{}' is {}", s1, len);

// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// * Modify borrowing
fn main() {
    let mut s = String::from("hello");
    change(&mut s);

    {
        let r1 = &mut s;
        println!("{}", r1);
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;

    println!("{}", r2);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
