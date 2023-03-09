import sys


# function to calculate the absolute difference between two numbers
def absolute_difference(x, y):
    return abs(int(x) - int(y))


# function to find the K unique numbers nearest to a given 10-digit number
def find_nearest_numbers(file_name, target_number, k):
    numbers = {} # create a hash table to store the 10-digit numbers from the input file

    # read the numbers from the input file and store them in the hash table
    with open(file_name, 'r') as file:
        for line in file:
            number = line.strip()
            #numbers[number] = numbers[number] + 1 if numbers[number] is not None else 1
            if number not in numbers.keys():
                numbers[number] = 1
            else:
                numbers[number] += 1

    # calculate the absolute differences between each number in the hash table and the target number
    differences = {} #max size of 2
    print(numbers)
    for number in numbers:
        difference = absolute_difference(number, target_number)
        print(difference)
        if difference not in differences:
            differences[difference] = [number]
        else:
            differences[difference].append(number)

    # sort the unique differences in ascending order
    sorted_differences = sorted(differences.keys())
    print(differences)
    # print the K unique numbers nearest to the target number
    count = 0
    for difference in sorted_differences: # iterate through the unique differences calculated
        for number in differences[difference]: # iterate through the numbers that were calculated to be that difference
            for i in range(numbers[number]): # repeat the print method the amount of time the number appeared
                print(number)
            count += 1
            print(count)
        if count >= k:
            return


# main function to handle command-line inputs
if __name__ == '__main__':
    # check if the correct number of arguments is provided
    if len(sys.argv) < 3 or len(sys.argv) > 4:
        print('Usage: python findnumbers.py input_file target_number [k]')
        sys.exit()

    # get the input file name, target number, and k (if provided)
    file_name = sys.argv[1]
    target_number = sys.argv[2]
    k = 1 # set default value of k to 1
    if len(sys.argv) == 4:
        k = int(sys.argv[3])

    # find the K unique numbers nearest to the target number
    find_nearest_numbers(file_name, target_number, k)
