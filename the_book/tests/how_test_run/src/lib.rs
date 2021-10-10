// * Controlling How Tests Are Run

// * Running Tests in Parallel or Consecutively
// Default parallel

// Consecutively
// $ cargo test -- --test-threads=1

// * Showing Function Output
// $ cargo test -- --show-output
fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }
}
