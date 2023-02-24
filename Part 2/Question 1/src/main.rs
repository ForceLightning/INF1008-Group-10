use std::env;

fn main() {
    let args: Vec<String>  = env::args().collect();
    let mut phone_numbers: Vec<String> = args[1..]
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
    // quick sort
    quick_sort(&mut phone_numbers);
    // find the median of the sorted array
    // if the array has even number of elements, the median is the two middle elements.
    // if the array has odd number of elements, the median is the middle element.
    let len = phone_numbers.len();
    let median = if len % 2 == 0 {
        let first = &phone_numbers[len/2 - 1];
        let second = &phone_numbers[len/2];
        format!("{},{}", first, second)
    } else {
        phone_numbers[len/2].clone().to_string()
    };
    println!("{}", median);
    
}

fn quick_sort<T: PartialOrd>(arr: &mut [T]) {
    /* 
     * This function sorts the array using quick sort
     * It uses the partition function to partition the array
     * and then recursively calls itself on the two parts
     * of the array
     */
    if arr.len() <= 1 {
        return;
    }
    let pivot = partition(arr); // partition the array
    quick_sort(&mut arr[..pivot]);
    quick_sort(&mut arr[pivot+1..]);
}

fn partition<T: PartialOrd>(arr: &mut [T]) -> usize {
    /*
     * This function partitions the array into two parts:
     * 1. Elements less than the pivot
     * 2. Elements greater than the pivot
     */
    let len = arr.len();
    let pivot_index = len / 2; // choose a pivot
    arr.swap(pivot_index, len-1); // move the pivot to the end
    let mut store_index = 0; // store_index is the index where the pivot should be placed
    for i in 0..len - 1{
        if arr[i] < arr[len-1] { // if the element is less than the pivot
            arr.swap(i, store_index); // swap it with the element at store_index
            store_index += 1; // increment store_index
        }
    }
    arr.swap(store_index, len-1);
    store_index
}