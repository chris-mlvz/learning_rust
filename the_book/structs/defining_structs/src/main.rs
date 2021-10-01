// * Defining and Instantiating Structs
// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }

// fn main() {
//     let user1 = build_user(String::from("mail"), String::from("name"));
//     // * Creating Instances From Other Instances With Struct Update Syntax
//     let user2 = User {
//         email: String::from("another@example.com"),
//         username: String::from("anotherusername567"),
//         ..user1
//     };
//     println!("{}", user1.active);
// }

// * Using the Field Init Shorthand when Variables and Fields Have the Same Name
// fn build_user(email: String, username: String) -> User {
//     User {
//         email,
//         username,
//         active: true,
//         sign_in_count: 1,
//     }
// }

// * Using Tuple Structs without Named Fields to Create Different Types
fn main() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("{},{}", black.0, origin.0);
}
