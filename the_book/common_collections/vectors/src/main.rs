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
    // let mut v = vec![1, 2, 3, 4, 5];

    // let first = &v[0];

    // v.push(6);

    // println!("The first element is: {}", first);

    // * Iterating over the values in a vector
    // let v = vec![100, 32, 57];
    // for i in &v {
    //     println!("{}", i);
    // }

    // * Iterating over mutable references
    // let mut v = vec![100, 32, 57];
    // for i in &mut v {
    //     *i += 50;
    //     println!("{}", *i);
    // }
    // println!("{:?}", v);

    // * Using an Enum to Store Multiple Types
    #[derive(Debug)]
    enum SpreedsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreedsheetCell::Int(3),
        SpreedsheetCell::Text(String::from("blue")),
        SpreedsheetCell::Float(10.12),
    ];

    for elem in row {
        println!("{:?}", elem);
    }
}
