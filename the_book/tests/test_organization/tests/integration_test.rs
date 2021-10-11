use test_organization;

mod common;

#[test]
fn it_adds_two() {
  common::setup();
  assert_eq!(4, test_organization::add_two(2));
}

// To run all the tests in a particular integration test file
// cargo test --test integration_test
