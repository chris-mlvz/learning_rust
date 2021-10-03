fn main() {
    // * Creating a New Vector
    // let v: Vec<i32> = Vec::new();
    // let vector = vec![1, 2, 3];

    // * Updating a Vector
    // let mut v = Vec::new();
    // v.push(5);
    // v.push(6);
    // v.push(7);
    // v.push(8);
    // println!("{:?}", v);

    // * Reading elements of Vectors
    // let v = vec![1, 2, 3, 4, 5];

    // let third: &i32 = &v[2];
    // println!("The third element is {}", third);

    // match v.get(2) {
    //     Some(third) => println!("The third element is {}", third),
    //     None => println!("There is not third element"),
    // }

    // * panic and None
    // let v = vec![1, 2, 3, 4, 5];

    // let does_not_exit = &v[100];
    // let does_not_exit = v.get(100);

    // * inmutable borrow
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    v.push(6);

    println!("The first element is: {}", first);
}
