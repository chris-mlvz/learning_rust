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
// fn main() {
//     let mut number = 3;
//     while number != 0 {
//         println!("{}", number);

//         number -= 1;
//     }

//     println!("LIFTOFF!!!");
// }

// * Looping Through a Collection with for
// fn main() {
//     let a = [10, 20, 30, 40, 50];

//     for elem in a.iter() {
//         println!("the value is : {}", elem);
//     }

// }

fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
