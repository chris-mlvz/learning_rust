use std::fmt::Display;

// Using Trait Bounds to Conditionally Implement Methods

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest number is x {}", self.x);
        } else {
            println!("The largest number is x {}", self.y);
        }
    }
}

fn main() {
    let pair = Pair::new(5.5, 5.1);
    pair.cmp_display()
}

// * implement a trait for any type that implements another trait
// impl<T: Display> ToString for T {
//     // --snip--
// }
