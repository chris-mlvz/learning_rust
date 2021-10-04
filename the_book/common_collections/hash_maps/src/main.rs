// * Hash Maps

// * Creating a New Hash Map
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:?}", scores);

    // * Another way of constructing a HashMap
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("{:?}", scores);

    // * Accessing Values in a Hash Map
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    match score {
        Some(x) => println!("{}", x),
        None => println!("Error getting the value"),
    }

    // * Updating a Hash Map
    // * Overwriting a Value
    let mut test_map = HashMap::new();
    test_map.insert(String::from("Blue"), 10);
    test_map.insert(String::from("Blue"), 25);

    println!("{:?}", test_map);
}
