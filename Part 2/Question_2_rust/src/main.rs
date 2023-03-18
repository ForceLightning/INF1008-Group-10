use std::{env, fs};
use std::collections::{HashMap, BTreeMap};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let target_number = &args[2].parse::<i64>().unwrap();
    let k = &args[3].parse::<u32>().unwrap();

    if *k <= 0 {
        panic!("k must be greater than 0");
    }

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
    let result = kth_nearest(*k, *target_number, phone_numbers);
    result.iter().for_each(|number| println!("{}", number));
}

/// Returns the k nearest numbers to the target number
/// while accounting for duplicates
/// # Arguments
/// * `k` - The number of nearest numbers to return
/// * `target` - The target number
/// * `numbers` - The list of numbers to search
/// # Example
/// ```
/// let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
/// let result = kth_nearest(3, 5, numbers);
/// assert_eq!(result, vec![4, 5, 6]);
/// ```
fn kth_nearest(k: u32, target: i64, numbers: Vec<i64>) -> Vec<i64> {
    let mut numbers_counter = HashMap::new();
    numbers.iter().for_each(|number| {
        let count = numbers_counter.entry(number).or_insert(0);
        *count += 1;
    });
    let mut differences : BTreeMap<i64, Vec<i64>> = BTreeMap::new();
    for (number, _) in numbers_counter.iter() {
        let abs_distance = (*number - target).abs();
        let difference_count = differences.entry(abs_distance).or_insert(Vec::new());
        difference_count.push(*number.clone());
    }
    // iterate k keys in the BTreeMap and return that to a result
    let result: Vec<i64> = differences.keys().take(k as usize).flat_map(|difference| {
        let numbers = differences.get(difference).unwrap();
        let internal_result: Vec<i64> = numbers.iter().flat_map(|number| {
            let num_times = numbers_counter.get(number).unwrap();
            vec![number.clone(); *num_times as usize]
        }).collect();
        internal_result
    }).collect();
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

    #[test]
    fn test_kth_nearest_pre_defined() {
        let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let result = kth_nearest(3, 5, numbers);
        assert_eq!(result, vec![4, 5, 6]);
    }

    fn naive_sorting_find_kth_nearest(k: u64, target: i64, numbers: Vec<i64>) -> Vec<i64> {
        let mut sorted_numbers = numbers.clone();
        quicksort(&mut sorted_numbers);
        // sorted_numbers.sort();
        let mut differences = HashMap::new();
        for number in sorted_numbers {
            let distance = number - target;
            let difference_count = differences.entry(distance).or_insert(0);
            *difference_count += 1;
        }
        let mut differences_sorted = differences.keys().collect::<Vec<&i64>>();
        differences_sorted.sort_by(|a, b| a.abs().cmp(&b.abs()));
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
            if abs_distance > last_abs_distance {
                count += 1;
                last_abs_distance = abs_distance;
            }
            if count >= k {
                break;
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

    #[test]
    fn test_kth_nearest_random() {
        (0..1000).progress_count(1000).for_each(|_| {
            let max_length = 10000;
            let min_number = 1000000000;
            let max_number = 9999999999;
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

    fn generate_valid_phone_numbers(num_numbers: usize, min_value: u64, max_value: u64, seed: u64) -> Vec<i64> {
        let mut rng = ChaChaRng::seed_from_u64(seed);
        let mut numbers: Vec<i64> = Vec::new();
        for _ in 0..num_numbers {
            let number = rng.gen_range(min_value..=max_value);
            numbers.push(number as i64);
        }
        numbers
    }

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
                let target = rng.gen_range(1000000000..=9999999999);
                let k = rng.gen_range(1..=length);
                let durations = compare_naive_sort_and_btreemap(5, numbers, target, k as u64);
                (length, durations.0, durations.1)
            }).collect();
        results
    }

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
            let mut btreemap_result = kth_nearest(k as u32, target, numbers_clone_btreemap);
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

    #[test]
    fn compare_naive_and_our_implementation() {
        let max_length = 10000;
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