// * Test Organization
// * Unit Tests
// unit tests go in the same files as the code, you’ll use #[cfg(test)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // * Testing Private Functions
    //  Rust’s privacy rules do allow you to test private functions
    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}
