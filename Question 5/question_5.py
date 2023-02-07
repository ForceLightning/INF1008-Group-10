#Function for selection sort (Stable)
def selectionSort(array, size):

    for ind in range(size):
        min_index = ind

        for j in range(ind + 1, size):
            # select the minimum element in every iteration
            if array[j][1] < array[min_index][1]:
                min_index = j
        # swapping the elements to sort the array
        (array[ind], array[min_index]) = (array[min_index], array[ind])
        print(array)



# Function to do insertion sort (Stable)
def insertionSort(arr):
    sorted_arr = []
    # Traverse through 1 to len(arr)
    for i in range(1, len(arr)):

        key = arr[i]
        # Move elements of arr[0..i-1], that are
        # greater than key, to one position ahead
        # of their current position
        j = i-1
        while j >= 0 and key[1] < arr[j][1]:
            arr[j + 1] = arr[j]
            j -= 1
        arr[j + 1] = key
        print(arr)
    return arr




# Python program for implementation of Quicksort Sort (Stable)
# Function to find the partition position
def partition(array, low, high):

    # choose the rightmost element as pivot
    pivot = array[high][1]

    # pointer for greater element
    i = low - 1

    # traverse through all elements
    # compare each element with pivot
    for j in range(low, high):
        if array[j][1] <= pivot:

            # If element smaller than pivot is found
            # swap it with the greater element pointed by i
            i = i + 1

            # Swapping element at i with element at j
            (array[i], array[j]) = (array[j], array[i])
            print(array)
    # Swap the pivot element with the greater element specified by i
    (array[i + 1], array[high]) = (array[high], array[i + 1])
    # Return the position from where partition is done
    return i + 1


# function to perform quicksort


def quickSort(array, low, high):
    if low < high:

        # Find pivot element such that
        # element smaller than pivot are on the left
        # element greater than pivot are on the right
        pi = partition(array, low, high)

        # Recursive call on the left of pivot
        quickSort(array, low, pi - 1)

        # Recursive call on the right of pivot
        quickSort(array, pi + 1, high)


data_selection = [("d", 4), ("c", 5), ("b", 3), ("a", 3)]
#print("Unsorted Array: ", data_selection)
print("=====Selection Sort=====")
size = len(data_selection)
selectionSort(data_selection, size)

print("=====Insertion Sort=====")
data_insertion = [("d", 4), ("c", 5), ("b", 3), ("a", 3)]
print(insertionSort(data_insertion))

print("=====Quick Sort=====")
data_quick = [("d", 4), ("c", 5), ("b", 3), ("a", 3)]
quickSort(data_quick, 0, len(data_quick) - 1)
print(data_quick)
