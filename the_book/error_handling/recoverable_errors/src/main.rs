// * Recoverable Errors with Result

// use std::fs::File;

// fn main() {
//     let f = File::open("hello.txt");

//     let f = match f {
//         Ok(file) => file,
//         Err(error) => panic!("Problem opening the file: {:?}", error),
//     };
// }

// * Matching on Different Errors
// use std::fs::File;
// use std::io::ErrorKind;

// fn main() {
//     let f = File::open("hello.txt");

//     let f = match f {
//         Ok(file) => file,
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create("hello.txt") {
//                 Ok(fc) => fc,
//                 Err(e) => panic!("Problem creating the file: {:?}", e),
//             },
//             other_error => {
//                 panic!("Problem opening the file: {:?}", other_error);
//             }
//         },
//     };
//     println!("{:?}", f);
// }

// * Matching on Different Errors with closures
// use std::fs::File;
// use std::io::ErrorKind;

// fn main() {
//     let f = File::open("hello.txt").unwrap_or_else(|error| {
//         if error.kind() == ErrorKind::NotFound {
//             File::create("hello.txt").unwrap_or_else(|error| {
//                 panic!("Problem creating the file: {:?}", error);
//             })
//         } else {
//             panic!("Problem opening the file: {:?}", error);
//         }
//     });
// }

// * Shortcuts for Panic on Error: unwrap and expect
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
    let f1 = File::open("hello.txt").expect("Failed to open hello.txt");
}
