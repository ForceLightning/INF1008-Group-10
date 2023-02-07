#Function for selection sort (Stable)
def selectionSort(array, size, stable=True):
    for ind in range(size):
        min_index = ind

        if stable:
            for j in range(ind + 1, size):
                # select the minimum element in every iteration
                if array[j][1] < array[min_index][1]:
                    min_index = j
        else:
            for j in range(size - 1, ind, -1):
                if array[j][1] < array[min_index][1]:
                    min_index = j
        # swapping the elements to sort the array
        (array[ind], array[min_index]) = (array[min_index], array[ind])
        print(array)



# Function to do insertion sort (Stable)
def insertionSort(arr, stable=True):
    # Traverse through 1 to len(arr)
    for i in range(1, len(arr)):

        key = arr[i]
        # Move elements of arr[0..i-1], that are
        # greater than key, to one position ahead
        # of their current position
        j = i-1
        if stable:
            while j >= 0 and key[1] < arr[j][1]:
                arr[j + 1] = arr[j]
                j -= 1
            arr[j + 1] = key
        else:
            while j >= 0 and key[1] <= arr[j][1]:
                arr[j + 1] = arr[j]
                j -= 1
            arr[j + 1] = key
        print(arr)
    return arr




# Python program for implementation of Quicksort Sort (Stable)
# Function to find the partition position
def partition(array, low, high, stable=True):

    # choose the rightmost element as pivot
    if stable:
        pivot = array[high][1]
    else:
        pivot = array[low][1]

    # pointer for greater element
    i = low - 1

    # traverse through all elements
    # compare each element with pivot
    for j in range(low, high):
        if (stable and array[j][1] <= pivot) or (not stable and array[j][1] < pivot):
            # If element smaller than pivot is found
            # swap it with the greater element pointed by i
            i += 1
            # Swapping element at i with element at j
            (array[i], array[j]) = (array[j], array[i])
            print(array)
    # Swap the pivot element with the greater element specified by i
    (array[i + 1], array[high]) = (array[high], array[i + 1])
    # Return the position from where partition is done
    return i + 1


# function to perform quicksort
def quickSort(array, low, high, stable=True):
    if low < high:

        # Find pivot element such that
        # element smaller than pivot are on the left
        # element greater than pivot are on the right
        pi = partition(array, low, high, stable)

        # Recursive call on the left of pivot
        quickSort(array, low, pi - 1)

        # Recursive call on the right of pivot
        quickSort(array, pi + 1, high)


data_selection = [("", 4), ("", 5), ("first", 3), ("second", 3)]
#print("Unsorted Array: ", data_selection)
print("=====Stable Selection Sort=====")
print(data_selection)
size = len(data_selection)
selectionSort(data_selection, size)

data_selection = [("", 4), ("", 5), ("first", 3), ("second", 3)]
print("=====Unstable Selection Sort=====")
size = len(data_selection)
print(data_selection)
selectionSort(data_selection, size, False)

print("=====Stable Insertion Sort=====")
data_insertion = [("", 4), ("", 5), ("first", 3), ("second", 3)]
print(data_insertion)
print(insertionSort(data_insertion))

print("=====Unstable Insertion Sort=====")
data_insertion = [("", 4), ("", 5), ("first", 3), ("second", 3)]
print(data_insertion)
print(insertionSort(data_insertion, False))

print("=====Stable Quick Sort=====")
data_quick = [("first", 3), ("", 5), ("", 4), ("second", 3)]
print(data_quick)
quickSort(data_quick, 0, len(data_quick) - 1)
print(data_quick)

print("=====Unstable Quick Sort=====")
data_quick = [("first", 3), ("", 5), ("", 4), ("second", 3)]
print(data_quick)
quickSort(data_quick, 0, len(data_quick) - 1, False)
print(data_quick)
