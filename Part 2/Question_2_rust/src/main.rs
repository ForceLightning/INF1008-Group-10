use std::{env, fs};
use std::collections::{HashMap, BTreeMap};

/// This function is the entry point of the program.
/// It takes in a file name, a target number, and a k value.
/// It will then read the file and find the k nearest numbers to the target number.
/// In the case of a tie between the last 2 numbers, it will print both numbers.
/// # Arguments
/// * `filename` - The name of the file to read
/// * `target_number` - The target number to find the k nearest numbers to
/// * `k` - The number of nearest numbers to return
/// # Example
/// ```bash
/// cargo run --release -- phonescraped 1234567890 3
/// > 1234567890
/// > 1234567891
/// > 1234567889
/// ```
fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 4, "Usage: cargo run --release -- <filename> <target_number> <k>");
    let filename = &args[1];
    let target_number = phone_number_sanitizer(&args[2]).unwrap();
    let k = &args[3].parse::<i64>().unwrap();

    if *k <= 0 {
        panic!("k must be greater than 0");
    }

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let phone_numbers: Vec<i64> = contents
        .split_whitespace()
        .map(|number| { // clean the phone number string
            let cleaned: String = number
                .chars()
                .filter(|c| c.is_digit(10)) // filter out non-digits
                .collect();
            cleaned
        })
        .filter(|cleaned| cleaned.len() >= 10 && cleaned.len() <= 11) // filter out numbers that are too short or too long length wise
        .map(|cleaned| cleaned.chars().rev().take(10).collect::<String>().chars().rev().collect::<String>().parse().unwrap()) // take the last 10 characters and parse it into an integer
        .filter(|number| number >= &1000000000) // filter out numbers that are less than 10 digits long
        .collect();
    let result = kth_nearest(*k, target_number, phone_numbers);
    result.iter().for_each(|number| println!("{}", number));
}


/// Cleans a phone number string and returns a 64 bit integer
/// # Arguments
/// * `phone_number` - The phone number to clean
/// # Errors
/// This function will return an error if the phone number is less than 10 digits long
/// or if the phone number contains invalid characters
/// # Example
/// ```rust
/// let result = phone_number_sanitizer(&String::from("1234567890"));
/// assert_eq!(result, Ok(1234567890));
/// ```
fn phone_number_sanitizer(phone_number: &String) -> Result<i64, &'static str> {
    let cleaned = phone_number
        .chars()
        .filter(|c| c.is_digit(10)) // filter out non-digits
        .collect::<String>()
        .chars()
        .rev() // reverse the string and take the last 10 characters
        .take(10)
        .collect::<String>()
        .chars()
        .rev() // reverse it back again
        .collect::<String>() // collect it back into a string
        .parse::<i64>(); // parse it into an integer
    match cleaned {
        Ok(number) => {
            if number < 1000000000 {
                Err("Phone number must be at least 10 digits long")
            } else {
                Ok(number)
            }
        },
        Err(_) => Err("Phone number has invalid characters")
    }
}

/// Returns the k nearest numbers to the target number
/// while accounting for duplicates
/// # Arguments
/// * `k` - The number of nearest numbers to return
/// * `target` - The target number
/// * `numbers` - The list of numbers to search
/// # Example
/// ```rust
/// let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
/// let result = kth_nearest(3, 5, numbers);
/// assert_eq!(result, vec![4, 5, 6]);
/// ```
fn kth_nearest(k: i64, target: i64, numbers: Vec<i64>) -> Vec<i64> {
    // count the number of times each number appears in the list
    let mut numbers_counter = HashMap::new();
    numbers.iter().for_each(|number| {
        let count = numbers_counter.entry(number).or_insert(0);
        *count += 1;
    });
    // calculate the differences between each number and the target number,
    // and store the numbers that have the same difference in a BTreeMap
    let mut differences : BTreeMap<i64, Vec<i64>> = BTreeMap::new();
    for (number, _) in numbers_counter.iter() {
        let abs_distance = (*number - target).abs();
        let difference_count = differences.entry(abs_distance).or_insert(Vec::new());
        difference_count.push(*number.clone());
    }
    let mut result: Vec<i64> = Vec::new();
    let mut count = 0;
    for (_, numbers) in differences.iter().take(k as usize) { // take only k keys from the BTreeMap
        let mut numbers_to_add: Vec<i64> = Vec::new();
        for number in numbers {
            count += 1;
            let num_times = numbers_counter.get(number).unwrap();
            numbers_to_add.append(&mut vec![*number; *num_times as usize]);
        }
        result.append(&mut numbers_to_add);
        if count >= k as usize {
            // we have enough numbers, so we can stop
            break;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    use rand_chacha::{ChaChaRng, rand_core::SeedableRng};
    use std::io::Write;
    use std::time::{Duration, Instant};
    use rayon::prelude::*;
    use rayon::iter::ParallelIterator;
    use indicatif::{ParallelProgressIterator, ProgressStyle, ProgressIterator};

    /// This test will generate 100 random phone numbers and
    /// ensure that they are all sanitized correctly
    /// # Example
    /// ```bash
    /// cargo test --release -- test_sanitizer_valid_numbers
    /// ```
    #[test]
    fn test_sanitizer_valid_numbers() {
        for _ in 0..100 {
            let (phone_number, _) = valid_phone_number_generator();
            let result = phone_number_sanitizer(&phone_number.to_string());
            assert_eq!(result, Ok(phone_number));
        }
    }

    /// This test will generate 100 incorrect random phone numbers and
    /// ensure that they are all not sanitized
    /// # Example
    /// ```bash
    /// cargo test --release -- test_sanitizer_invalid_numbers
    /// ```
    #[test]
    fn test_sanitizer_invalid_numbers() {
        for _ in 0..100 {
            let phone_number = invalid_phone_number_generator();
            let result = phone_number_sanitizer(&phone_number);
            match result {
                Ok(_) => panic!("Invalid phone number was sanitized"),
                Err(_) => {}
            }
        }
    }

    /// This function generates a random valid phone number
    /// as a tuple of (i64, String) where the i64 is the
    /// sanitized phone number and the String is the original
    /// phone number
    /// # Example
    /// ```rust
    /// let (phone_number, original) = valid_phone_number_generator();
    /// assert!(phone_number = phone_number_sanitizer(&original).unwrap());
    /// ```
    fn valid_phone_number_generator() -> (i64, String) {
        let mut rng = ChaChaRng::seed_from_u64(0);
        // generate the 3 parts of a US phone number
        let first_part = rng.gen_range(100..1000);
        let second_part = rng.gen_range(0..1000);
        let third_part = rng.gen_range(0..10000);
        let prefix = match rng.gen_bool(0.5) {
            // US prefix
            true => "+1".to_owned(),
            false => "".to_owned()
        };
        // include dashses or spaces or none:
        let separator = match rng.gen_range(0..3) {
            0 => "-".to_owned(),
            1 => " ".to_owned(),
            _ => "".to_owned()
        };
        let phone_number = format!("{}{:0>3}{}{:0>3}{}{:0>4}", prefix, first_part, separator, second_part, separator, third_part);
        let phone_number_int = format!("{}{:0>3}{:0>3}{:0>4}", prefix, first_part, second_part, third_part).parse::<i64>().unwrap();
        (phone_number_int, phone_number)
    }

    /// This function generates a random invalid phone number
    /// as a String
    /// # Example
    /// ```rust
    /// let phone_number = invalid_phone_number_generator();
    /// assert!(phone_number_sanitizer(&phone_number).is_err());
    /// ```
    fn invalid_phone_number_generator() -> String {
        let mut rng = ChaChaRng::seed_from_u64(0);
        // generate the 3 parts of a US phone number
        let first_part = rng.gen_range(0..100); // invalid first part
        let second_part = rng.gen_range(0..1000);
        let third_part = rng.gen_range(0..10000);
        let prefix = match rng.gen_bool(0.5) {
            // add US prefix at random
            true => "+1".to_owned(),
            false => "".to_owned()
        };
        let mut number = format!("{}{:0>3}{:0>3}{:0>4}", prefix, first_part, second_part, third_part);
        // replace a random character with a non-digit at random
        match rng.gen_bool(0.5) {
            true => {
                rng.gen_range(0..number.len());
                let index = rng.gen_range(0..number.len());
                number.replace_range(index..index+1, "a");
            },
            false => {}
        }
        number
    }

    /// This test will test the kth-nearest function
    /// on a predefined set of numbers
    #[test]
    fn test_kth_nearest_pre_defined() {
        let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let result = kth_nearest(4, 5, numbers);
        assert_eq!(result, vec![3, 4, 5, 6, 7]);
    }

    /// This function is a solution that implements the quicksort algorithm
    /// to sort a vector of i64s then returns the kth nearest numbers
    /// to the target number
    /// # Arguments
    /// * `k` - the number of nearest numbers to return
    /// * `target` - the target number
    /// * `numbers` - the vector of numbers to search
    /// # Returns
    /// * `Vec<i64>` - the kth nearest numbers to the target number
    /// # Example
    /// ```rust
    /// let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    /// let result = kth_nearest(4, 5, numbers);
    /// assert_eq!(result, vec![3, 4, 5, 6, 7]);
    /// ```
    fn naive_sorting_find_kth_nearest(k: u64, target: i64, numbers: Vec<i64>) -> Vec<i64> {
        let mut sorted_numbers = numbers.clone();
        quicksort(&mut sorted_numbers);
        let mut differences = HashMap::new();
        // store the count of each number
        for number in sorted_numbers {
            let distance = number - target;
            let difference_count = differences.entry(distance).or_insert(0);
            *difference_count += 1;
        }
        let mut differences_sorted = differences.keys().collect::<Vec<&i64>>();
        differences_sorted.sort_by(|a, b| a.abs().cmp(&b.abs())); // sort by absolute distance from target
        let mut result: Vec<i64>  = Vec::new();
        let mut last_abs_distance: i64 = *differences.get(differences_sorted[0]).unwrap() as i64;
        let mut count = 0;
        for difference in differences_sorted {
            let num_times = differences.get(difference).unwrap();
            let actual_number = target + difference;
            for _ in 0..*num_times {
                result.push(actual_number);
            }
            let abs_distance = difference.abs();
            count += 1;
            if abs_distance > last_abs_distance {
                last_abs_distance = abs_distance;
                if count >= k {
                    // we have found the kth or k+1th nearest number
                    break;
                }
            }
        }
        return result;
    }

    /// This function is for testing the performance of the quick sort algorithm
    /// to the quick select algorithm.
    /// # Arguments
    /// * `arr` - A vector of T that is to be sorted    
    fn quicksort<T: PartialOrd>(arr: &mut [T]) {
        if arr.len() < 1 {
            return;
        }
        let pivot = partition(arr);
        quicksort(&mut arr[..pivot]);
        quicksort(&mut arr[pivot + 1..]);
    }

    /// This function is used by the quick sort algorithm to partition the vector.
    /// # Arguments
    /// * `arr` - A vector of T that is to be sorted
    /// # Returns
    /// * `usize` - The index of the pivot element
    /// # Example
    /// ```
    /// let mut arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    /// let pivot = partition(&mut arr);
    /// ```
    fn partition<T: PartialOrd>(arr: &mut [T]) -> usize {
        let len = arr.len();
        // let pivot_index = len / 2;
        // select a random pivot index
        let mut rng = rand::thread_rng();
        let pivot_index = rng.gen_range(0..len);
        arr.swap(pivot_index, len - 1);
        let mut store_index = 0;
        for i in 0..len - 1 {
            if arr[i] < arr[len - 1] {
                arr.swap(i, store_index);
                store_index += 1;
            }
        }
        arr.swap(store_index, len - 1);
        store_index
    }

    /// This function is used to test the kth-nearest function
    /// on a random set of 10 digit numbers.
    /// The test will run 1000 times and will print the progress
    /// of the test.
    #[test]
    fn test_kth_nearest_random() {
        (0..1000).progress_count(1000).for_each(|_| {
            let max_length = 10000;
            let min_number = 1000000000;
            let max_number = 1999999999;
            let seed = rand::thread_rng().gen_range(0..1<<32);
            let mut rng = ChaChaRng::seed_from_u64(seed);
            let numbers: Vec<i64> = (0..max_length).map(|_| rng.gen_range(0..max_number)).collect();
            let target = rng.gen_range(min_number..=max_number);
            let k = rng.gen_range(1..=max_length);
            let mut our_result = kth_nearest(k, target, numbers.clone());
            let mut naive_result = naive_sorting_find_kth_nearest(k as u64, target, numbers);
            our_result.sort();
            naive_result.sort();
            assert_eq!(our_result, naive_result);
        });
    }

    /// This function generates a vector of random 10 digit numbers.
    /// # Arguments
    /// * `num_numbers` - the number of numbers to generate
    /// * `min_value` - the minimum value of the numbers
    /// * `max_value` - the maximum value of the numbers
    /// * `seed` - the seed for the random number generator
    /// # Returns
    /// * `Vec<i64>` - a vector of random 10 digit numbers
    /// # Example
    /// ```rust
    /// let numbers = generate_valid_phone_numbers(100, 1000000000, 1999999999, 0);
    /// ```
    fn generate_valid_phone_numbers(num_numbers: usize, min_value: u64, max_value: u64, seed: u64) -> Vec<i64> {
        let mut rng = ChaChaRng::seed_from_u64(seed);
        let mut numbers: Vec<i64> = Vec::new();
        for _ in 0..num_numbers {
            let number = rng.gen_range(min_value..=max_value);
            numbers.push(number as i64);
        }
        numbers
    }

    /// This function is used to test the kth-nearest function against
    /// the naive sorting algorithm.
    /// # Arguments
    /// * `max_length` - the maximum length of the vector to test
    /// * `seed` - the seed for the random number generator
    /// # Returns
    /// * `Vec<(usize, Duration, Duration)>` - a vector of tuples containing the length of the vector,
    /// the time it took to run the naive sorting algorithm, and the time it took to run the btreemap implementation
    /// # Example
    /// ```rust
    /// let results = _compare_naive_sort_and_btreemap(100, 0);
    /// ```
    fn _compare_naive_sort_and_btreemap(max_length: usize, seed: u64) -> Vec<(usize, Duration, Duration)> {
        let style = ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}")
            .unwrap()
            .progress_chars("##-");
        let results = (1..=max_length)
            .into_par_iter()
            .progress_count(max_length as u64)
            .with_style(style)
            .map(|length| {
                let numbers = generate_valid_phone_numbers(length, 1000000000, 9999999999, seed);
                let mut rng = ChaChaRng::seed_from_u64(seed);
                let target = rng.gen_range(1000000000..=9999999999); // generate valid phone number target
                let k = rng.gen_range(1..=length);
                let durations = compare_naive_sort_and_btreemap(5, numbers, target, k as u64);
                (length, durations.0, durations.1)
            }).collect();
        results
    }

    /// This function is used to test the kth-nearest function against
    /// the naive sorting algorithm.
    /// # Arguments
    /// * `num_times` - the number of times to run the test and take the average for
    /// * `numbers` - the vector of numbers to test
    /// * `target` - the target number
    /// * `k` - the kth nearest number to find
    /// # Returns
    /// * `(Duration, Duration)` - a tuple containing the time it took to run the naive sorting algorithm,
    /// and the time it took to run the btreemap implementation
    /// # Example
    /// ```rust
    /// let durations = compare_naive_sort_and_btreemap(5, numbers, target, k as u64);
    /// ```
    fn compare_naive_sort_and_btreemap(num_times: u32, numbers: Vec<i64>, target: i64, k: u64) -> (Duration, Duration) {
        let mut sum_naive_duration = Duration::new(0, 0);
        let mut sum_btreemap_duration = Duration::new(0, 0);
        for _ in 0..num_times {
            let numbers_clone_naive = numbers.clone();
            let numbers_clone_btreemap = numbers.clone();
            let naive_start = Instant::now();
            let mut naive_result = naive_sorting_find_kth_nearest(k, target, numbers_clone_naive);
            let naive_end = Instant::now();
            let naive_duration = naive_end.duration_since(naive_start);
            sum_naive_duration += naive_duration;
            let btreemap_start = Instant::now();
            let mut btreemap_result = kth_nearest(k as i64, target, numbers_clone_btreemap);
            let btreemap_end = Instant::now();
            let btreemap_duration = btreemap_end.duration_since(btreemap_start);
            sum_btreemap_duration += btreemap_duration;
            naive_result.sort();
            btreemap_result.sort();
            // assert_eq!(naive_result, btreemap_result);
        }
        let avg_naive_duration = sum_naive_duration / num_times;
        let avg_btreemap_duration = sum_btreemap_duration / num_times;
        (avg_naive_duration, avg_btreemap_duration)
    }

    /// This test compares the naive sorting implementation with the btreemap implementation.
    /// It generates a csv file with the results.
    /// The csv file can be opened with a spreadsheet program like LibreOffice Calc.
    /// The first column is the length of the input array.
    /// The second column is the average duration of the naive sorting implementation in nanoseconds.
    /// The third column is the average duration of the btreemap implementation in nanoseconds.
    /// # Example
    /// ```ignore
    /// $ cargo test --release -- --nocapture compare_naive_and_our_implementation
    /// ```
    #[test]
    fn compare_naive_and_our_implementation() {
        let max_length = 50000;
        let seed = 42;
        let mut file = match std::fs::File::create("results.csv") {
            Ok(file) => file,
            Err(e) => panic!("Error creating file: {}", e),
        };
        
        let results = _compare_naive_sort_and_btreemap(max_length, seed);
        results.iter().for_each(|(length, naive_duration, btreemap_duration)| {
            let naive_duration_millis = naive_duration.as_nanos();
            let btreemap_duration_millis = btreemap_duration.as_nanos();
            let _ = writeln!(file, "{},{},{}", length, naive_duration_millis, btreemap_duration_millis);
            // println!("{},{},{}", length, naive_duration_millis, btreemap_duration_millis)
        });
    }
}