# INF1008 Assignment 2 Group 2 Submission
## Folder Directory
```
.
├── Task 1                      # Task 1 Root Directory
│   ├── src                     # Source files
│   ├── target                  # Compiled files
|   |   ├── doc                 # Documentation
|   |   └── release             # Compiled executible
│   └── Cargo.toml              # Cargo manifest
├── Task 2                      # Task 2 Root Directory
│   ├── python implementation   # Python implementation directory
│   |   └── task2.py            
│   └── rust implementation
│       ├── src                 # Source files
│       ├── target              # Compiled files
|       |   ├── doc             # Documentation
|       |   └── release         # Compiled executible
│       └── Cargo.toml          # Cargo manifest
├── *.pdf                       # Report PDF
└── README.md
```

# Task 1
## Compiling from source
Run `cargo build --release` from the root folder of the task.

## Usage
```ps
PS> qn_1.exe 123-456-7890 "(323) 456-7890" "+1 223-456-7890" 1-322-345-7890 "322 555 0000"
> 3223457890
```

Note that all phone numbers with spaces in them must be delimited by quotation marks `"<phone number>"`

## Running tests
Run `cargo test <test name> --nocapture` for individual test cases (Highly recommended as the final test case will take a long while to complete as it is doing comparisons with a quicksort implementation).
### List of tests:
- `test_quick_select`: Tests to check that the quickselect implementation produces the same result as an implementation that sorts the same input of random integers.
- `test_quick_select_phone_numbers`: Tests to check that the quickselect implementation produces the same result as an implementation that sorts the same input of random phone numbers.
- `compare_quickselect_and_quicksort_integers`: Tests the time taken to find the median value for an input array length of 1 to 100,000 with integers as elements. This is output to a file named "results_int.csv".
- `compare_quickselect_and_quicksort_phonenumbers`: Tests the time taken to find the median value for an input array length of 1 to 100,000 with strings as elements. This is output to a file named "results_phone.csv".

# Task 2
## Compiling from source (Rust Implementation)
Run `cargo build --release` from the "rust implementation" folder.

## Usage
### Rust implementation
```ps
PS> Question_2_rust.exe phonescraped 7327325555 2
> 7327325555
> 7327325555
> 7327325555
> 7327325555
> 7327325555
> 7327325554
```
### Python implementation
```ps
PS> python task2.py phonescraped 7327325555 2
> 7327325555
> 7327325555
> 7327325555
> 7327325555
> 7327325555
> 7327325554
```

## Running tests (Rust Implementation)
Run `cargo test <test name> --nocapture` for individual test cases (Highly recommended as the final test case will take a long while to complete as it is doing comparisons with a quicksort implementation).
### List of tests:
- `test_sanitizer_valid_numbers`: Tests if the phone number sanitizer produces the correctly parsed string as an integer.
- `test_sanitizer_invalid_numbers`: Tests if the phone number sanitizer incorrectly sanitizes malformed phone number strings as integers.
- `test_kth_nearest_pre_defined`: Tests if our implementation finds the correct k-th nearest values for a predefined target number and k.
- `test_kth_nearest_random`: Tests if our implementation finds the correct k-th nearest phone number for a randomly generated vector of phone numbers. This is tested against an implementation that uses quicksort to find the k-th nearest numbers.
- `compare_naive_and_our_implementation`: Compares the naive sorting implementation with our BTreeMap implementation. The results are output to a file named "results.csv".