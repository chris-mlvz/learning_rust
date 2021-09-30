// * Repeating Code with loop
// fn main() {
//     loop {
//         println!("again!");
//     }
// }

// * Returning Values from Loops
// fn main() {
//     let mut counter = 0;

//     let result = loop {
//         counter += 1;

//         if counter == 10 {
//             break counter * 2;
//         }
//     };

//     println!("The result is {}", result);
// }

// * Conditional Loops with while
fn main() {
    let mut number = 3;
    while number != 0 {
        println!("{}", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
