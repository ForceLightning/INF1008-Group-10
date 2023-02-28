use std::env;

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

fn quick_select<T: PartialOrd + Clone>(arr: &mut [T], k: usize) -> Option<T> {
    /*
     * This function uses quick select to find the kth smallest element in an array.
     * In this case we are using it to find the median value.
     * It is adapted from the quick sort algorithm, but instead of recursing on both
     * sides of the array, it recurses on only one side.
     */
    if arr.is_empty() {
        return None;
    }

    let pivot_index = arr.len() / 2;
    let pivot_value = arr[pivot_index].clone();

    let mut lesser = Vec::new();
    let mut greater = Vec::new();
    let mut equal = Vec::new();

    for elem in arr.iter() {
        if *elem < pivot_value {
            lesser.push(elem.clone());
        } else if *elem > pivot_value {
            greater.push(elem.clone());
        } else {
            equal.push(elem.clone());
        }
    }
    let len_lesser = lesser.len();
    let len_equal = equal.len();

    if k < len_lesser {
        quick_select(&mut lesser, k)
    } else if k < len_lesser + len_equal {
        Some(pivot_value)
    } else {
        quick_select(&mut greater, k - len_lesser - len_equal)
    }
}

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

    #[test]
    fn test_quick_select() {
        // create a vector of random numbers with a max length, max value and seed.
        let max_length: usize = 100;
        let max_value: usize = 100;
        let seed: u64 = 42;
        let mut rng = ChaChaRng::seed_from_u64(seed);
        let num_elements = rng.gen_range(0..max_length);
        let numbers: Vec<usize> = (0..num_elements)
            .map(|_| rng.gen_range(0..max_value))
            .collect();
        // find the median of a clone of the vector
        let mut numbers_clone = numbers.clone();
        numbers_clone.sort();
        let median = if num_elements % 2 == 0 {
            Option::Some((numbers_clone[num_elements / 2 - 1], Some(numbers_clone[num_elements / 2])))
        } else {
            Option::Some((numbers_clone[num_elements / 2], None))
        };
        let res = find_median_values(&numbers);
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

    #[test]
    fn test_quick_select_phone_numbers() {
        let max_length: usize = 100;
        let seed: u64 = 42;
        // create a vector of random phone numbers with a max length, max value and seed.
        let mut rng = ChaChaRng::seed_from_u64(seed);
        let num_elements = rng.gen_range(0..max_length);
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
}