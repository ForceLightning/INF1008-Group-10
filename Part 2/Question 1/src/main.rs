use std::env;

/// This function takes a list of phone numbers as command line arguments,
/// cleans them up, and finds the median value(s).
/// It uses quick select to find the median value(s).
/// It has an average time complexity of O(n), but worst case is O(n^2).
/// # Examples
/// ```
/// cargo run "123-456-7890" "(323) 456-7890" "+1 223-456-7890" "1-322-345-7890" "322 555 0000"
/// > 3223457890
/// ```
fn main() {
    let args: Vec<String>  = env::args().collect();
    assert!(args.len() > 1, "Please provide at least one phone number as a command line argument");
    let phone_numbers: Vec<i64> = args[1..]
        .iter()
        .map(|number| {
            let cleaned: String = number
                .chars()
                .filter(|&c| c.is_digit(10)) // filter out non-digits
                .collect();
            cleaned
        })
        .filter(|cleaned| cleaned.len() >= 10 && cleaned.len() <= 11) // filter out numbers with less than 10 digits
        .map(|cleaned| cleaned.chars().rev().take(10).collect::<String>().chars().rev().collect::<String>()) // take last 10 digits
        .map(|cleaned| cleaned.parse::<i64>().unwrap()) // parse to i64
        .filter(|number| number >= &1000000000) // filter out numbers with less than 10 digits
        .collect();
    // use quick select to find the median phone number
    // quick select has an average time complexity of O(n), but worst case is O(n^2)
    let res = find_median_values(&phone_numbers);
    match res {
        Some((median, Some(median2))) => {
            println!("{:0<10},{:0<10}", median, median2);
        },
        Some((median, None)) => {
            println!("{:0<10}", median);
        },
        _ => {
            println!("No median");
        }
    }
}

/// This function uses quick select to find the kth smallest element in an array,
/// indexed from 0.
/// In this case we are using it to find the median value.
/// It is adapted from the quick sort algorithm, but instead of recursing on both
/// sides of the array, it recurses on only one side.
/// It has an average time complexity of O(n), but worst case is O(n^2).
/// # Arguments
/// * `arr` - A mutable slice of type T
/// * `k` - The index of the element to find
/// # Returns
/// * `Option<T>` - The kth smallest element in the array
/// # Examples 
/// ```
/// let mut arr = [1, 2, 3, 4, 5];
/// let k = 2;
/// let res = quick_select(&mut arr, k);
/// assert_eq!(res, Some(3));
/// ```
fn quick_select<T: PartialOrd + Clone>(arr: &mut [T], k: usize) -> Option<T> {
    /*
     * This function uses quick select to find the kth smallest element in an array.
     * In this case we are using it to find the median value.
     * It is adapted from the quick sort algorithm, but instead of recursing on both
     * sides of the array, it recurses on only one side.
     */
    let len = arr.len();
    if len == 0 || k >= len {
        return None;
    }
    let pivot_index = len / 2;
    let pivot_value = arr[pivot_index].clone();

    let mut i = 0; // left side of the array
    let mut j = 0; // left side of the array (accounts for duplicates)
    let mut n = len - 1; // right side of the array
    while j <= n {
        if arr[j] < pivot_value {
            arr.swap(i, j);
            i += 1;
            j += 1;
        } else if arr[j] > pivot_value {
            arr.swap(j, n);
            n -= 1;
        } else {
            j += 1;
        }
    }
    if i <= k && k <= n {
        // Result is simply the k-th element
        Some(arr[k].clone())
    } else if k < i {
        // Recurse on the left side of the array
        quick_select(&mut arr[..i], k)
    } else {
        // Recurse on the right side of the array
        quick_select(&mut arr[n + 1..], k - n - 1)
    }
}

/// This function finds the median value(s) of an array.
/// If the array has an even number of elements, it returns the two middle values.
/// If the array has an odd number of elements, it returns the middle value.
/// It uses quick select to find the median value(s).
/// It has an average time complexity of O(n), but worst case is O(n^2).
/// # Arguments
/// * `arr` - A slice of type T
/// # Returns
/// * `Option<(T, Option<T>)>` - A tuple of the median value(s)
/// # Examples
/// ```
/// let arr = [1, 2, 3, 4, 5];
/// let res = find_median_values(&arr);
/// assert_eq!(res, Some(3, None));
/// ```
fn find_median_values<T: PartialOrd + Clone>(arr: &[T]) -> Option<(T, Option<T>)> {
    /*
     * This function finds the median value(s) of an array.
     * If the array has an even number of elements, it returns the two middle values.
     * If the array has an odd number of elements, it returns the middle value.
     * It uses quick select to find the median value(s).
     * It has an average time complexity of O(n), but worst case is O(n^2).
     * It is adapted from the quick sort algorithm, but instead of recursing on both
     * sides of the array, it recurses on only one side.
     * It returns a tuple of the median value(s).
     */
    let mut arr_clone = arr.clone().to_vec();
    let length = arr.len();
    // handle the case where the array is empty
    if length == 0 {
        return Option::None;
    }
    if length % 2 == 0 {
        // return two values if the array has an even number of elements
        let median = quick_select(&mut arr_clone, length / 2 - 1);
        let median2 = quick_select(&mut arr_clone, length / 2);
        match (median, median2) {
            (Some(median), Some(median2)) => {
                Option::Some((median, Some(median2)))
            },
            _ => {
                Option::None
            }
        }
    } else {
        // otherwise return one value
        let median = quick_select(&mut arr_clone, length / 2);
        match median {
            Some(median) => {
                Option::Some((median, None))
            },
            _ => {
                Option::None
            }
        }
    }
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
    use indicatif::{ParallelProgressIterator, ProgressStyle};

    /// This function is used to test quick select by comparing it to the sort function
    /// on a vector of random numbers.
    /// It creates a vector of random numbers, then sorts the vector and finds the median value,
    /// then asserts that the quick select algorithm also finds the median value.
    #[test]
    fn test_quick_select() {
        /* 
         * This test checks that the quick select algorithm works to find the median
         * value for an array of numbers. It uses a random number generator to create
         * a vector of random numbers, then sorts the vector and finds the median value,
         * then asserts that the quick select algorithm also finds the median value.
         */
        // create a vector of random numbers with a max length, max value and seed.
        let max_length: usize = 100; // max possible length of the vector
        let max_value: usize = 100; // max possible value of the random numbers
        let num_tries = 100; // number of times to run the test
        for _ in 0..num_tries{
            // intialize the random number generator
            let seed = rand::thread_rng().gen_range(0..1<<32);
            let mut rng = ChaChaRng::seed_from_u64(seed);
            let num_elements = rng.gen_range(1..max_length);
            let numbers: Vec<usize> = (0..num_elements)
                .map(|_| rng.gen_range(0..max_value))
                .collect();
            // find the median of a clone of the vector
            let mut numbers_clone = numbers.clone();
            numbers_clone.sort();
            let median = if num_elements % 2 == 0 {
                // if the length of the array is even, return the two middle values
                Option::Some((numbers_clone[num_elements / 2 - 1], Some(numbers_clone[num_elements / 2])))
            } else {
                // otherwise, return the middle value
                Option::Some((numbers_clone[num_elements / 2], None))
            };
            let res = find_median_values(&numbers);
            match(&median, &res) {
                // match the median and the result of the quick select algorithm
                (Some((median, Some(median2))), Some((res, Some(res2)))) => {
                    assert_eq!(median, res);
                    assert_eq!(median2, res2);
                },
                (Some((median, None)), Some((res, None))) => {
                    assert_eq!(median, res);
                },
                _ => {
                    panic!("median and res are not the same");
                }
            }
        }
    }
    
    /// This function tests the worst case scenario for the quick select algorithm.
    /// It creates a vector of numbers from 1 to 10, then calls quick select on the
    /// vector to find the 0th, 1st, 2nd, 3rd, and 4th element.
    /// It asserts that the quick select algorithm returns the correct values.
    #[test]
    fn quick_select_worst_case() {
        /*
         *  This test quickly checks some manual inputs to make sure the quick select
         *  algorithm works in the worst case scenario.
         */
        let mut arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let res = quick_select(&mut arr, 0);
        assert_eq!(res, Some(1));
        let res = quick_select(&mut arr, 1);
        assert_eq!(res, Some(2));
        let res = quick_select(&mut arr, 2);
        assert_eq!(res, Some(3));
        let res = quick_select(&mut arr, 3);
        assert_eq!(res, Some(4));

    }

    /// This function generates a vector of random phone numbers.
    /// It takes the number of phone numbers to generate, the length of each phone number
    /// # Arguments
    /// * `num_numbers` - The number of phone numbers to generate
    /// * `phone_num_length` - The length of each phone number
    /// * `seed` - The seed for the random number generator
    /// # Returns
    /// * `phone_numbers` - A vector of phone numbers as strings
    /// # Example
    /// ```
    /// let phone_numbers = generate_phone_numbers(10, 10, 42);
    /// ```
    fn generate_phone_numbers(num_numbers: usize, phone_num_length: usize, seed: u64) -> Vec<String> {
        let mut rng = ChaChaRng::seed_from_u64(seed);
        let phone_numbers: Vec<String> = (0..num_numbers)
            .map(|_| {
                let phone_number: String = (0..phone_num_length)
                    .map(|_| rng.gen_range(0..10).to_string())
                    .collect();
                phone_number
            })
            .collect();
        phone_numbers
    }
    enum ComparisonType {
        PhoneNumbers,
        Integers
    }
    /// This test is used to test quick select by comparing it to the sort function
    /// on a vector of random phone numbers.
    /// It creates a vector of random phone numbers, then sorts the vector and finds the median value,
    /// then asserts that the quick select algorithm also finds the median value.
    #[test]
    fn test_quick_select_phone_numbers() {
        let max_length: usize = 100;
        let seed = rand::thread_rng().gen_range(0..1<<32);
        let num_tries = 100;
        // create a vector of random phone numbers with a max length, max value and seed.
        for _ in 0..num_tries {
            // initialize the random number generator
            let mut rng = ChaChaRng::seed_from_u64(seed);
            let num_elements = rng.gen_range(0..max_length);
            let phone_numbers: Vec<String> = generate_phone_numbers(num_elements, 10, seed);
            // find the median of a clone of the vector
            let mut phone_numbers_clone = phone_numbers.clone();
            // clean the phone numbers
            phone_numbers_clone = phone_numbers_clone
                .iter()
                .map(|number| number.chars().rev().take(10).collect::<String>().chars().rev().collect::<String>())
                .collect();
            phone_numbers_clone.sort();
            let median = if num_elements % 2 == 0 {
                Some((phone_numbers_clone[num_elements / 2 - 1].clone(), Some(phone_numbers_clone[num_elements / 2].clone())))
            } else {
                Some((phone_numbers_clone[num_elements / 2].clone(), None))
            };
            let res = find_median_values(&phone_numbers);
            // Check that the median and the result of the quick select algorithm are the same
            match(&median, &res) {
                (Some((median, Some(median2))), Some((res, Some(res2)))) => {
                    assert_eq!(median, res);
                    assert_eq!(median2, res2);
                },
                (Some((median, None)), Some((res, None))) => {
                    assert_eq!(median, res);
                },
                _ => {
                    panic!("median and res are not the same");
                }
            }
        }
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

    // This function is used to compare the performance of quick select and quick sort
    // on a vector of either phone numbers or integers of an array up to a certain length.
    // It takes a comparison type, the maximum length of the array and a seed for the random number generator.
    // It returns a vector of tuples containing the length of the array, the time taken to sort the array
    // # Arguments
    // * `comparison` - The type of comparison to be made
    // * `max_length` - The maximum length of the array
    // * `seed` - The seed for the random number generator
    // # Returns
    // * `Vec<(usize, Duration, Duration)>` - A vector of tuples containing the length of the array, the time taken to sort the array
    // using quick sort and the time taken to find the median value using quick select.
    // # Example
    // ```
    // let comparison = ComparisonType::PhoneNumbers;
    // let max_length = 100;
    // let seed = 42;
    // let results = _compare_quickselect_and_quicksort(comparison, max_length, seed);
    // ```
    fn _compare_quickselect_and_quicksort(comparison: ComparisonType, max_length: usize, seed: u64) -> Vec<(usize, Duration, Duration)> {
        // let max_length: usize = 100000;
        // let seed: u64 = 42;
        let style = ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}/{eta_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}")
            .unwrap()
            .progress_chars("##-");
        let results = match comparison {
            ComparisonType::PhoneNumbers => {
                // Phone numbers case
                let phone_numbers = generate_phone_numbers(max_length, 10, seed);
                let res: Vec<(usize, Duration, Duration)> = (1..=max_length)
                    .into_par_iter() // parallelize the loop
                    .progress_count(max_length as u64)
                    .with_style(style)
                    .map(|num_elements| {
                        let phone_numbers_slice = &phone_numbers[..(max_length - num_elements)]; // get a slice of the vector
                        let timings = compare_quickselect_and_quicksort(phone_numbers_slice);
                        ((max_length - num_elements + 1), timings.0, timings.1)
                    }).collect(); // collect the results
                res.into_iter().rev().collect()
            },
            ComparisonType::Integers => {
                // Integers case
                let mut rng = ChaChaRng::seed_from_u64(seed);
                let arr: Vec<i32> = (0..max_length).map(|_| rng.gen_range(0..1000000)).collect();
                let res: Vec<(usize, Duration, Duration)> = (1..=max_length)
                    .into_par_iter() // parallelize the loop
                    .progress_count(max_length as u64)
                    .with_style(style)
                    .map(|num_elements| {
                        let arr_slice = &arr[..(max_length - num_elements)]; // get a slice of the vector
                        let timings = compare_quickselect_and_quicksort(arr_slice);
                        ((max_length - num_elements + 1), timings.0, timings.1)
                    })
                    .collect();
                res.into_iter().rev().collect()
            }
        };
        results
    }

    // This function is for testing the performance of the quick sort algorithm
    // to the quick select algorithm.
    // # Arguments
    // * `arr` - A vector of T that is to be sorted
    // # Returns
    // * `(Duration, Duration)` - The average time it takes to sort the vector using quicksort and quickselect
    // # Example
    // ```
    // let mut arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // let timings = compare_quickselect_and_quicksort(&arr);
    // ```
    fn compare_quickselect_and_quicksort<T: PartialOrd + Clone>(arr: &[T]) -> (Duration, Duration) {
        let mut arr_clone = arr.to_vec();
        let mut sum_quicksort_time = Duration::new(0, 0);
        let mut sum_quickselect_time = Duration::new(0, 0);
        for _ in 0..5 { // run the test 5 times and take the average
            let start = Instant::now();
            quicksort(&mut arr_clone);
            let end = Instant::now();
            sum_quicksort_time += end - start;
            let start = Instant::now();
            find_median_values(&arr);
            let end = Instant::now();
            sum_quickselect_time += end - start;
        }
        let avg_quicksort_time = sum_quicksort_time / 5;
        let avg_quickselect_time = sum_quickselect_time / 5;
        (avg_quicksort_time, avg_quickselect_time)
    }


    /// This test is used to compare the performance of the quick select algorithm
    /// to the quick sort algorithm.
    /// It creates a vector of random phone numbers, then sorts the vector and finds the median value,
    /// then asserts that the quick select algorithm also finds the median value.
    /// It then compares the time taken to find the median value using the quick select algorithm
    /// to the time taken to sort the vector and find the median value.
    /// It then prints the results to a csv file.
    #[test]
    fn compare_quickselect_and_quicksort_phonenumbers() {
        let max_length: usize = 100000;
        let seed: u64 = 42;
        // create a csv file to store the results
        let mut file = match std::fs::File::create("results_phone.csv") {
            Ok(file) => file,
            Err(err) => panic!("couldn't create file: {}", err),
        };
        
        let results = _compare_quickselect_and_quicksort(ComparisonType::PhoneNumbers, max_length, seed);
        results.iter().for_each(|(length, quicksort_time, quickselect_time)| {
            let ratio = quicksort_time.as_nanos() as f64 / quickselect_time.as_nanos() as f64;
            let _ = writeln!(file, "{0}, {1}, {2}, {3}", length, quicksort_time.as_nanos(), quickselect_time.as_nanos(), ratio);
        });
    }

    // This test is used to compare the performance of the quick select algorithm
    // to the quick sort algorithm.
    // It creates a vector of random integers, then sorts the vector and finds the median value,
    // then asserts that the quick select algorithm also finds the median value.
    // It then compares the time taken to find the median value using the quick select algorithm
    // to the time taken to sort the vector and find the median value.
    // It then prints the results to a csv file.
    #[test]
    fn compare_quickselect_and_quicksort_integers() {
        let max_length: usize = 100000;
        let seed: u64 = 42;
        // create a csv file to store the results
        let mut file = match std::fs::File::create("results_int.csv") {
            Ok(file) => file,
            Err(err) => panic!("couldn't create file: {}", err),
        };
        let results = _compare_quickselect_and_quicksort(ComparisonType::Integers, max_length, seed);
        results.iter().for_each(|(length, quicksort_time, quickselect_time)| {
            let ratio = quicksort_time.as_nanos() as f64 / quickselect_time.as_nanos() as f64;
            let _ = writeln!(file, "{0}, {1}, {2}, {3}", length, quicksort_time.as_nanos(), quickselect_time.as_nanos(), ratio);
        });
    }
}