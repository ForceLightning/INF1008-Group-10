use std::{env, fs};
use std::collections::{HashMap, BTreeMap};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let target_number = &args[2].parse::<i64>().unwrap();
    let k = &args[3].parse::<u32>().unwrap();

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let phone_numbers: Vec<i64> = contents
        .split_whitespace()
        .map(|number| {
            let cleaned: String = number
                .chars()
                .filter(|c| c.is_digit(10))
                .collect();
            cleaned
        })
        .filter(|cleaned| cleaned.len() >= 10)
        .map(|cleaned| cleaned.chars().rev().take(10).collect::<String>().chars().rev().collect::<String>().parse().unwrap())
        .collect();

    let mut phone_numbers_hash_map = HashMap::new();
    for number in phone_numbers {
        let count = phone_numbers_hash_map.entry(number).or_insert(0);
        *count += 1;
    }

    // dbg!(&phone_numbers_hash_map);

    let mut differences : BTreeMap<i64, Vec<i64>> = BTreeMap::new();
    for (number, count) in &phone_numbers_hash_map {
        let abs_difference = (number - target_number).abs();
        let difference_count = differences.entry(abs_difference).or_insert(Vec::new());
        difference_count.push(number.clone());
    }
    let mut count = 0;
    for difference in differences.keys() {
        for number in differences.get(difference).unwrap() {
            let num_times = phone_numbers_hash_map.get(number).unwrap();
            for i in 0..*num_times {
                println!("{}", &number);
            }
            count += 1;
        }
        if count >= *k {
            return;
        }
    }
}
