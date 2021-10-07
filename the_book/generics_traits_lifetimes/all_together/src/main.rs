use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let s1 = "Hello";
    let s2 = "World";
    let ann = "All together";

    let result = longest_with_an_announcement(s1, s2, ann);
    println!("The largest number is {}", result);

    let c = '🍕';

    let result = longest_with_an_announcement(s1, s2, c);
    println!("The largest char is {}", result);

    println!("test");
}
