#include <stdio.h>

int arr[100];
int start = 50;
int end = 51;

int insertItem(int input, int position) {   // position: 0 for inserting at the front; 1 for inserting at the end
    if (position == 0) {
        if (start == 0) {
            printf("Overflow!\n");
        }
        else {
            arr[start] = input;
            start--;
        }
    }
    else if (position == 1) {
        if (end == 99) {
            printf("Overflow!\n");
        }
        else {
            arr[end] = input;
            end++;
        }
    }
    else {
        printf("Invalid Input!\n");
    }
}

int removeItem(int position) {              // position: 0 for removing at the front; 1 for removing at the end
    if (end - start == 1) {
        printf("List is empty!\n");
        return 0;
    }
    if (position == 0) {
        start++;
    }
    else if (position == 1) {
        end--;
    }
    else {
        printf("Invalid Input!\n");
    }
}

int get(int i) {                            // get(i) returns the item in the list at index i
    if (i >= 0 && i < end - start - 1) {
        return arr[start + 1 + i];
        
    }
    else {
        printf("Index out of range!\n");
        return NULL;
    }
}

int printList() {
    printf("[");
    for (int i = 0; i < end - start - 2; i++) {
        printf("%d,", arr[start + 1 + i]);
    }
    if (end - start == 1) {
        printf("]\n");
    }
    else {
        printf("%d]\n", arr[end - 1]);
    }
}

int main() {
    // initial list
    printf("Initial list: ");
    printList();

    // inserting 10 items to the start of the list
    for (int i = 0; i < 10; i++) {
        insertItem(i + 10, 0);
    }
    printf("After adding 10 items to the start of the list: ");
    printList();

    // inserting 10 items to the end of the list
    for (int i = 0; i < 10; i++) {
        insertItem(i * 2, 1);
    }
    printf("After adding 10 items to the end of the list: ");
    printList();

    // inserting the item '13' at the start of the list
    insertItem(13, 0);
    printf("After adding the item '13' to the start of the list: ");
    printList();

    // inserting the item '14' at the end of the list
    insertItem(14, 1);
    printf("After adding the item '14' to the start of the list: ");
    printList();

    // inserting the item '15' at an invalid position
    printf("Adding the item '15' to an invalid position:\n");
    insertItem(15, 2);
    printf("Current list: ");
    printList();

    // removing the item at the start of the list
    removeItem(0);
    printf("After removing item at the start of the list: ");
    printList();

    // removing the item at the end of the list
    removeItem(1);
    printf("After removing item at the end of the list: ");
    printList();

    // removing the item at an invalid position
    printf("Removing item at an invalid position:\n");
    removeItem(2);
    printf("Current list: ");
    printList();

    // get the item at index 5 of the list
    printf("Item at index 5: %d\n", get(5));

    // get the item at index 20 of the list
    printf("Getting an item with an index that is out of range (index 20):\n");
    printf("Item at index 20: %d\n", get(20));

    // attempting to overflow at the start of the list
    printf("Attempting to overflow at the start of the list:\n");
    for (int i = 0; i < 42; i++) {
        insertItem(1, 0);
    }
    printf("Current list: ");
    printList();

    // attempting to overflow at the end of the list
    printf("Attempting to overflow at the end of the list:\n");
    for (int i = 0; i < 40; i++) {
        insertItem(0, 1);
    }
    printf("Current list: ");
    printList();

    // removing all the items in the list
    for (int i = 0; i < 98; i++) {
        removeItem(1);
    }
    printf("After removing every items in the list: ");
    printList();

    // attempting to remove an item from an empty list
    printf("Removing an item from an empty list:\n");
    removeItem(1);
    printf("Current list: ");
    printList();
}