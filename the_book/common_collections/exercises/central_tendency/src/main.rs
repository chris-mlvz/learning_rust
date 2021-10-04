use std::collections::HashMap;
use std::io;

fn main() {
    println!("Central tendency measures [mean, median, mode]");
    println!("Introduce your number by spaces:");
    let mut user_data = String::new();
    io::stdin()
        .read_line(&mut user_data)
        .expect("Failed to read line");

    let mut numbers: Vec<i32> = user_data
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect();

    println!("{:?}", numbers);

    println!("Mean: {}", mean(&numbers));
    println!("Median: {}", median(&mut numbers));
    println!("Mode: {:?}", mode(&numbers));
}

fn mean(numbers: &Vec<i32>) -> i32 {
    let mut result = 0;
    for num in numbers {
        result += *num
    }
    result / numbers.len() as i32
}

fn median(numbers: &mut Vec<i32>) -> i32 {
    numbers.sort();

    let mid = numbers.len() / 2;
    if numbers.len() % 2 == 0 {
        mean(&vec![numbers[mid - 1], numbers[mid]]) as i32
    } else {
        numbers[mid]
    }
}

fn mode(numbers: &Vec<i32>) -> Vec<i32> {
    let mut map = HashMap::new();
    for integer in numbers {
        let count = map.entry(integer).or_insert(0);
        *count += 1;
    }

    let max_value = map.values().cloned().max().unwrap_or(0);

    map.into_iter()
        .filter(|&(_, v)| v == max_value)
        .map(|(&k, _)| k)
        .collect()
}
