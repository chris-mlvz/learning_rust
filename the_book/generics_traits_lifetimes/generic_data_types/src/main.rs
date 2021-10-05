// * Generic Data Types

// * In Function Definitions
// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest(&number_list);
//     println!("The largest number is {}", result);

//     let char_list = vec!['y', 'm', 'a', 'q'];

//     let result = largest(&char_list);
//     println!("The largest char is {}", result);
// }

// * In Struct Definitions
// #[derive(Debug)]
// struct Point<T> {
//     x: T,
//     y: T,
// }

// fn main() {
//     let integer = Point { x: 5, y: 10 };
//     let float = Point { x: 1.0, y: 4.0 };
//     println!("{:?}", integer);
//     println!("{:?}", float);
// }

// ! Don't work
// struct Point<T> {
//     x: T,
//     y: T,
// }

// fn main() {
//     let wont_work = Point { x: 5, y: 4.0 };
// }
// #[derive(Debug)]
// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// fn main() {
//     let both_integer = Point { x: 5, y: 10 };
//     let both_float = Point { x: 1.0, y: 4.0 };
//     let integer_and_float = Point { x: 5, y: 4.0 };

//     println!("{:?}", both_integer);
//     println!("{:?}", both_float);
//     println!("{:?}", integer_and_float);
// }

// * In Enum Definitions
// fn main() {

// }

// enum Option<T> {
//     Some(T),
//     None,
// }

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// * In Method Definitions
// struct Point<T> {
//     x: T,
//     y: T,
// }

// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

// fn main() {
//     let p = Point { x: 5, y: 10 };
//     println!("p.x = {}", p.x());
// }

// * A method that uses different generic types from its struct’s definition
// #[derive(Debug)]
// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// impl<T, U> Point<T, U> {
//     fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
//         Point {
//             x: self.x,
//             y: other.y,
//         }
//     }
// }

// fn main() {
//     let p1 = Point { x: 5, y: 10.4 };
//     let p2 = Point { x: "Hello", y: 'c' };

//     let p3 = p1.mixup(p2);
//     println!("{:?}", p3);
// }

// * struct with a particular concrete type for the generic type parameter T
// impl Point<f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }

// * Performance of Code Using Generics
// fn main() {
//     let integer = Some(5);
//     let float = Some(5.0);
// }

// Rust compiles generic code into code that specifies the type in each
// instance, we pay no runtime cost for using generics. When the code runs,
// it performs just as it would if we had duplicated each definition by hand.

enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}