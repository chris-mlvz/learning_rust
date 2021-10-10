// * How to Write Tests
// * The Anatomy of a Test Function
#[cfg(test)]
mod tests {
    use super::*;

    // * Checking Results with the assert! Macro
    // #[test]
    // fn larger_can_hold_smaller() {
    //     let larger = Rectangle {
    //         width: 8,
    //         height: 7,
    //     };
    //     let smaller = Rectangle {
    //         width: 5,
    //         height: 1,
    //     };
    //     assert!(larger.can_hold(&smaller));
    // }

    // #[test]
    // fn smaller_cannot_hold_larger() {
    //     let larger = Rectangle {
    //         width: 8,
    //         height: 7,
    //     };
    //     let smaller = Rectangle {
    //         width: 8,
    //         height: 1,
    //     };

    //     assert!(!smaller.can_hold(&larger));
    // }

    // * Testing Equality with the assert_eq! and assert_ne! Macros

    // #[test]
    // fn it_adds_two() {
    //     assert_eq!(4, add_two(2));
    // }

    // * Adding Custom Failure Messages
    // #[test]
    // fn greeting_contains_name() {
    //     let result = greeting("Carol");
    //     assert!(
    //         result.contains("Carol"),
    //         "Greeting did not contain name, value was '{}'",
    //         result
    //     );
    // }

    // * Checking for Panics with should_panic
    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}

// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width < other.width && self.height > other.height
//     }
// }

// pub fn add_two(a: i32) -> i32 {
//     a + 3
// }

// pub fn greeting(name: &str) -> String {
//     format!("Hello!")
// }

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be less than or equal to 100, got {}",
                value
            );
        } else if value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}
