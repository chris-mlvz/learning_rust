fn main() {
    // * Creating a new String
    // let mut s = String::new();

    // * to_string method
    // let data = "initial contents".to_string();

    // * String From
    // let s = String::from("initial contents");

    // * Updating a String
    // let mut s = String::from("foo");
    // s.push_str("bar");

    // let mut s1 = String::from("foo");
    // let s2 = "bar";
    // s1.push_str(s2);
    // println!("s2 is {}", s2);

    // let mut s2 = String::from("lo");
    // s.push('l');

    // * Concatenation with the + Operator or the format! Macro
    // let s1 = String::from("Hello, ");
    // let s2 = String::from("world!");
    // let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    // println!("{}{}", s2, s3);

    // let s1 = String::from("tic");
    // let s2 = String::from("tac");
    // let s3 = String::from("toe");

    // let s = s1 + "-" + &s2 + "-" + &s3;
    // println!("{}", s);

    // // * format macro
    // let format_str = format!("{}-{}-{}", s2, s2, s3);
    // println!("{}", format_str);

    // ! INDEXING INTO STRINGS
    // let s1 = String::from("hello");
    // let h = s1[0];

    // let hello = "Здравствуйте";
    // let answer = &hello[0];

    // * Slicing Strings
    // let hello = "Здравствуйте";
    // let s = &hello[0..2];
    // println!("{}", s);

    // * Methods for Iterating Over Strings
    let s = String::from("नमस्ते🍕");
    for c in s.chars() {
        println!("{}", c);
    }
    for b in s.bytes() {
        println!("{}", b);
    }

    println!("{} {}", s, s.len());
}
