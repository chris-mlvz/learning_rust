// * Closures: Anonymous Functions that Can Capture Their Environment
// use std::thread;
// use std::time::Duration;

// fn main() {
//     let simulated_user_specified_value = 50;
//     let simulated_random_number = 3;

//     generate_workout(simulated_user_specified_value, simulated_random_number)
// }

// fn generate_workout(intensity: u32, random_number: u32) {
//     let mut expensive_result = Cacher::new(|num| {
//         println!("calculating slowly...");
//         thread::sleep(Duration::from_secs(2));
//         num
//     });

//     if intensity < 25 {
//         println!("Today, do {} pushups!", expensive_result.value(intensity));
//         println!("Next, do {} situps!", expensive_result.value(intensity));
//     } else {
//         if random_number == 3 {
//             println!("Take a break today! Remember to stay hydrated!");
//         } else {
//             println!(
//                 "Today, run for {} minutes!",
//                 expensive_result.value(intensity)
//             );
//         }
//     }
// }

// Limitations of the Cacher Implementation
// ToDo: Add hash map to value
// ToDo: Make more generic, take and return  different parameters

// struct Cacher<T>
// where
//     T: Fn(u32) -> u32,
// {
//     calculation: T,
//     value: Option<u32>,
// }

// impl<T> Cacher<T>
// where
//     T: Fn(u32) -> u32,
// {
//     fn new(calculation: T) -> Cacher<T> {
//         Cacher {
//             calculation,
//             value: None,
//         }
//     }

//     fn value(&mut self, arg: u32) -> u32 {
//         match self.value {
//             Some(v) => v,
//             None => {
//                 let v = (self.calculation)(arg);
//                 self.value = Some(v);
//                 v
//             }
//         }
//     }
// }

// * Closures, syntax
// fn  add_one_v1   (x: u32) -> u32 { x + 1 }
// let add_one_v2 = |x: u32| -> u32 { x + 1 };
// let add_one_v3 = |x|             { x + 1 };
// let add_one_v4 = |x|               x + 1  ;

// * Capturing the Environment with Closures
// fn main() {
//     let x = 4;

//     let equal_to_x = |z| z == x;

//     let y = 4;

//     assert!(equal_to_x(y));
// }

// ! Move keyword
fn main() {
    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;

    // println!("can't use x here: {:?}", x);

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}
