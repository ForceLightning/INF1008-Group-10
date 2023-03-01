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
    let phone_numbers: Vec<String> = args[1..]
        .iter()
        .map(|number| {
            let cleaned: String = number
                .chars()
                .filter(|&c| c.is_digit(10)) // filter out non-digits
                .collect();
            cleaned
        })
        .filter(|cleaned| cleaned.len() >= 10) // filter out numbers with less than 10 digits
        .map(|cleaned| cleaned.chars().rev().take(10).collect::<String>().chars().rev().collect()) // take last 10 digits
        .collect();
    // use quick select to find the median phone number
    // quick select has an average time complexity of O(n), but worst case is O(n^2)
    let res = find_median_values(&phone_numbers);
    match res {
        Some((median, Some(median2))) => {
            println!("{},{}", median, median2);
        },
        Some((median, None)) => {
            println!("{}", median);
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

    let mut i = 0;
    let mut j = 0;
    let mut n = len - 1;
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
        Some(arr[k].clone())
    } else if k < i {
        quick_select(&mut arr[..i], k)
    } else {
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
        let seed: u64 = 42;
        let mut rng = ChaChaRng::seed_from_u64(seed);
        let num_elements = rng.gen_range(0..max_length);
        // let num_elements = 10;
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
        dbg!(&numbers_clone[num_elements / 2 - 1..=num_elements / 2+1]);
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

    /// This test is used to test quick select by comparing it to the sort function
    /// on a vector of random phone numbers.
    /// It creates a vector of random phone numbers, then sorts the vector and finds the median value,
    /// then asserts that the quick select algorithm also finds the median value.
    #[test]
    fn test_quick_select_phone_numbers() {
        let max_length: usize = 100;
        let seed: u64 = 42;
        // create a vector of random phone numbers with a max length, max value and seed.
        let mut rng = ChaChaRng::seed_from_u64(seed);
        let num_elements = rng.gen_range(0..max_length);
        // let num_elements = 5;
        let phone_numbers: Vec<String> = generate_phone_numbers(num_elements, 10, seed);
        // find the median of a clone of the vector
        let mut phone_numbers_clone = phone_numbers.clone();
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
        dbg!(&phone_numbers_clone[num_elements / 2 - 1..=num_elements / 2+1]);
        let res = find_median_values(&phone_numbers);
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
        let pivot_index = len / 2;
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

    /// This test is used to compare the performance of the quick select algorithm
    /// to the quick sort algorithm.
    /// It creates a vector of random phone numbers, then sorts the vector and finds the median value,
    /// then asserts that the quick select algorithm also finds the median value.
    /// It then compares the time taken to find the median value using the quick select algorithm
    /// to the time taken to sort the vector and find the median value.
    /// It then prints the results to a csv file.
    #[test]
    fn compare_quickselect_and_quicksort() {
        let max_length: usize = 10000;
        let seed: u64 = 42;
        // create a csv file to store the results
        let mut file = match std::fs::File::create("results.csv") {
            Ok(file) => file,
            Err(err) => panic!("couldn't create file: {}", err),
        };
        
        println!("{0: <10} | {1: <10} | {2: <10} | {3: <10}", "length", "quicksort", "quickselect", "ratio");
        let phone_numbers: Vec<String> = generate_phone_numbers(max_length, 10, seed);
        for num_elements in 1..=max_length {
            let mut phone_numbers_clone = phone_numbers[..num_elements].to_vec();
            let mut sum_quicksort_time: Duration = Duration::new(0, 0);
            let mut sum_quickselect_time: Duration = Duration::new(0, 0);
            for i in 0..5 {
                let start = Instant::now();
                let _ = quicksort(&mut phone_numbers_clone);
                let end = Instant::now();
                if i == 0 {
                    sum_quicksort_time = end - start;
                } else {
                    sum_quicksort_time += end - start;
                }
                let start = Instant::now();
                let _ = find_median_values(&phone_numbers[..num_elements]);
                let end = Instant::now();
                if i == 0 {
                    sum_quickselect_time = end - start;
                } else {
                    sum_quickselect_time += end - start;
                }
            }
            let avg_quicksort_time = sum_quicksort_time / 5;
            let avg_quickselect_time = sum_quickselect_time / 5;
            let ratio = avg_quicksort_time.as_nanos() as f64 / avg_quickselect_time.as_nanos() as f64;
            // println!("num_elements: {}, quicksort: {:?}, quickselect: {:?}", num_elements, avg_quicksort_time, avg_quickselect_time);
            println!("{0: <10} | {1: <10} | {2: <10} | {3: <10} ", num_elements, avg_quicksort_time.as_nanos(), avg_quickselect_time.as_nanos(), ratio);
            writeln!(file, "{}, {}, {}, {}", num_elements, avg_quicksort_time.as_nanos(), avg_quickselect_time.as_nanos(), ratio).expect("couldn't write to file");
        }

    }
}