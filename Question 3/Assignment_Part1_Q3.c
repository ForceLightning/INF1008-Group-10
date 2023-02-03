#include <stdio.h>

int arr[100];
int start = 50;
int end = 51;

int insertItem(int input, int position) {   // position: 0 for inserting at the front; 1 for inserting at the end
    if (start == 0 || end == 100) {
        printf("Overflow!");
        return 0;
    }
    if (position == 0) {
        arr[start] = input;
        start--;
    }
    else if (position == 1) {
        arr[end] = input;
        end++;
    }
    else {
        printf("Invalid Input!");
    }
}

int removeItem(int position) {              // position: 0 for removing at the front; 1 for removing at the end
    if (end - start == 1) {
        printf("List is empty!");
        return 0;
    }
    if (position == 0) {
        start++;
    }
    else if (position == 1) {
        end--;
    }
    else {
        printf("Invalid Input!");
    }
}

int get(int i) {                            // get(i) returns the item in the list at index i
    return arr[start + 1 + i];
}

int main() {
    for (int i = 0; i < 10; i++) {
        insertItem(i + 10, 0);
        insertItem(i * 2, 1);
    }
    for (int i = 0; i < end - start - 1; i++) {
        printf("%d,", arr[start + 1 + i]);
    }
    printf("\n");

    for (int i = 0; i < 3; i++) {
        removeItem(0);
        removeItem(1);
    }
    for (int i = 0; i < end - start - 1; i++) {
        printf("%d,", arr[start + 1 + i]);
    }
    printf("\n");

    printf("%d", get(5));
    printf("\n");

    for (int i = 0; i < 50; i++) {
        insertItem(40, 1);
    }
    for (int i = 0; i < 60; i++) {
        removeItem(1);
    }
    for (int i = 0; i < end - start - 1; i++) {
        printf("%d,", arr[start + 1 + i]);
    }
    printf("\n");
}