// * The Iterator Trait and the next Method
// pub trait Iterator {
//   type Item;

//   fn next(&mut self) -> Option<Self::Item> {
//     // methods with default implementations elided
//   }
// }

#[cfg(test)]
mod tests {
  #[test]
  fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
  }
}
